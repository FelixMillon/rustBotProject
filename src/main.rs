use axum::{
    Router,
    routing::get,
    routing::post,
    response::Json,
    extract::Json as AxumJson,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use game::Game;
use events::EventType;
use serde::{Deserialize, Serialize};

mod game;
mod gatherers;
mod scouts;
mod resources;
mod id_generator;
mod events;

#[derive(Deserialize)]
struct ResetRequest {
    columns: u32,
    rows: u32,
    gatherers: u8,
    scouts: u8,
    seed: u64,
    empty_display: Option<char>,
    obstacle_display: Option<char>,
    base_display: Option<char>,
}

#[derive(Serialize)]
struct StateResponse {
    map: Vec<Vec<char>>,
    crystal_count: u16,
    energy_count: u16,
}

fn create_new_game(rows: u32, columns: u32, seed: u64, gatherers: u8, scouts: u8, empty_display: char, obstacle_display: char, base_display: char) -> Game {
    let mut id_generator = id_generator::IDGenerator::new();
    let mut map = Game::new(rows, columns, seed, empty_display, obstacle_display, base_display);
    map.generate_map_obstacles();
    map.generate_resources(&mut id_generator, 10);
    
    for _ in 0..scouts {
        map.add_scout(rows / 2, columns / 2, &mut id_generator);
    }

    for _ in 0..gatherers {
        map.add_gatherer(rows / 2, columns / 2, &mut id_generator);
    }

    map
}

#[tokio::main]
async fn main() {
    let map = Arc::new(Mutex::new(create_new_game(40, 80, 40, 3, 7, ' ', '8', '#')));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/state", get({
            let map = Arc::clone(&map);
            move || {
                let map = Arc::clone(&map);
                async move {
                    let mut map_lock = map.lock().unwrap();
                    map_lock.handle_event(EventType::Tick);

                    let response = StateResponse {
                        map: map_lock.generate_display().iter().map(|row| {
                            row.iter().map(|cell| cell.display).collect::<Vec<_>>()
                        }).collect(),
                        crystal_count: map_lock.base.crystal,
                        energy_count: map_lock.base.energy,
                    };

                    Json(response)
                }
            }
        }))
        .route("/reset", post({
            let map = Arc::clone(&map);
            move |AxumJson(body): AxumJson<ResetRequest>| {
                let map = Arc::clone(&map);
                async move {
                    let new_game = create_new_game(
                        body.rows.clamp(15, 200),
                        body.columns.clamp(15, 200),
                        body.seed,
                        body.gatherers.clamp(0, 5),
                        body.scouts.clamp(1, 15),
                        body.empty_display.unwrap_or(' '),
                        body.obstacle_display.unwrap_or('8'),
                        body.base_display.unwrap_or('#'),
                    );
        
                    *map.lock().unwrap() = new_game;
                    Json("Game has been reset.")
                }
            }
        }))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

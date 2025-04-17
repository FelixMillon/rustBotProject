use axum::{
    Router,
    routing::get,
    response::Json,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use game::Game;
use events::EventType;
use serde::Serialize;

mod game;
mod gatherers;
mod scouts;
mod resources;
mod id_generator;
mod events;

#[derive(Serialize)]
struct StateResponse {
    map: Vec<Vec<char>>,
    crystal_count: u16,
    energy_count: u16,
}

fn create_new_game() -> Game {
    let mut id_generator = id_generator::IDGenerator::new();
    let mut map = Game::new(20, 40, 44);
    map.generate_map_obstacles();
    map.generate_resources(&mut id_generator, 10);
    for _ in 0..7 {
        map.add_scout(10, 20, &mut id_generator);
    }
    for _ in 0..3 {
        map.add_gatherer(10, 20, &mut id_generator);
    }
    map
}

#[tokio::main]
async fn main() {
    let map = Arc::new(Mutex::new(create_new_game()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Route GET /state
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
        // Route GET /reset
        .route("/reset", get({
            let map = Arc::clone(&map);
            move || {
                let map = Arc::clone(&map);
                async move {
                    let new_game = create_new_game();
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

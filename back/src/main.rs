use axum::{
    Router,
    routing::get,
    routing::post,
    response::Json,
    extract::Json as AxumJson,
    extract::Path,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use game::Game;
use events::EventType;
use serde::{Deserialize, Serialize};

mod game;
mod gatherers;
mod scouts;
mod resources;
mod id_generator;
mod events;


type SharedGames = Arc<Mutex<HashMap<String, Game>>>;

#[derive(Deserialize)]
struct ResetRequest {
    columns: u32,
    rows: u32,
    gatherers: u8,
    scouts: u8,
    resources: u8,
    seed: u64,
    empty_display: Option<char>,
    obstacle_display: Option<char>,
    base_display: Option<char>,
    scout_display: Option<char>,
    gatherer_display: Option<char>,
}

#[derive(Serialize)]
struct StateResponse {
    map: Vec<Vec<char>>,
    crystal_count: u16,
    energy_count: u16,
}

fn create_new_game(
    rows: u32,
    columns: u32,
    seed: u64,
    gatherers: u8,
    scouts: u8,
    resources: u8,
    empty_display: char,
    obstacle_display: char,
    base_display: char,
    scout_display: char,
    gatherer_display: char
) -> Game {
    let mut id_generator = id_generator::IDGenerator::new();
    let mut map = Game::new(rows, columns, seed, empty_display, obstacle_display, base_display,scout_display,gatherer_display);
    map.generate_map_obstacles();
    map.generate_resources(&mut id_generator, resources);
    
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
    let games: Arc<Mutex<HashMap<String, Game>>> = Arc::new(Mutex::new(HashMap::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/state/:id", get({
            let games = Arc::clone(&games);
            move |Path(id): Path<String>| {
                let games = Arc::clone(&games);
                async move {
                    let mut map = games.lock().unwrap();
                    if let Some(game) = map.get_mut(&id) {
                        game.handle_event(EventType::Tick);
                        let response = StateResponse {
                            map: game.generate_display().iter().map(|row| {
                                row.iter().map(|cell| cell.display).collect::<Vec<_>>()
                            }).collect(),
                            crystal_count: game.base.crystal,
                            energy_count: game.base.energy,
                        };
                        Json(response)
                    } else {
                        Json(StateResponse {
                            map: vec![],
                            crystal_count: 0,
                            energy_count: 0,
                        })
                    }
                }
            }
        }))
        .route("/start", post({
            let games = Arc::clone(&games);
            move |AxumJson(body): AxumJson<ResetRequest>| {
                let games = Arc::clone(&games);
                async move {
                    let game_id = Uuid::new_v4().to_string();
                    let new_game = create_new_game(
                        body.rows.clamp(15, 200),
                        body.columns.clamp(15, 200),
                        body.seed,
                        body.gatherers.clamp(0, 15),
                        body.scouts.clamp(1, 15),
                        body.resources.clamp(1, 50),
                        body.empty_display.unwrap_or(' '),
                        body.obstacle_display.unwrap_or('8'),
                        body.base_display.unwrap_or('#'),
                        body.scout_display.unwrap_or('S'),
                        body.gatherer_display.unwrap_or('G'),
                    );
                    {
                        let mut map_guard = games.lock().unwrap();
                        map_guard.insert(game_id.clone(), new_game);
                    }
                    let games_clone = Arc::clone(&games);
                    let id_clone = game_id.clone();
                    tokio::spawn(async move {
                        loop {
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            let mut map = games_clone.lock().unwrap();
                            if let Some(game) = map.get_mut(&id_clone) {
                                game.handle_event(EventType::Tick);
                            } else {
                                break;
                            }
                        }
                    });
                    Json(game_id)
                }
            }
        }))
        .route("/reset/:id", post({
            let games = Arc::clone(&games);
            move |Path(id): Path<String>, AxumJson(body): AxumJson<ResetRequest>| {
                let games = Arc::clone(&games);
                async move {
                    let new_game = create_new_game(
                        body.rows.clamp(15, 200),
                        body.columns.clamp(15, 200),
                        body.seed,
                        body.gatherers.clamp(0, 15),
                        body.scouts.clamp(1, 15),
                        body.resources.clamp(1, 50),
                        body.empty_display.unwrap_or(' '),
                        body.obstacle_display.unwrap_or('8'),
                        body.base_display.unwrap_or('#'),
                        body.scout_display.unwrap_or('S'),
                        body.gatherer_display.unwrap_or('G'),
                    );
                    let mut map = games.lock().unwrap();
                    if map.contains_key(&id) {
                        map.insert(id.clone(), new_game);
                        Json("Game has been reset.")
                    } else {
                        Json("Invalid game ID.")
                    }
                }
            }
        }))
        .route("/stop/:id", post({
            let games = Arc::clone(&games);
            move |Path(id): Path<String>| {
                let games = Arc::clone(&games);
                async move {
                    let mut map = games.lock().unwrap();
                    if map.remove(&id).is_some() {
                        Json("Game stopped and state cleared.")
                    } else {
                        Json("Invalid game ID.")
                    }
                }
            }
        }))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

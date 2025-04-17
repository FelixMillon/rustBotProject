use axum::{Router, routing::get};
use axum::response::Json;
use axum::Server;
use serde::Serialize;
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use tokio::sync::{Mutex, mpsc};
use game::{Game, Cell};
use events::EventType;
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

#[tokio::main]
async fn main() {
    let mut id_generator = id_generator::IDGenerator::new();
    let mut map = Game::new(20, 40, 44);
    map.generate_map_obstacles();
    map.generate_resources(&mut id_generator, 10);
    map.add_scout(10, 20, &mut id_generator);
    map.add_scout(10, 20, &mut id_generator);
    map.add_scout(10, 20, &mut id_generator);
    map.add_scout(10, 20, &mut id_generator);
    map.add_scout(10, 20, &mut id_generator);
    map.add_scout(10, 20, &mut id_generator);
    map.add_scout(10, 20, &mut id_generator);
    map.add_gatherer(10, 20, &mut id_generator);
    map.add_gatherer(10, 20, &mut id_generator);
    map.add_gatherer(10, 20, &mut id_generator);

    let map = Arc::new(Mutex::new(map));

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                EventType::Tick => {
                    println!("Tick event received");
                },
                _ => {}
            }
        }
    });

    let app = Router::new().route("/state", get(move || {
        let map = Arc::clone(&map);

        async move {
            let mut map_lock = map.lock().await;
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
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

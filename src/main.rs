use std::error::Error;

use serde::{Serialize, Deserialize};
use warp::{Rejection, Filter};

mod map;

use map::{Config, MapOptions};

pub async fn make_map(config: Config) -> Result<warp::reply::Json, Rejection> {
    let map = map::generate_chunk::<250, 250>(&config);
    Ok(warp::reply::json(&map))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string(&Config::default()).unwrap());
    println!("Starting...");

    let route = warp::get()
        .and(warp::body::content_length_limit(2048))
        .and(warp::body::json())
        .and_then(make_map);
    
    println!("🚀🚀🚀");
    warp::serve(route).run(([0,0,0,0], 8080)).await;
    println!("Stopped...");
    Ok(())
}
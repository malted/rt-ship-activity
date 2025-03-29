#![feature(async_closure)]

#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::sync::broadcast;
use rocket_ws as ws;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Heartbeat {
    username: String,
    avatar_url: String,
    editor: Option<String>,
    language: Option<String>,
    operating_system: Option<String>,
    ip_address: String,
    user_seconds_today: u64,
    global_seconds_today: u64,
}

struct State {
    tx: broadcast::Sender<Heartbeat>,
}

#[get("/echo")]
fn echo_stream(ws: ws::WebSocket, state: &rocket::State<State>) -> ws::Stream!['static] {
    let mut r = state.tx.subscribe();

    ws::Stream! { ws =>
        let _ = ws;

        while let Ok(hb) = r.recv().await {
            if let Ok(serialised) = serde_json::to_string(&hb) {
                yield ws::Message::Text(serialised)
            } else {
                eprintln!("Couldn't serialise into json: {:?}", hb)
            }
        }
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/heartbeat", data = "<hb>")]
fn ingest_heartbeat(state: &rocket::State<State>, hb: Json<Heartbeat>) -> &'static str {
    println!("hey, {:?}", hb);

    let _ = state.tx.send(hb.0);

    "Received"
}

#[launch]
fn rocket() -> _ {
    let (tx, _) = broadcast::channel(16);

    rocket::build()
        .mount("/", routes![hello, ingest_heartbeat, echo_stream])
        .manage(State { tx })
}

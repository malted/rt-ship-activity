#![feature(async_closure)]

#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize};
use rocket::tokio::sync::broadcast;
use rocket_ws as ws;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Heartbeat<'r> {
    name: &'r str,
}

struct State {
    tx: broadcast::Sender<String>,
}

#[get("/echo")]
fn echo_stream(ws: ws::WebSocket, state: &rocket::State<State>) -> ws::Stream!['static] {
    // let ws = ws.config(ws::Config {
    //     ..Default::default()
    // });

    let mut r = state.tx.subscribe();

    ws::Stream! { ws =>
        while let Ok(hb) = r.recv().await {
            yield ws::Message::Text(hb);
        }
    }

    // ws::Stream! { ws =>
    //     while let Ok(i) = r.recv().await {
    //         let is = i.to_string();
    //         yield is;
    //         // println!("got = {}", i);
    //     }

    //     // for await message in ws {

    //     // }
    // }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/heartbeat", data = "<hb>")]
fn ingest_heartbeat(state: &rocket::State<State>, hb: Json<Heartbeat<'_>>) -> &'static str {
    println!("hey, {:?}", hb.name);

    let _ = state.tx.send(hb.name.to_string());

    "Received"
}

#[launch]
fn rocket() -> _ {
    let (tx, _) = broadcast::channel(16);

    rocket::build()
        .mount("/", routes![hello, ingest_heartbeat, echo_stream])
        .manage(State { tx })
}

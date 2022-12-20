#![feature(iter_advance_by)]
mod render;

use futures_util::{FutureExt, StreamExt};
use warp::Filter;
use render::Item;
use render::template::HTMLTemplate;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let test = HTMLTemplate::new("index.html");

    let Ok(body) = test.compile() else {
        panic!("{}", "unable to compile html");
    };

    println!("{}", body);

    /*
    let index = warp::path::end().map(move || warp::reply::html(body.clone()));

    let echo = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                println!("recv: {:?}", tx);
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let routes = warp::get().and(
        index
            .or(echo)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
     */
}
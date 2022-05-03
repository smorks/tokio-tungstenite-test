use std::time::Duration;

use async_std::task;
use async_tungstenite::tungstenite::Message;
use futures::{pin_mut, select, FutureExt};
use futures_util::StreamExt;
use url::Url;

fn main() {
    task::block_on(run());
}

async fn run() {
    let url = "wss://lemmy.ca/api/v3/ws";
    let req_url = Url::parse(url).expect("invalid url");

    let (ws, _r) = async_tungstenite::async_std::connect_async(req_url)
        .await
        .expect("error connecting to websocket");

    let (ws_w, ws_r) = ws.split();
    let (tx, rx) = futures::channel::mpsc::unbounded();

    let fwd_to_ws = rx.map(Ok).forward(ws_w).fuse();

    let ws_read = ws_r
        .for_each(|r_msg| async {
            let msg = r_msg.expect("read error");
            println!("read msg: {:?}", &msg);
            let _s = msg.into_text().unwrap();
        })
        .fuse();

    let sleep = task::sleep(Duration::from_secs(30)).fuse();

    pin_mut!(ws_read, fwd_to_ws, sleep);

    let do_stuff_join = task::spawn(do_stuff(tx));

    select! {
        _ = ws_read => println!("ws_read done"),
        fwd_r = fwd_to_ws => {
             print!("fwd_to_ws done: ");
             match fwd_r {
                 Ok(_) => println!("no error."),
                 Err(e) => println!("error: {}", e),
             }
        }
        _ = sleep => println!("sleep done"),
    };

    let _ = do_stuff_join.await;
}

async fn do_stuff(tx: futures::channel::mpsc::UnboundedSender<Message>) {
    println!("doing stuff...");
    // if login() isn't called, then Ping messages are read, but then fwd_to_ws ends when this function ends?
    login(tx);
    let _slp = task::sleep(Duration::from_secs(10)).await;
    println!("done stuff.");
}

fn login(tx: futures::channel::mpsc::UnboundedSender<Message>) {
    let json =
        r#"{"op":"Login","data":{"username_or_email":"testing123","password":"password123"}}"#;

    // this causes fwd_to_ws to finish?
    tx.unbounded_send(Message::Text(String::from(json)))
        .expect("error sending message.");
}

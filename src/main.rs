extern crate ws;

fn listen_on_websocket() {
    ws::listen("127.0.0.1:3012", |out|{
        move |msg| {
            out.send(msg)
        }
    }).unwrap()
}

fn main() {
    println!("Listening on websocket...");
    listen_on_websocket();
}


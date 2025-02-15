#[macro_use] extern crate rocket;
use rocket::futures::{SinkExt, StreamExt};

#[get ("/<name>")]
fn client(ws :ws::WebSocket ,name :&str)->ws::Channel<'_>{
    ws.channel(move |mut stream| Box::pin(async move{
        let messages = format!("hello {}",name);
        let _ = stream.send(messages.into()).await;
        Ok(())
    }))
}

#[get ("/server")]
fn server(ws :ws::WebSocket)->ws::Channel<'static>{
    ws.channel(move |mut stream| Box::pin(async move{
        while let Some(message) = stream.next().await {
            let _ = stream.send(message?).await;
        }
        Ok(())
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/client", routes![client])
        .mount("/server", routes![server])
}



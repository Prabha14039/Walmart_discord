#[macro_use] extern crate rocket;
mod server;

#[get ("/world/<name>")]
fn fetch(name :&str)-> String{
    server::helper();
    format!("Hello {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch])
}



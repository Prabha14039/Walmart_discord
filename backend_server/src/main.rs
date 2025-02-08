use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::{Read,Write};


fn handle_client(mut stream: TcpStream) {
    let msg = b"helle";
    stream.write(msg).unwrap();
    println!("sent hello");
}

fn main() {
    let  socket = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in socket.incoming(){
        match stream {
            Ok(stream) => {
                println!("client connected at port 8080");
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err (_e) => {println!("error");}
        }
    }
}

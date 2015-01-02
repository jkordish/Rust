use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // bind the listener to the specified address
    let mut acceptor = listener.listen();

    println!("[INFO] Server Running")

    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        match stream {
        Err(e) => { /* connection failed */ }
        Ok(stream) => spawn(proc() {
            // connection succeeded
            handle_client(stream);
            })
        }
    }

    // close the socket server
    drop(acceptor);

}

fn handle_client(mut stream: TcpStream) {

    println!("[INFO] Connection")

    stream.write(b"Type Something\r\n");

    let mut buf = [0, ..1024];

    loop {
        match stream.read(&mut buf) {
            Ok(n) => stream.write(buf.slice_to(n)),
            Err(_) => break,
        };
    }
}

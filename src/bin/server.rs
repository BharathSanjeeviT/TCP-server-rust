use std::env;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use chatmf::ThreadPool;

fn main() {
    let n_pool: u32 = if let Some(m) = env::args().nth(2) {
        match m.parse() {
            Ok(m) => m,
            Err(_) => { println!("Initialised Pooling with 2 connections"); 2
            }
        }
    } else { println!("Initialised Pooling with 2 connections"); 2 };
    let server_ip: String = if let Some(m) = env::args().nth(1) {
        m
    } else {
        println!("Initialised Pooling with 2 connections");
        "127.0.0.1:6969".to_string()
    };
    let listener = TcpListener::bind(&server_ip).unwrap();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    
    println!("Server started Listening at {server_ip}");

    let mut pool = ThreadPool::new(n_pool as i32);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                clients.lock().unwrap().push(stream.try_clone().unwrap());
                println!("New Connection : {}", stream.peer_addr().unwrap());
                pool.execute(stream, clients);
            }
            Err(e) => println!("Client Error: {e:?}"),
        }
    }
}

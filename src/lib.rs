use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self};

pub struct ThreadPool { size: i32 }

impl ThreadPool {
    pub fn new(n: i32) -> Self {
        Self { size: n }
    }

    pub fn execute(&mut self, stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
        if self.size == 0 {
            eprintln!("Unable to execute thread: max count reached");
        } else {
            self.size -= 1;
            thread::spawn(move || {
                handle_conn(stream, clients);
            });
            self.size += 1;
        }
    }
}

fn handle_conn(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 64];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                print!("Connection Break : ");
                break;
            }
            Ok(size) => {
                let rec_msg = String::from_utf8_lossy(&buffer[..size]);
                print!("Received: {rec_msg}");
                let clients = clients.lock().unwrap();
                for client in clients.iter() {
                    let mut client = client.try_clone().unwrap();
                    if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                        if let Err(e) = client.write_all(
                            format!("{} : {}", stream.peer_addr().unwrap(), rec_msg).as_bytes(),
                        ) {
                            eprint!("ERRORRRRR + {e}");
                        }
                    }
                }
            }
            Err(e) => {
                println!("{:?} ERRORRRRRRR", e);
                break;
            }
        }
    }
    let mut clients = clients.lock().unwrap();
    if let Some(pos) = clients
        .iter()
        .position(|x| x.peer_addr().unwrap() == stream.peer_addr().unwrap())
    {
        clients.remove(pos);
        println!("A client disconncted");
    }
}

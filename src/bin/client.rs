use std::io::prelude::*;
use std::io;
use std::net::TcpStream;

fn read_user_input () -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn handle_write (stream:&mut TcpStream) -> bool {
    let user_input = String::from(read_user_input());

    if user_input != "q" {
        stream.write(user_input.as_bytes()).unwrap();
        stream.flush().unwrap();
        return false;
    }
        return true;
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    stream.set_nonblocking(true).unwrap();
    let mut quit = false;
    while !quit {
        quit = handle_write(&mut stream);
    }
    Ok(())
}

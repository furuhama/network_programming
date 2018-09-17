use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
use std::time::Duration;
use rand::{thread_rng, Rng};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection has come from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }

        // Set sleeping time before send response
        let sleep = Duration::from_secs(*thread_rng().choose(&[0, 1, 2, 3, 4]).unwrap());
        println!("Sleeping for {:?} seconds before send response", sleep);

        // Sleep for a while
        thread::sleep(sleep);

        // Send response
        stream.write(&buf[..bytes_read])?;
    }
}

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Could not bind port `8080`");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

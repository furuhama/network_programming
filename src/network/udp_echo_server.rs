use std::thread;
use std::net::UdpSocket;

pub fn run() {
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("Could not bind port `8080`");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src).expect("Failed to send a response");
                });
            },
            Err(e) => {
                eprintln!("Could not receive a datagram: {}", e);
            }
        }
    }
}

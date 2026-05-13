//Otus rust hw03 Socket network simulator
//v 0.3.0
//
use std::env::{self, Args};
use std::io::Read;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut args: Args = env::args();

    let address = args.nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("Try to connect address {address}");
    let connection = TcpListener::bind(&address);

    match connection {
        Ok(listener) => {
            println!("Server is listening on {}", address);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("New client connected: {}", stream.peer_addr().unwrap());
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(bytes_read) => {
                                println!("Read {} bytes from client", bytes_read);
                            }
                            Err(e) => {
                                eprintln!("Failed to read from client: {}", e);
                            }
                        }
                        let request = String::from_utf8_lossy(&buf)
                            .chars()
                            .take_while(|&c| c != '\n')
                            .collect::<String>();
                        println!(
                            "Received data from client: {:?}",
                            String::from_utf8_lossy(request.as_bytes())
                        );

                        todo!(
                            "Process client request and send response if needed: status: ON/OFF, current power"
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to accept client: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
        }
    }
}

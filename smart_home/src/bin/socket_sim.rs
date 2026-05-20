//Otus rust hw03 Socket network simulator
//v 0.3.0
//
use std::env::{self};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct ConnectionConfig {
    ip: String,
    port: u16,
    is_connected: Arc<Mutex<bool>>,
    state: Arc<Mutex<String>>,
    max_power: f32,
}

impl ConnectionConfig {
    fn new(address: &str) -> Self {
        let (_ip, _port) = match address.split_once(':') {
            Some((_ip, _port)) => (_ip.to_string(), _port.parse::<u16>().unwrap_or(0_u16)),
            None => ("127.0.0.1".to_string(), 0_u16),
        };

        Self {
            ip: _ip,
            port: _port,
            is_connected: Arc::new(Mutex::new(false)),
            state: Arc::new(Mutex::new(String::from("OFF"))),
            max_power: 1000.0,
        }
    }
}

fn main() {
    let configs: Vec<ConnectionConfig> = env::args()
        .skip(1)
        .map(|address| ConnectionConfig::new(address.as_str()))
        .collect();
    println!(
        "Configurations: {:?}",
        configs
            .iter()
            .map(|c| format!("{}:{}", c.ip, c.port))
            .collect::<Vec<String>>()
    );

    let tcp_listerer_threads: Vec<thread::JoinHandle<()>> = configs
        .clone()
        .into_iter()
        .map(|config| {
            let cloned_config = config.clone();
            println!(
                "Spawning thread for address {}:{}",
                cloned_config.ip, cloned_config.port
            );
            thread::spawn(move || {
                println!(
                    "Try to connect address {}:{}",
                    cloned_config.ip, cloned_config.port
                );
                let tcp_listener =
                    TcpListener::bind(format!("{}:{}", cloned_config.ip, cloned_config.port))
                        .unwrap_or_else(|e| {
                            panic!(
                                "Failed to bind to address {}:{}: {}",
                                cloned_config.ip, cloned_config.port, e
                            )
                        });
                println!(
                    "Listening for incoming connections on {}:{}",
                    cloned_config.ip, cloned_config.port
                );
                for stream in tcp_listener.incoming() {
                    println!(
                        "New client connected to {}:{}",
                        cloned_config.ip, cloned_config.port
                    );
                    match stream {
                        Ok(mut stream) => loop {
                            let mut buf = [0; 64];
                            match stream.read(&mut buf) {
                                Ok(bytes_read) => {
                                    if bytes_read > 0 {
                                        println!("Read {} bytes from client", bytes_read);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to read from client: {}", e);
                                }
                            }
                            let request = String::from_utf8_lossy(&buf)
                                .chars()
                                .take_while(|&c| c != '\0')
                                .collect::<String>();
                            if !request.is_empty() {
                                println!(
                                    "Received data from client: {:?}",
                                    String::from_utf8_lossy(request.as_bytes())
                                );
                                match request.as_str() {
                                    "GET STATE" => {
                                        let _writed = stream
                                            .write(config.state.lock().unwrap().as_bytes())
                                            .unwrap_or_else(|e| {
                                                eprintln!("Failed to write to client: {}", e);
                                                0
                                            });
                                    }

                                    "GET POWER" => {
                                        let power = if *config.state.lock().unwrap() == "ON" {
                                            rand::random_range(0.0..config.max_power)
                                        } else {
                                            0.0
                                        };
                                        let _writed = stream
                                            .write(power.to_string().as_bytes())
                                            .unwrap_or_else(|e| {
                                                eprintln!("Failed to write to client: {}", e);
                                                0
                                            });
                                    }

                                    "SET ON" => {
                                        *config.state.lock().unwrap() = "ON".to_string();
                                        let _writed =
                                            stream.write("OK".as_bytes()).unwrap_or_else(|e| {
                                                eprintln!("Failed to write to client: {}", e);
                                                0
                                            });
                                    }

                                    "SET OFF" => {
                                        *config.state.lock().unwrap() = "OFF".to_string();
                                        let _writed =
                                            stream.write("OK".as_bytes()).unwrap_or_else(|e| {
                                                eprintln!("Failed to write to client: {}", e);
                                                0
                                            });
                                    }
                                    _ => eprintln!("Uncknown request"),
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to accept client: {}", e);
                        }
                    }
                }
            })
        })
        .collect();
    tcp_listerer_threads
        .into_iter()
        .for_each(|t| t.join().unwrap());
}

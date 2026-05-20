//Otus rust hw03 Thermometer network simulator
//v 0.3.0

use serde::Deserialize;
use std::net::UdpSocket;

#[derive(Deserialize, Debug)]
struct Config {
    connection: ConnectionConfig,
}

#[derive(Deserialize, Debug)]
struct ConnectionConfig {
    ip: String,
    port: u16,
    interval_ms: u64,
}

fn simulate_temperature() -> f64 {
    // Simulate a temperature reading between 15.0 and 30.0 degrees Celsius
    rand::random_range(10.0..30.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Thermometer network simulator");

    if args.len() < 2 {
        println!("Usage: thermometer_sim <config_toml_file>");
        return Err("Config file not provided".into());
    }

    let configs: Vec<Config> = args[1..]
        .iter()
        .map(|config_toml_file| {
            println!("Reading config from: {}", config_toml_file);
            let contents = std::fs::read_to_string(config_toml_file)?;
            let config: Config = toml::from_str(&contents)?;
            println!("Config: {:?}", config);
            Ok(config)
        })
        .collect::<Result<_, Box<dyn std::error::Error>>>()?;

    let connections = configs
        .iter()
        .map(|config| {
            let remote_address = format!("{}:{}", config.connection.ip, config.connection.port);
            let interval_ms = config.connection.interval_ms;
            let connection =
                UdpSocket::bind(format!("127.0.0.1:{}", config.connection.port + 1000))
                    .expect("Failed to bind socket");
            (connection, remote_address, interval_ms)
        })
        .collect::<Vec<(UdpSocket, String, u64)>>();

    let treads: Vec<std::thread::JoinHandle<()>> = connections
        .into_iter()
        .map(|(connection, remote_address, interval_ms)| {
            std::thread::spawn(move || {
                println!(
                    "Starting to send data to {}:{}",
                    remote_address,
                    connection.local_addr().unwrap()
                );
                loop {
                    let temp = simulate_temperature();
                    let message = format!("{:.2}", temp);
                    connection
                        .send_to(message.as_bytes(), &remote_address)
                        .expect("Failed to send message");
                    println!("Send {} to {}", message, remote_address);
                    std::thread::sleep(std::time::Duration::from_millis(interval_ms));
                }
            })
        })
        .collect();
    treads.into_iter().for_each(|t| t.join().unwrap());

    Ok(())
}

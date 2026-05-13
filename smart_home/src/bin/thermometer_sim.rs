//Otus rust hw03 Thermometer network simulator
//v 0.3.0

use rand::Rng;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    let mut rng = rand::thread_rng();
    let temp: f64 = rng.gen_range(15.0..30.0);
    temp
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Thermometer network simulator");

    if args.len() < 2 {
        println!("Usage: thermometer_sim <config_toml_file>");
        return Err("Config file not provided".into());
    }

    let config_toml_file = &args[1];
    println!("Reading config from: {}", config_toml_file);
    let contents = std::fs::read_to_string(config_toml_file)?;
    let config: Config = toml::from_str(&contents)?;
    println!("Config: {:?}", config);

    let remote_address = format!("{}:{}", config.connection.ip, config.connection.port);
    let connection = UdpSocket::bind("127.0.0.1:3000")?;
    // println!("UDP connection established to {}", remote_address);

    loop {
        let temp = simulate_temperature();
        let message = format!("{{\"temperature\": {:.2}}}", temp);
        connection
            .send_to(message.as_bytes(), remote_address.clone())
            .expect("Failed to send message");
        println!("Sent: {}", message);
        std::thread::sleep(std::time::Duration::from_millis(
            config.connection.interval_ms,
        ));
    }

    Ok(())
}

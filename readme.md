# Otus Rust Developer Homeworks
## Homework 3

Refactoring the Smart Home crate
Goals:

Implement remote communication features for Socket and Thermometer and create  device simulator for tests

Step by step instructions:

Socket:

    * Has same basic functions: on/off + get_power
    * Sync communication via TCP
    * A Socket can use real TCP update and simulated (for test)

Socket simulator:

    * Read ip for TCP-connection from cli args
    * Implemented as non-blocking network communication
    * It can store a socket state
    * It can manage socket by many clients at the same time

Thermometer:

    * Has same basic functions: get_temperature 
    * Get temperature as UDP-packets in parallel thread
    * The parallel thread starts when new Thermometer created and stop when it droped
    * Thermometer object can return last received temperature value
    * Thermometer can simulate remote receive temperature value (for tests)

Thermometer simulator:

    * Implemented as non blocked network TCP-connection
    * Read ip for send UDP-packets and time interval sending from configuration file 
    * Send random temperature to ip with configured interval

Add new example with Smarthome, Sockets and Thermometers for work with simulators. Example should:

    * Report home state if simulators are started
    * Send error message if some device don't get data

Acceptance:

    * Package successfully build without errors
    * Example successfully run and print the report about a SmartHome
    * Cargo clippy and cargo fmt --check without errors and warnings
    * All tests passed


Some shell scripts:

    cargo build to build everything
    cargo run --bin socket_sim -- "127.0.0.1:8000" (or ./smart_home/run_socket_sim.sh)
    cargo run --bin thermometer_sim -- settings.toml (or ./smart_home/run_thermometer_sim.sh)
    cargo run --example hw03 to run the main example
    cargo test to run tests
    cargo clippy / cargo fmt --check for lint/format checks

settings.toml format:

    [connection]
    ip = "127.0.0.1"
    port = 8000
    interval_ms = 1000

File structure:

    smart_home/
      src/
        lib.rs              # Core library (Home, Room, Socket, Thermometer)
        bin/
          socket_sim.rs     # TCP socket device simulator
          thermometer_sim.rs# UDP thermometer simulator
      examples/
        hw03.rs             # Example usage
      settings.toml         # Thermometer simulator config
      build.rs              # Copies settings.toml to target directory


Some information:

    * socket_sim has a todo!() for processing client requests — this is incomplete
    * Thermometer uses UdpSocket as a field but doesn't actually receive from it (only generates random values via rand)
    * hw03.rs example is essentially empty (fn main() {})
    * Tests for socket_test assert socket1.get_power() != 0.0 which may be flaky since power comes from random when remote read fails

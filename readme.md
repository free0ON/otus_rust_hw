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
    * Implemented as non block network communication
    * It can store a socket state
    * It can manage socket by many clients at the same time

Thermometer:

    * Has same basic functons: get_temperature 
    * Get temperature as UDP-pakets in parallel thread
    * The parallel thead starts then new Thermometer created and stop then it droped
    * Thermometer object can return last recived temperature value
    * Thermometer can simulate remote recive temperature value (for tests)

Thermometer simulator:

    * Implemented as non blocked network TCP-connection
    * Read ip for send UDP-pakets and time interval sending from configuration file 
    * Send random temperature to ip with setted interval

Add new example with Smarthome, Sockets and Thermometers for work with simulators. Example should:

    * Report home state if simulators are started
    * Send error message if some device dont get data

Acceptance:

    * Package sucsessfully build without errors
    * Example sucsessfully run and print the report about a SmartHome
    * Cargo clippy and cargo fmt --check without errors and warnings
    * All tests passed

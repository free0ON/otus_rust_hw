# Otus Rust Developer Homeworks
## Homework 2
Refactoring the Smart Home crate
Goals:

Refactorings Smart Home crate with Rust std library 

Step by step instructions:

Add error handling

Panic -> return Option in getter room by key
Panic -> return Option in getter device by key

Refactor objects storage:

    * Replace &[] devices and rooms with std collections. Use string as key.
    * Implement Debug trait with all custom types
    * Implement dynamic add/delete device in to Room
    * Implement dymanic add/delete room in to Home
    * Implement device reference getter for Home. Getter should get room name and device name. In case device not found return error with details. Error should implement trait std::error::Error 
    * Implement trait From to convert objects Sockets and Thermometers in to SmartDevice objects.
    * Create macros for simple room construct. It gets tuples (key, Socket) or (key, Thermometer) and return Room with all devices with keys.

Refactor report generator:
    * Make report method in trait and implement it with all custom types: SmartDevice, Room, Home

Refactor tests with new features.

Refactor example application:
    * Dynamic add/delete room
    * Dynamic add/delete device
    * Add function witch get any object with report create ability. Make with this function report by some home, some room, some device
    * Show error handling features

Acceptance:
    * Package sucsessfully build without errors
    * Example sucsessfully run and print the report about a SmartHome
    * Cargo clippy and cargo fmt --check without errors and warnings
    * All tests passed

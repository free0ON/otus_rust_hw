# Otus Rust Developer Homeworks
## Homework 2
Refactoring the Smart Home crate
Goals:

Refactoring Smart Home crate with Rust std library 

Step by step instructions:

Add error handling

    x Panic -> return Option in getter room by key
    x Panic -> return Option in getter device by key

Refactor objects storage:

    x Replace &[] devices and rooms with std collections. Use string as key.
    x Implement Debug trait with all custom types
    x Implement dynamic add/delete device in to Room
    x Implement dymanic add/delete room in to Home
    x Implement device reference getter for Home. Getter should get room name and device name. In case device not found return error with details. Error should implement trait std::error::Error 
    x Implement trait From to convert objects Sockets and Thermometers in to SmartDevice objects.
    x Create macros for simple room construct. It gets tuples (key, Socket) or (key, Thermometer) and return Room with all devices with keys.

Refactor report generator:

    x Make report method in trait and implement it with all custom types: SmartDevice, Room, Home

Refactor tests with new features.

Refactor example application:
    
    x Dynamic add/delete room
    x Dynamic add/delete device
    x Add function witch get any object with report create ability. Make with this function report by some home, some room, some device
    x Show error handling features

Acceptance:
    
    x Package sucsessfully build without errors
    x Example sucsessfully run and print the report about a SmartHome
    x Cargo clippy and cargo fmt --check without errors and warnings
    x All tests passed

# Otus Rust Developer Homeworks
## Homework 1
Template for crate Smart Home
Goals:

Make template for crate SmartHome and write demo exampls. 

Step by step instructions:

The crate and example in one package.

For Lib:
Implements as lib craite smart_home

    Discribe type: SmartTermometr. The type must provide:
        Constructor takes fields values and returns current temperature (random value). 
    
    Describe type: SmartSocket. The type must provide:
        Constructor takes fields values.
        On/off + get state.
        Get power: if off - 0, else - random value  

    Describe type: SmartDevice. The type must content one of device (SmartTermometr or SmartSocket) and provide:
        Print state info about device.
    
    Describe type: RoomSmartDevices. The type must provide:
        Constructor takes fields valuies.
        Get referense to device by index
        Get mut reference to device by index
        Print report info about all devices in the room
    
    Descripe type: SmartHome. The type must provide:
        Constructor takes rooms vector.
        Get reference to room by index
        Get mut referense to room by index
        Print report info about all rooms
        
    Array size is variable
    
    In case of outbouded array index application close with panic!


For Example:
    
    Implements as bin crate.
    Make an object of SmartHome and print report about it.
    For this object off one SmartSocket in some Room and update report
    
Acceptance:

    Package sucsessfully build without errors
    Example sucsessfully run and print the report about a SmartHome
    Cargo clippy and cargo fmt --check without errors and warnings
    All tests passed
    

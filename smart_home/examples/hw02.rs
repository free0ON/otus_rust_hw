use std::collections::HashMap;

// Otus hw02 example application
// v 0.2.1
use smart_home::*;

fn main() {
    // * Add function witch get any object with report create ability. Make with this function report by some home, some room, some device
    // * Show error handling features

    let kettle_socket = Socket::new("3", "Kettle power socket", SocketState::Off, 2000.0);

    let kettle_thermometer = Thermometer::new("4", "Kettle thermometer", 10.0, 40.0);

    let hall_charger_socket = Socket::new("3", "Hall charger socket", SocketState::On, 500.0);

    let kitchen = new_room!(
        "1",
        "Kitchen",
        (
            "1",
            Socket::new("1", "Coffe mashine power socket", SocketState::On, 1000.0)
        ),
        (
            "2",
            Thermometer::new("2", "Kitchen thermometer", 10.0, 40.0)
        ),
    );

    let hall = new_room!(
        "2",
        "Hall",
        (
            "1",
            Socket::new("1", "Lamps power socket", SocketState::Off, 150.0)
        ),
        ("2", Thermometer::new("2", "Hall thermometer", 10.0, 40.0)),
    );

    let mut home = Home::new("1", "My Home", HashMap::<String, Box<Room>>::new());

    println!(
        "
Add kitchen and hall to home"
    );
    home.add_room(kitchen);
    home.add_room(hall);

    report(&home);
    println!(
        "
Add 2 devises to kitchen and 1 device to hall"
    );
    home.get_mut_room("1")
        .expect("Room not found")
        .add_device(kettle_socket.into());
    home.get_mut_room("1")
        .expect("Room not found")
        .add_device(kettle_thermometer.into());
    home.get_mut_room("2")
        .expect("Room not found")
        .add_device(hall_charger_socket.into());

    report(&home);

    println!(
        "
Delete kettle socket from kitchen"
    );
    home.get_mut_room("2")
        .expect("Room not found")
        .delete_device("3");
    println!(
        "
Delete hall from home"
    );

    home.delete_room("2");

    println!(
        "
Report home, kitchen and kettle socket"
    );

    report(&home);

    report(home.get_room("1").expect("Room not found"));

    report(
        home.get_room("1")
            .expect("Room not found")
            .get_device("2")
            .expect("Device not found"),
    );

    println!(
        "
Error handling: try to get not existing room and device"
    );
    match home.get_device("1", "5") {
        Ok(_result) => (),
        Err(err) => println!(
            "
    Error get_device 5 from room 1: {}",
            err
        ),
    }
    match home.get_device("2", "1") {
        Ok(_result) => (),
        Err(err) => println!(
            "
    Error get_device 1 from room 2: {}",
            err
        ),
    }

    if home.get_room("2").is_none() {
        println!(
            "  
    Error get_room 2: Room 2 room was deleted"
        );
    }
}

use smart_home::*;

fn main() {
    let mut kitchen_devices = [
        Device::ThermometerDevice(Thermometer::new(1, "Kitchen sensor", 10.0, 50.0)),
        Device::SocketDevice(Socket::new(2, "Coffee machine", SocketState::On, 900.0)),
    ];

    let mut bedroom_devices = [
        Device::ThermometerDevice(Thermometer::new(1, "Bedroom sensor", 10.0, 40.0)),
        Device::SocketDevice(Socket::new(2, "Lamp", SocketState::On, 60.0)),
    ];

    let mut rooms = [
        Room::new(1, "Kitchen", &mut kitchen_devices),
        Room::new(2, "Bedroom", &mut bedroom_devices),
    ];

    let mut house = Home::new(1, "My Smart House", &mut rooms);

    println!("--- INITIAL REPORT ---");
    house.update();
    house.report();

    // Выключаем умную розетку в одной из комнат: Bedroom -> device #1 (Lamp).
    house
        .get_mut_room(1)
        .get_mut_device(1)
        .set_state(SocketState::Off);

    println!("\n--- AFTER TURNING OFF SOCKET ---");
    house.report();
}

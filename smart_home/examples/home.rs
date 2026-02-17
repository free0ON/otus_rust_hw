use smart_home::*;

fn main() {
    let therm1 = Thermometr::new(1, "Virtual thermometr".to_string(), -50.0, 50.0);
    let therm2 = Thermometr::new(2, "Virtual thermometr".to_string(), -50.0, 50.0);
    let socket1 = Socket::new(3, "Virtual socket".to_string(), SocketState::On, 1000.0);
    let socket2 = Socket::new(4, "Virtual socket".to_string(), SocketState::Off, 1000.0);

    let mut devices1 = [
        Device::ThermometrDevice(therm1),
        Device::ThermometrDevice(therm2),
        Device::SocketDevice(socket1),
        Device::SocketDevice(socket2),
    ];
    let mut room1 = Room::new(1, "Kitchen".to_string(), &mut devices1);

    let therm3 = Thermometr::new(1, "Virtual thermometr".to_string(), -50.0, 50.0);
    let therm4 = Thermometr::new(2, "Virtual thermometr".to_string(), -50.0, 50.0);
    let socket3 = Socket::new(3, "Virtual socket".to_string(), SocketState::On, 1000.0);
    let socket4 = Socket::new(4, "Virtual socket".to_string(), SocketState::Off, 1000.0);
    let mut devices2 = [
        Device::ThermometrDevice(therm3),
        Device::ThermometrDevice(therm4),
        Device::SocketDevice(socket3),
        Device::SocketDevice(socket4),
    ];
    let mut room2 = Room::new(2, "Hall".to_string(), &mut devices2);

    let rooms = &mut [&mut room1, &mut room2];
    let mut home = Home::new(1, "My home".to_string(), rooms);
    home.update();
    println!("Before Off {}", home);
    let room = home.get_mut_room(0);
    let device = room.get_mut_device(0);
    device.set_state(SocketState::Off);
    // println!("After Off {}", home);
    // let mut device5 = room5.get_mut_device(0);
}

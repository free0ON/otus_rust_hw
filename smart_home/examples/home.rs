use smart_home::*;

fn main() {
    let term1 = Thermometr::new(1, "Virtual thermometr".to_string(), -50.0, 50.0);
    let term2 = Thermometr::new(2, "Virtual thermometr".to_string(), -50.0, 50.0);
    let socket1 = Socket::new(1, "Virtual socket".to_string(), SocketState::On, 1000.0);
    let socket2 = Socket::new(2, "Virtual socket".to_string(), SocketState::Off, 1000.0);
    let devices1: Vec<&dyn Device> = vec![&term1, &term2, &socket1, &socket2];
    let room1 = Room::new(1, "Kitchen".to_string(), devices1);

    let term3 = Thermometr::new(3, "Virtual thermometr".to_string(), -50.0, 50.0);
    let term4 = Thermometr::new(4, "Virtual thermometr".to_string(), -50.0, 50.0);
    let socket3 = Socket::new(3, "Virtual socket".to_string(), SocketState::On, 1000.0);
    let socket4 = Socket::new(4, "Virtual socket".to_string(), SocketState::Off, 1000.0);
    let devices2: Vec<&dyn Device> = vec![&term3, &term4, &socket3, &socket4];
    let room2 = Room::new(2, "Hall".to_string(), devices2);

    let rooms = vec![&room1, &room2];
    let mut home = Home::new(1, "My home".to_string(), rooms);

    // println!("{}", home.report().as_str());
    let &mut room = home.get_mut_room(0);
    let device = room.get_mut_device(0);
    device.set_state(SocketState::Off);

    // let mut device5 = room5.get_mut_device(0);
}

// OTUS Rust hw02 0.2.0
// smart_home library refactoring
use std::fmt::{self, Display};
pub struct Home<'a> {
    id: usize,
    name: String,
    rooms: &'a mut [Room<'a>],
}

impl<'a> Home<'a> {
    pub fn new(_id: usize, _name: &str, _rooms: &'a mut [Room<'a>]) -> Self {
        Self {
            id: _id,
            name: _name.to_string(),
            rooms: _rooms,
        }
    }
    pub fn get_room(&self, index: usize) -> &Room<'a> {
        self.rooms
            .get(index)
            .unwrap_or_else(|| panic!("Room index is outbounded"))
    }

    pub fn get_mut_room(&mut self, index: usize) -> &mut Room<'a> {
        self.rooms
            .get_mut(index)
            .unwrap_or_else(|| panic!("Room index is outbouded"))
    }

    pub fn update(&mut self) {
        for room in self.rooms.iter_mut() {
            for device in room.devices.iter_mut() {
                device.update();
            }
        }
    }

    pub fn report(&self) {
        println!("{}", self);
    }
}

impl<'a> Display for Home<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum_rooms = self.rooms.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(x.to_string().as_str());
            acc
        });

        writeln!(
            f,
            "Report of Smarthome id {} name {} with rooms: \n{} ",
            self.id, self.name, sum_rooms
        )
    }
}

impl<'a> PartialEq for Home<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

pub struct Room<'a> {
    id: usize,
    name: String,
    devices: &'a mut [Device],
}

impl<'a> Room<'a> {
    pub fn new(_id: usize, _name: &str, _devices: &'a mut [Device]) -> Self {
        Self {
            id: _id,
            name: _name.to_string(),
            devices: _devices,
        }
    }

    pub fn get_device(&self, index: usize) -> &Device {
        if index < self.devices.len() {
            &self.devices[index]
        } else {
            panic!("Device index is outbounded");
        }
    }

    pub fn get_mut_device(&mut self, index: usize) -> &mut Device {
        if index < self.devices.len() {
            &mut self.devices[index]
        } else {
            panic!("Device index is outdounded")
        }
    }
}

impl<'a> PartialEq for Room<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl<'a> Display for Room<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum_devices = self.devices.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(x.to_string().as_str());
            acc
        });

        writeln!(
            f,
            "Room id {} name {} devices: \n{}",
            self.id, self.name, sum_devices
        )
    }
}

#[derive(Debug)]
pub enum Device {
    ThermometerDevice(Thermometer),
    SocketDevice(Socket),
}

impl Device {
    pub fn update(&mut self) {
        match self {
            Device::ThermometerDevice(thermometer) => thermometer.update_temperature(),
            Device::SocketDevice(socket) => socket.update_power(),
        }
    }

    pub fn set_state(&mut self, state: SocketState) {
        match self {
            Device::ThermometerDevice(thermometer) => thermometer.update_temperature(),
            Device::SocketDevice(socket) => {
                socket.update_power();
                socket.set_state(state);
            }
        }
    }

    pub fn get_state(&mut self) -> Option<SocketState> {
        match self {
            Device::ThermometerDevice(thermometer) => {
                thermometer.update_temperature();
                None
            }
            Device::SocketDevice(socket) => {
                socket.update_power();
                Some(socket.state)
            }
        }
    }

    pub fn get_power(&mut self) -> Option<f32> {
        match self {
            Device::ThermometerDevice(_) => None,
            Device::SocketDevice(socket) => {
                socket.update_power();
                Some(socket.power)
            }
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Device::ThermometerDevice(thermometer) => writeln!(f, "{}", thermometer),
            Device::SocketDevice(socket) => writeln!(f, "{}", socket),
        }
    }
}

#[derive(Debug)]
pub struct Thermometer {
    id: usize,
    name: String,
    temperature: f32,
    min_temperature: f32,
    max_temperature: f32,
}

impl Thermometer {
    pub fn new(_id: usize, _name: &str, _min_temperature: f32, _max_temperature: f32) -> Self {
        Self {
            id: _id,
            name: _name.to_string(),
            temperature: 0.0,
            min_temperature: _min_temperature,
            max_temperature: _max_temperature,
        }
    }

    fn update_temperature(&mut self) {
        self.temperature = rand::random_range(self.min_temperature..self.max_temperature);
    }

    pub fn get_temperature(&mut self) -> f32 {
        self.update_temperature();

        self.temperature
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl PartialEq for Thermometer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.max_temperature == other.max_temperature
            && self.min_temperature == other.min_temperature
    }
}

impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id {}, name {}, temperature {}",
            self.id, self.name, self.temperature
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketState {
    On,
    Off,
}

#[derive(Debug)]
pub struct Socket {
    id: usize,
    name: String,
    state: SocketState,
    power: f32,
    max_power: f32,
}

impl Socket {
    pub fn new(_id: usize, _name: &str, _state: SocketState, _max_power: f32) -> Self {
        Socket {
            id: _id,
            name: _name.to_string(),
            state: _state,
            power: 0.0,
            max_power: _max_power,
        }
    }

    pub fn set_state(&mut self, _state: SocketState) {
        self.state = _state;
        self.update_power();
    }

    pub fn get_state(&self) -> &SocketState {
        &self.state
    }

    pub fn get_power(&mut self) -> f32 {
        self.update_power();
        self.power
    }

    pub fn update_power(&mut self) {
        match self.state {
            SocketState::Off => self.power = 0.0,
            SocketState::On => self.power = rand::random_range(0.0..self.max_power),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id {}, name {}, state {:?},  power {}",
            self.id, self.name, self.state, self.power
        )
    }
}

impl PartialEq for Socket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.max_power == other.max_power
    }
}

#[cfg(test)]
mod tests {

    use crate::{Device, Home, Room, Socket, SocketState, Thermometer};
    #[test]
    fn thermometr_test() {
        let mut term1 = Thermometer::new(1, "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new(2, "Virtual thermometer", -50.0, 50.0);
        term1.update_temperature();
        assert!(term1 != term2);
    }

    #[test]
    fn socket_test() {
        let mut socket1 = Socket::new(1, "Virtual socket", SocketState::On, 1000.0);
        let mut socket2 = Socket::new(2, "Virtual socket", SocketState::Off, 1000.0);
        socket1.update_power();
        socket2.update_power();
        assert!(socket1 != socket2);
        assert!(socket2.get_power() == 0.0);
        assert!(socket1.get_power() != 0.0);
    }

    #[test]
    fn room_test() {
        let term1 = Thermometer::new(1, "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new(2, "Virtual thermometer", -50.0, 50.0);
        let socket1 = Socket::new(1, "Virtual socket", SocketState::On, 1000.0);
        let socket2 = Socket::new(2, "Virtual socket", SocketState::Off, 1000.0);
        let mut devices = [
            Device::ThermometerDevice(term1),
            Device::ThermometerDevice(term2),
            Device::SocketDevice(socket1),
            Device::SocketDevice(socket2),
        ];
        let room1 = Room::new(1, "Kitchen", &mut devices);
        println!("{}", room1);
    }

    #[test]
    fn home_test() {
        let term1 = Thermometer::new(1, "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new(2, "Virtual thermometer", -50.0, 50.0);

        let socket1 = Socket::new(1, "Virtual socket", SocketState::On, 1000.0);
        let socket2 = Socket::new(2, "Virtual socket", SocketState::Off, 1000.0);

        let mut devices1 = [
            Device::ThermometerDevice(term1),
            Device::ThermometerDevice(term2),
            Device::SocketDevice(socket1),
            Device::SocketDevice(socket2),
        ];
        let room1 = Room::new(1, "Kitchen", &mut devices1);

        let term3 = Thermometer::new(3, "Virtual thermometer", -50.0, 50.0);
        let term4 = Thermometer::new(4, "Virtual thermometer", -50.0, 50.0);
        let socket3 = Socket::new(3, "Virtual socket", SocketState::On, 1000.0);
        let socket4 = Socket::new(4, "Virtual socket", SocketState::Off, 1000.0);
        let mut devices2 = [
            Device::ThermometerDevice(term3),
            Device::ThermometerDevice(term4),
            Device::SocketDevice(socket3),
            Device::SocketDevice(socket4),
        ];
        let room2 = Room::new(2, "Hall", &mut devices2);

        let rooms = &mut [room1, room2];
        let mut home = Home::new(1, "My home", rooms);
        let room = home.get_mut_room(1);
        let device = room.get_mut_device(2);
        device.update();
        println!("{}", device);
        match device.get_state() {
            Some(state) => {
                assert!(state == SocketState::On);
            }
            _ => {
                panic!("Device is not a socket");
            }
        }
        println!("{}", device);
        device.update();
    }

    #[test]
    fn can_turn_off_socket_through_home_mut_refs() {
        let mut kitchen_devices = [Device::SocketDevice(Socket::new(
            1,
            "coffee",
            SocketState::On,
            800.0,
        ))];
        let mut rooms = [Room::new(1, "kitchen", &mut kitchen_devices)];
        let mut home = Home::new(1, "home", &mut rooms);

        home.get_mut_room(0)
            .get_mut_device(0)
            .set_state(SocketState::Off);

        let socket_after = home.get_mut_room(0).get_mut_device(0);
        match socket_after.get_state() {
            Some(state) => assert!(state == SocketState::Off),
            _ => panic!("Device is not a socket"),
        }
    }

    #[test]
    #[should_panic]
    fn test_panic_out_of_bound() {
        let term1 = Thermometer::new(1, "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new(2, "Virtual thermometer", -50.0, 50.0);
        let socket1 = Socket::new(1, "Virtual socket", SocketState::On, 1000.0);
        let socket2 = Socket::new(2, "Virtual socket", SocketState::Off, 1000.0);

        let mut devices = [
            Device::ThermometerDevice(term1),
            Device::ThermometerDevice(term2),
            Device::SocketDevice(socket1),
            Device::SocketDevice(socket2),
        ];
        let room = Room::new(1, "Kitchen", &mut devices);

        room.get_device(99);
    }
}

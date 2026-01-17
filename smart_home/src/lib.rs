use core::panic;
use std::any::Any;
use std::fmt::Display;
#[derive(Clone)]
pub struct Home<'a> {
    id: usize,
    name: String,
    rooms: Vec<&'a Room<'a>>,
}

impl<'a> Home<'a> {
    pub fn new(_id: usize, _name: String, _rooms: Vec<&'a Room<'a>>) -> Self {
        Self {
            id: _id,
            name: _name,
            rooms: _rooms,
        }
    }

    pub fn get_room(&self, index: usize) -> &Room<'_> {
        if index < self.rooms.len() {
            self.rooms[index]
        } else {
            panic!("Room index is outbounded")
        }
    }

    pub fn get_mut_room(&mut self, index: usize) -> &'a mut &Room<'a> {
        if index < self.rooms.len() {
            &mut self.rooms[index]
        } else {
            panic!("Room index is outbouded")
        }
    }

    pub fn report(&self) -> String {
        self.rooms.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(x.report().as_str());
            acc
        })
    }
}

impl<'a> PartialEq for Home<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
#[derive(Clone)]
pub struct Room<'a> {
    id: usize,
    name: String,
    devices: Vec<&'a dyn Device>,
}

impl<'a> Room<'a> {
    pub fn new(_id: usize, _name: String, _devices: Vec<&'a dyn Device>) -> Self {
        Self {
            id: _id,
            name: _name,
            devices: _devices,
        }
    }

    pub fn get_device(&self, index: usize) -> &dyn Device {
        if index < self.devices.len() {
            self.devices[index]
        } else {
            panic!("Device index is outbounded");
        }
    }

    pub fn get_mut_device(&mut self, index: usize) -> &mut &'a dyn Device {
        if index < self.devices.len() {
            &mut self.devices[index]
        } else {
            panic!("Device index is outdounded")
        }
    }

    pub fn report(&self) -> String {
        self.devices.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(x.report().as_str());
            acc
        })
    }
}

impl<'a> PartialEq for Room<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

pub trait Device: Any {
    fn update(&mut self);
    fn report(&self) -> String;
    fn set_state(&mut self, state: SocketState);
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn Device) -> bool
    where
        Self: Sized + PartialEq + Any,
    {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            println!("self: {}", self.report());
            println!("other: {}", other.report());
            *self == *other
        } else {
            false
        }
    }
}

impl Device for Thermometr {
    fn update(&mut self) {
        self.update_temperature();
    }

    fn report(&self) -> String {
        format!("id: {}, name: {} ", self.id, self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_state(&mut self, _state: SocketState) {}
}

impl Device for Socket {
    fn update(&mut self) {
        self.update_power();
    }

    fn report(&self) -> String {
        format!("id: {}, name: {}", self.id, self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_state(&mut self, _state: SocketState) {
        self.set_state(_state);
    }
}

#[derive(Debug)]
pub struct Thermometr {
    id: usize,
    name: String,
    temperature: f32,
    min_temperature: f32,
    max_temperature: f32,
}

impl Thermometr {
    pub fn new(_id: usize, _name: String, _min_temperature: f32, _max_temperature: f32) -> Self {
        Self {
            id: _id,
            name: _name,
            temperature: 0.0,
            min_temperature: _min_temperature,
            max_temperature: _max_temperature,
        }
    }

    fn update_temperature(&mut self) {
        self.temperature = rand::random_range(self.min_temperature..self.max_temperature);
    }

    pub fn get_temperature(&mut self) -> f32 {
        self.update();
        self.temperature
    }
}

impl PartialEq for Thermometr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.max_temperature == other.max_temperature
            && self.min_temperature == other.min_temperature
    }
}

impl Display for Thermometr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id {}, name {}, temperature {}",
            self.id, self.name, self.temperature
        )
    }
}

#[derive(Debug)]
pub enum SocketState {
    On,
    Off,
}

pub struct Socket {
    id: usize,
    name: String,
    state: SocketState,
    power: f32,
    max_power: f32,
}

impl Socket {
    pub fn new(_id: usize, _name: String, _state: SocketState, _max_power: f32) -> Self {
        Socket {
            id: _id,
            name: _name,
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
    use crate::{Device, Home, Room, Socket, SocketState, Thermometr};
    #[test]
    fn thermometr_test() {
        let mut term1 = Thermometr::new(1, "Virtual thermometr".to_string(), -50.0, 50.0);
        let term2 = Thermometr::new(2, "Virtual thermometr".to_string(), -50.0, 50.0);
        term1.update();
        println!("{}", term1);
        assert!(term1 != term2);
    }

    #[test]
    fn socket_test() {
        let mut socket1 = Socket::new(1, "Virtual socket".to_string(), SocketState::On, 1000.0);
        let mut socket2 = Socket::new(2, "Virtual socket".to_string(), SocketState::Off, 1000.0);
        socket1.update();
        socket2.update();
        println!("{}", socket1);
        assert!(socket1 != socket2);
        assert!(socket2.get_power() == 0.0);
        assert!(socket1.get_power() != 0.0);
    }

    #[test]
    fn room_test() {
        let term1 = Thermometr::new(1, "Virtual thermometr".to_string(), -50.0, 50.0);
        let term2 = Thermometr::new(2, "Virtual thermometr".to_string(), -50.0, 50.0);
        let socket1 = Socket::new(1, "Virtual socket".to_string(), SocketState::On, 1000.0);
        let socket2 = Socket::new(2, "Virtual socket".to_string(), SocketState::Off, 1000.0);
        let devices: Vec<&dyn Device> = vec![&term1, &term2, &socket1, &socket2];
        let room1 = Room::new(1, "Kitchen".to_string(), devices);
        assert!(term1.dyn_eq(room1.get_device(0)));
        println!("{}", room1.report().as_str());
        println!("{}", room1.get_device(0).report().as_str());
    }

    #[test]
    fn home_test() {
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
        let &mut room = home.get_mut_room(1);
        let device = room.get_mut_device(0);
        device.set_state(SocketState::Off);
        // assert!(home.get_room(0) != home.get_room(1));
    }
}

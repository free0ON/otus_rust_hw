// OTUS Rust hw03 0.3.0
// Implement async network communication with device simulator
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{self, Display};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};

pub trait Reportable {
    fn report(&self) -> String;
}

pub struct Home {
    id: String,
    name: String,
    rooms: HashMap<String, Box<Room>>,
}

impl Home {
    pub fn new(_id: &str, _name: &str, _rooms: HashMap<String, Box<Room>>) -> Self {
        Self {
            id: _id.to_string(),
            name: _name.to_string(),
            rooms: _rooms,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_room(&self, index: &str) -> Option<&Room> {
        self.rooms.get(index).map(|room| room.as_ref())
    }

    pub fn get_mut_room(&mut self, index: &str) -> Option<&mut Room> {
        self.rooms.get_mut(index).map(|room| room.as_mut())
    }

    pub fn update(&mut self) {
        for (_id, room) in self.rooms.iter_mut() {
            room.update();
        }
    }

    pub fn add_room(&mut self, _room: Room) {
        self.rooms
            .insert(_room.get_id().to_string(), Box::<Room>::new(_room));
    }

    pub fn delete_room(&mut self, id: &str) {
        self.rooms.remove(id);
    }

    pub fn get_device(&self, room: &str, device: &str) -> Result<&dyn Device, Box<dyn Error>> {
        match self.get_room(room) {
            Some(room) => match room.get_device(device) {
                Some(device) => Ok(device),
                None => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Device {device} not found"),
                ))),
            },
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Room {room} not found"),
            ))),
        }
    }
}

impl Reportable for Home {
    fn report(&self) -> String {
        format!(
            "
==========================
Report of {self}
=========================="
        )
    }
}
impl Display for Home {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum_rooms = self
            .rooms
            .iter()
            .fold(String::new(), |mut acc, (_id, room)| {
                acc.push_str(room.report().as_str());
                acc
            });

        writeln!(
            f,
            "Smarthome id {} name {} with rooms: \n{sum_rooms}",
            self.id, self.name
        )
    }
}

impl PartialEq for Home {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[macro_export]
macro_rules! new_home {
    ($id:expr, $name:expr, $(val:expr),* $(,)?) => {
        Home::new($id, $name, hash_map_room!($($val),*))
    };
}

#[macro_export]
macro_rules! new_home_from_rooms {
    ($id:expr, $name:expr, $(val:expr),* $(,)?) => {
        Home::new($id, $name, hash_map_room!($($val),*))
    };
}

#[macro_export]
macro_rules! hash_map_room {
    ($($val:expr),* $(,)?) => {
    {
        let mut map = std::collections::HashMap::new();
        $( map.insert($val.get_id(), Box::<Room>::new($val)); )*
        map
    }
    };
}

pub struct Room {
    id: String,
    name: String,
    devices: HashMap<String, Box<dyn Device>>,
}

impl Room {
    pub fn new(_id: &str, _name: &str, _devices: HashMap<String, Box<dyn Device>>) -> Self {
        Self {
            id: _id.to_string(),
            name: _name.to_string(),
            devices: _devices,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_device(&self, index: &str) -> Option<&dyn Device> {
        self.devices.get(index).map(|device| device.as_ref())
    }

    pub fn get_mut_device(&mut self, index: &str) -> Option<&mut dyn Device> {
        self.devices.get_mut(index).map(|device| device.as_mut())
    }

    pub fn update(&mut self) {
        for (_id, device) in self.devices.iter_mut() {
            device.update();
        }
    }

    pub fn add_device(&mut self, _device: Box<dyn Device>) {
        self.devices.insert(_device.get_id().to_string(), _device);
    }

    pub fn delete_device(&mut self, id: &str) {
        self.devices.remove(id);
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Reportable for Room {
    fn report(&self) -> String {
        format!(
            "
    ---------------------------
    Report of {self}"
        )
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum_devices = self
            .devices
            .iter()
            .fold(String::new(), |mut acc, (_id, device)| {
                acc.push_str(device.report().as_str());
                acc
            });

        writeln!(
            f,
            "Room id {} name {} devices: \n{}",
            self.id, self.name, sum_devices
        )
    }
}

#[macro_export]
macro_rules! new_room {
    ($id:expr, $name:expr, $(($key:expr, $val:expr)),* $(,)?) => {
        Room::new($id, $name, hash_map_device!($($key, $val),*))
    };
}
#[macro_export]
macro_rules! hash_map_device {
    ($($key:expr, $val:expr),* $(,)?) => {
    {
        let mut map = std::collections::HashMap::new();
        $( map.insert($key.to_string(), Box::<dyn Device>::from($val)); )*
        map
    }
    };
}

pub trait Device: Any + Reportable {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str {
        "Unknown device"
    }
    fn update(&mut self);
    fn set_state(&mut self, state: SocketState);
    fn get_state(&self) -> Option<SocketState>;
    fn as_any(&self) -> &dyn Any;
}

impl Device for Thermometer {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn update(&mut self) {
        self.update_temperature();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_state(&mut self, _state: SocketState) {}
    fn get_state(&self) -> Option<SocketState> {
        None
    }
}

impl Device for Socket {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn update(&mut self) {
        self.update_power();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_state(&mut self, _state: SocketState) {
        self.set_state(_state);
    }

    fn get_state(&self) -> Option<SocketState> {
        Some(self.state)
    }
}

pub struct Thermometer {
    id: String,
    name: String,
    temperature: f32,
    min_temperature: f32,
    max_temperature: f32,
    client: UdpSocket,
}

impl Thermometer {
    pub fn new(
        _id: &str,
        _name: &str,
        _min_temperature: f32,
        _max_temperature: f32,
        client: &str,
    ) -> Self {
        Self {
            id: _id.to_string(),
            name: _name.to_string(),
            temperature: 0.0,
            min_temperature: _min_temperature,
            max_temperature: _max_temperature,
            client: UdpSocket::bind(client).unwrap(),
        }
    }

    fn update_temperature(&mut self) {
        self.temperature = rand::random_range(self.min_temperature..self.max_temperature);
    }

    pub fn get_temperature(&mut self) -> f32 {
        self.update_temperature();

        self.temperature
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

impl Reportable for Thermometer {
    fn report(&self) -> String {
        format!(
            "
        {self}"
        )
    }
}

impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Thermometer id {}, name {}, temperature {}",
            self.id, self.name, self.temperature
        )
    }
}

impl From<Thermometer> for Box<dyn Device> {
    fn from(other: Thermometer) -> Self {
        Box::new(other)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketState {
    On,
    Off,
}

pub struct Socket {
    id: String,
    name: String,
    state: SocketState,
    power: f32,
    max_power: f32,
    address: String,
    remote_connection: TcpStream,
}

impl Socket {
    pub fn new(
        _id: &str,
        _name: &str,
        _state: SocketState,
        _max_power: f32,
        _address: &str,
    ) -> Self {
        Socket {
            id: _id.to_string(),
            name: _name.to_string(),
            state: _state,
            power: 0.0,
            max_power: _max_power,
            address: _address.to_string(),
            remote_connection: {
                match TcpStream::connect(_address) {
                    Ok(connection) => connection,
                    Err(err) => panic!("{err}"),
                }
            },
        }
    }

    pub fn set_state(&mut self, _state: SocketState) {
        self.state = _state;
        self.update_power();
        match _state {
            SocketState::On => self.remote_connection.write("ON".as_bytes()).unwrap(),
            SocketState::Off => self.remote_connection.write("OFF".as_bytes()).unwrap(),
        };
    }

    pub fn get_state(&self) -> Option<SocketState> {
        Some(self.state)
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

impl Reportable for Socket {
    fn report(&self) -> String {
        format!(
            "       
        {self}"
        )
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket id {}, name {}, state {:?},  power {}",
            self.id, self.name, self.state, self.power
        )
    }
}

impl PartialEq for Socket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.max_power == other.max_power
    }
}

impl From<Socket> for Box<dyn Device> {
    fn from(other: Socket) -> Self {
        Box::new(other)
    }
}

impl Debug for Box<dyn Device> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device: {}", self.report())
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Room id {}, name {}", self.id, self.name)
    }
}

impl Debug for Home {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Home id {}, name {}", self.id, self.name)
    }
}

pub fn report(reportable: &dyn Reportable) {
    println!("{}", reportable.report());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Device, Home, Room, Socket, SocketState, Thermometer};
    use std::collections::HashMap;
    #[test]
    fn thermometer_test() {
        let mut term1 = Thermometer::new("1", "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new("2", "Virtual thermometer", -50.0, 50.0);
        term1.update_temperature();
        assert!(term1 != term2);
    }

    #[test]
    fn socket_test() {
        let mut socket1 = Socket::new("1", "Virtual socket", SocketState::On, 1000.0);
        let mut socket2 = Socket::new("2", "Virtual socket", SocketState::Off, 1000.0);
        socket1.update_power();
        socket2.update_power();
        assert!(socket1 != socket2);
        assert!(socket2.get_power() == 0.0);
        assert!(socket1.get_power() != 0.0);
    }

    #[test]
    fn room_test() {
        let term1 = Thermometer::new("1", "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new("2", "Virtual thermometer", -50.0, 50.0);
        let socket1 = Socket::new("3", "Virtual socket", SocketState::On, 1000.0);
        let socket2 = Socket::new("4", "Virtual socket", SocketState::Off, 1000.0);
        let devices = HashMap::<String, Box<dyn Device>>::new();
        let mut room1 = Room::new("1", "Kitchen", devices);
        room1.add_device(term1.into());
        room1.add_device(term2.into());
        room1.add_device(socket1.into());
        room1.add_device(socket2.into());
        println!("{}", room1);
    }

    #[test]
    fn home_test() {
        let term1 = Thermometer::new("1", "Virtual thermometer", -50.0, 50.0);
        let term2 = Thermometer::new("2", "Virtual thermometer", -50.0, 50.0);

        let socket1 = Socket::new("3", "Virtual socket", SocketState::On, 1000.0);
        let socket2 = Socket::new("4", "Virtual socket", SocketState::Off, 1000.0);

        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        let term3 = Thermometer::new("5", "Virtual thermometer", -50.0, 50.0);
        let term4 = Thermometer::new("6", "Virtual thermometer", -50.0, 50.0);
        let socket3 = Socket::new("7", "Virtual socket", SocketState::On, 1000.0);
        let socket4 = Socket::new("8", "Virtual socket", SocketState::Off, 1000.0);
        room1.add_device(Box::<dyn Device>::from(term1));
        room1.add_device(Box::<dyn Device>::from(term2));
        room1.add_device(Box::<dyn Device>::from(socket1));
        room1.add_device(Box::<dyn Device>::from(socket2));
        let mut room2 = Room::new("2", "Hall", HashMap::<String, Box<dyn Device>>::new());

        room2.add_device(term3.into());
        room2.add_device(term4.into());
        room2.add_device(socket3.into());
        room2.add_device(socket4.into());

        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        home.add_room(room1);
        home.add_room(room2);
        let room = match home.get_mut_room("1") {
            Some(room) => room,
            None => panic!("Room was not found"),
        };
        let device = match room.get_mut_device("3") {
            Some(device) => device,
            None => panic!("Room wasd not found"),
        };
        device.update();
        println!("{}", device.report());
        match device.get_state() {
            Some(state) => {
                assert!(state == SocketState::On);
            }
            _ => {
                panic!("Device is not a socket");
            }
        }
        println!("{}", device.report());
        device.update();
    }

    #[test]
    fn can_turn_off_socket_through_home_mut_refs() {
        let socket1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        room1.add_device(socket1.into());
        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        home.add_room(room1);
        home.get_mut_room("1")
            .expect("Room not found")
            .get_mut_device("1")
            .expect("Device not found")
            .set_state(SocketState::Off);

        let socket_after = home
            .get_mut_room("1")
            .expect("Room not found")
            .get_mut_device("1")
            .expect("Device non found");
        match socket_after.get_state() {
            Some(state) => assert!(state == SocketState::Off),
            _ => panic!("Device is not a socket"),
        }
    }

    #[test]
    fn get_device_test() {
        let socket1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        room1.add_device(socket1.into());
        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        home.add_room(room1);
        let device = home
            .get_device("1", "1")
            .expect("Device not found")
            .as_any()
            .downcast_ref::<Socket>()
            .expect("Device is not a socket");
        assert!(device.get_name() == "Coffee mashine power socket");
        assert!(home.get_device("1", "0").is_err());
    }

    #[test]
    fn add_room_test() {
        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        let room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        home.add_room(room1);
        assert!(home.get_room("1").is_some());
    }

    #[test]
    fn add_device_test() {
        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        let socket1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        room1.add_device(socket1.into());
        assert!(room1.get_device("1").is_some());
    }

    #[test]
    fn debug_test() {
        let socket1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        room1.add_device(socket1.into());
        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        home.add_room(room1);
        println!("{:?}", home);
        println!("{:?}", home.get_room("1").expect("Room not found"));
        println!(
            "{:?}",
            home.get_room("1")
                .expect("Room not found")
                .get_device("1")
                .expect("Device not found")
                .as_any()
        );
    }

    #[test]
    fn test_macro_new_room() {
        let room = new_room!(
            "1",
            "Kitchen",
            (
                "1",
                Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0)
            ),
            (
                "2",
                Thermometer::new("2", "Virtual thermometer", -50.0, 50.0)
            ),
            (
                "3",
                Socket::new("3", "TV power socket", SocketState::Off, 1000.0)
            )
        );
        assert!(
            room.get_device("1").expect("Device not found").get_state() == Some(SocketState::On)
        );
        assert!(
            room.get_device("3").expect("Device not found").get_state() == Some(SocketState::Off)
        );
    }

    #[test]
    fn reportable_test() {
        let socket1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        room1.add_device(socket1.into());
        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        home.add_room(room1);
        report(&home);
        report(home.get_room("1").expect("Room not found"));
        report(home.get_device("1", "1").expect("Device not found"));
    }

    #[test]
    fn error_handling_test() {
        let mut home = Home::new("1", "My home", HashMap::<String, Box<Room>>::new());
        let room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        let device1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        home.add_room(room1);
        home.get_mut_room("1")
            .expect("Room not found")
            .add_device(device1.into());
        assert!(home.get_device("1", "1").is_ok());
        assert!(home.get_device("2", "1").is_err());
        assert!(home.get_room("1").is_some());
        assert!(home.get_room("2").is_none());
    }
}

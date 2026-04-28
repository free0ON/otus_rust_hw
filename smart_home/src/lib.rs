// OTUS Rust hw02 0.2.0
// smart_home library refactoring
use std::any::Any;
use std::fmt::{self, Display};
use std::collections::HashMap;
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
    pub fn get_room(&self, index: &str) -> Option<&Box<Room>> {
        self.rooms.get(index) 
   }

    pub fn get_mut_room(&mut self, index: &str) -> Option<&mut Box<Room>> {
        self.rooms.get_mut(index)
    }

    pub fn update(&mut self) {
        for (_id, room) in self.rooms.iter_mut() {
            room.update();               
        }
    }

    pub fn report(&self) {
        println!("{}", self);
    }

    pub fn add_room(&mut self, id: &str, _room: Box<Room>) {
        self.rooms.insert(id.to_string(), _room);
    }

    pub fn delete_room(&mut self, id: &str) {
        self.rooms.remove(id);
    }
}

impl Display for Home {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum_rooms = self.rooms.iter().fold(String::new(), |mut acc, (_id, room)| {
            acc.push_str(room.to_string().as_str());
            acc
        });

        writeln!(
            f,
            "Report of Smarthome id {} name {} with rooms: \n{} ",
            self.id, self.name, sum_rooms
        )
    }
}

impl PartialEq for Home {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
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

    pub fn get_device(&self, index: &str) -> Option<&Box<dyn Device>> {
        self.devices.get(index)
    }

    pub fn get_mut_device(&mut self, index: &str) -> Option<&mut Box<dyn Device>> {
        self.devices.get_mut(index)
    }

    pub fn update(&mut self) {
        for (_id, device) in self.devices.iter_mut() {
            device.update();
        }
    }

    pub fn add_device(&mut self, id: &str, _device: Box<dyn Device>) {
        self.devices.insert(id.to_string(), _device);
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

impl Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sum_devices = self.devices.iter().fold(String::new(), |mut acc, (_id, device)| {
            acc.push_str(device.to_string().as_str());
            acc
        });

        writeln!(
            f,
            "Room id {} name {} devices: \n{}",
            self.id, self.name, sum_devices
        )
    }
}

pub trait Device: Any + Display {
    fn update(&mut self);
    fn report(&self) -> String;
    fn set_state(&mut self, state: SocketState);
    fn get_state(&self) -> Option<SocketState>;
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


impl Device for Thermometer {
    fn update(&mut self) {
        self.update_temperature();
    }

    fn report(&self) -> String {
        format!("id: {}, name: {} ", self.id, self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_state(&mut self, _state: SocketState) {
        
    }
    fn get_state(&self) -> Option<SocketState> {
        None
    }
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

    fn get_state(&self) -> Option<SocketState> {
        Some(self.state)
    }
}

#[derive(Debug)]
pub struct Thermometer {
    id: String,
    name: String,
    temperature: f32,
    min_temperature: f32,
    max_temperature: f32,
}

impl Thermometer {
    pub fn new(_id: &str, _name: &str, _min_temperature: f32, _max_temperature: f32) -> Self {
        Self {
            id: _id.to_string(),
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

    pub fn get_id(&self) -> &str {
        self.id.as_str()
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

// impl From<dyn Device> for Thermometer {
//     fn from(other: &(dyn Device + 'static)) -> Self {
//         Self{
//             id: other.id.clone(),
//             name: other.name.clone(),
//             temperature: other.temperature,
//             max_temperature: other.max_temperature,
//             min_temperature: other.min_temperature,
//         }
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketState {
    On,
    Off,
}

#[derive(Debug)]
pub struct Socket {
    id: String,
    name: String,
    state: SocketState,
    power: f32,
    max_power: f32,
}

impl Socket {
    pub fn new(_id: &str, _name: &str, _state: SocketState, _max_power: f32) -> Self {
        Socket {
            id: _id.to_string(),
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

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
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
    use std::collections::HashMap;
    use crate::{Device, Home, Room, Socket, SocketState, Thermometer};
    #[test]
    fn thermometr_test() {
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
        let devices  = HashMap::<String, Box<dyn Device>>::new();
        let mut room1 = Room::new("1", "Kitchen", devices);
        room1.add_device("1", Box::<Thermometer>::new(term1));
        room1.add_device("2", Box::<Thermometer>::new(term2));
        room1.add_device("3", Box::<Socket>::new(socket1));
        room1.add_device("4", Box::<Socket>::new(socket2));
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
        room1.add_device("1", Box::<Thermometer>::new(term1));
        room1.add_device("2", Box::<Thermometer>::new(term2));
        room1.add_device("3", Box::<Socket>::new(socket1));
        room1.add_device("4", Box::<Socket>::new(socket2));
        let mut room2 = Room::new("2", "Hall", HashMap::<String, Box<dyn Device>>::new());
        
        room2.add_device("1", Box::<Thermometer>::new(term3));
        room2.add_device("2", Box::<Thermometer>::new(term4));
        room2.add_device("3", Box::<Socket>::new(socket3));
        room2.add_device("4", Box::<Socket>::new(socket4));
        
        let mut home = Home::new("1", "My home", HashMap::<String, Box::<Room>>::new());
        home.add_room("1", Box::<Room>::new(room1));
        home.add_room("2", Box::<Room>::new(room2));
        let room = match home.get_mut_room("1") {
            Some(room) => room,
            None => panic!("Room was not found"),
        };
        let device = match room.get_mut_device("3") 
        {
            Some(device) => device,
            None => panic!("Room wasd not found"),
        };
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
        let socket1 = Socket::new("1", "Coffee mashine power socket", SocketState::On, 1000.0);
        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        room1.add_device("1",Box::<Socket>::new(socket1));
        let mut home = Home::new("1", "My home", HashMap::<String, Box::<Room>>::new());
        home.add_room("1", Box::<Room>::new(room1));
        home.get_mut_room("1").expect("Room not found")
            .get_mut_device("1").expect("Device not found")
            .set_state(SocketState::Off);

        let socket_after = home.get_mut_room("1").expect("Room not found")
                               .get_mut_device("1").expect("Device non found");
        match socket_after.get_state() {
            Some(state) => assert!(state == SocketState::Off),
            _ => panic!("Device is not a socket"),
        }
    }

}

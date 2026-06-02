// OTUS Rust hw03 0.3.0
// Implement async network communication with device simulator
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{self, Display};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};

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
        self.state
            .lock()
            .map(|state| *state)
            .unwrap_or_else(|e| {
                eprintln!("Error locking state mutex for get_state: {e}");
                SocketState::Off
            })
            .into()
    }
}

pub struct Thermometer {
    id: String,
    name: String,
    temperature: Arc<Mutex<f32>>,
    min_temperature: f32,
    max_temperature: f32,
    address: String,
    update_interval_ms: u64,
    updater: std::thread::JoinHandle<()>,
    is_updater_running: Arc<Mutex<bool>>,
}

impl Thermometer {
    pub fn new(
        _id: &str,
        _name: &str,
        _min_temperature: f32,
        _max_temperature: f32,
        _address_port: &str,
        _update_interval_ms: u64,
    ) -> Self {
        let _temperature = Arc::new(Mutex::new(0.0));
        let cloned_temperature = _temperature.clone();
        let _connection = match UdpSocket::bind(_address_port) {
            Ok(conn) => conn,
            Err(e) => panic!("Failed to bind UDP socket: {e}"),
        };
        let cloned_update_interval_ms = _update_interval_ms;
        let _is_updater_running = Arc::new(Mutex::new(true));
        let cloned_is_updater_running = _is_updater_running.clone();
        let cloned_id = _id.to_string();
        let cloned_name = _name.to_string();
        let _updater = {
            std::thread::spawn(move || {
                loop {
                    if !*cloned_is_updater_running.lock().unwrap() {
                        println!(
                            "Updater thread thermometer {} {} is stopping",
                            cloned_id, cloned_name
                        );
                        break;
                    }
                    let mut buffer = [0; 1024];
                    match _connection.recv(&mut buffer) {
                        Ok(size) => match std::str::from_utf8(&buffer[..size]) {
                            Ok(temp_str) => match temp_str.trim().parse::<f32>() {
                                Ok(temp) => {
                                    println!("Received temperature: {temp}");
                                    let mut temp_lock = cloned_temperature.lock().unwrap();
                                    *temp_lock = temp;
                                    std::thread::sleep(std::time::Duration::from_millis(
                                        cloned_update_interval_ms,
                                    ));
                                }
                                Err(e) => eprintln!("Error parsing temperature: {e}"),
                            },
                            Err(e) => eprintln!("Error parsing temperature: {e}"),
                        },
                        Err(e) => eprintln!("Error receiving temperature: {e}"),
                    }
                }
            })
        };

        Self {
            id: _id.to_string(),
            name: _name.to_string(),
            temperature: _temperature,
            min_temperature: _min_temperature,
            max_temperature: _max_temperature,
            address: _address_port.to_string(),
            update_interval_ms: _update_interval_ms,
            updater: _updater,
            is_updater_running: _is_updater_running,
        }
    }

    fn update_temperature(&mut self) {
        let temp_lock = self.temperature.try_lock();
        if self.updater.is_finished() {
            let random_temp = rand::random_range(self.min_temperature..self.max_temperature);
            temp_lock
                .map(|mut temp| *temp = random_temp)
                .unwrap_or_else(|e| eprintln!("Error locking temperature mutex: {e}"));
        }
    }

    pub fn get_temperature(&self) -> f32 {
        // self.update_temperature();
        self.temperature
            .lock()
            .map(|temp| *temp)
            .unwrap_or_else(|e| {
                eprintln!("Error locking temperature mutex: {e}");
                0.0
            })
    }
}

impl Drop for Thermometer {
    fn drop(&mut self) {
        if !self.updater.is_finished() {
            println!(
                "Dropping thermometer, stopping updater thread... {} {}",
                self.id, self.name
            );
            self.is_updater_running
                .lock()
                .map(|mut running| *running = false)
                .unwrap_or_else(|e| eprintln!("Error locking is_updater_running mutex: {e}"));
        }
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
            "Thermometer id {}, name {}, address {}, update interval {}, temperature {}",
            self.id,
            self.name,
            self.address,
            self.update_interval_ms,
            self.get_temperature()
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
    state: Arc<Mutex<SocketState>>,
    power: Arc<Mutex<f32>>,
    max_power: f32,
    address: String,
    connection: TcpStream,
}

impl Socket {
    pub fn new(
        _id: &str,
        _name: &str,
        _state: SocketState,
        _max_power: f32,
        _address: &str,
    ) -> Self {
        let init_power = Arc::new(Mutex::new(0.0));
        let init_state = Arc::new(Mutex::new(_state));
        let mut _connection = TcpStream::connect(_address).unwrap_or_else(|e| {
            panic!("Failed to connect to socket simulator {_address}: {e}");
        });
        Self {
            id: _id.to_string(),
            name: _name.to_string(),
            state: init_state,
            power: init_power,
            max_power: _max_power,
            address: _address.to_string(),
            connection: _connection,
        }
    }

    pub fn set_state(&mut self, _state: SocketState) {
        println!(
            "Setting state for socket {} {} to {_state:?}",
            self.id, self.name
        );
        match _state {
            SocketState::On => {
                if self.connection.write("SET ON".as_bytes()).is_ok() {
                    let mut response_buffer = [0; 1024];
                    if self.connection.read(&mut response_buffer).is_ok() {
                        let response = String::from_utf8_lossy(&response_buffer)
                            .chars()
                            .take_while(|&c| c != '\0')
                            .collect::<String>();

                        println!("Response from socket simulator: {}", response);
                        match response.as_str() {
                            "OK" => {
                                self.state
                                    .lock()
                                    .map(|mut state| *state = SocketState::On)
                                    .unwrap_or_else(|e| {
                                        eprintln!("Error locking state mutex: {e}");
                                    });
                                println!("Socket {} {} is now ON", self.id, self.name);
                            }
                            _ => {
                                eprintln!(
                                    "Unexpected response from socket simulator: {}",
                                    response
                                );
                            }
                        }
                    } else {
                        eprintln!("Error SET ON");
                    }
                }
            }
            SocketState::Off => {
                if self.connection.write("SET OFF".as_bytes()).is_ok() {
                    let mut response_buffer = [0; 1024];
                    if self.connection.read(&mut response_buffer).is_ok() {
                        let response = String::from_utf8_lossy(&response_buffer)
                            .chars()
                            .take_while(|&c| c != '\0')
                            .collect::<String>();

                        println!("Response from socket simulator: {}", response);
                        match response.as_str() {
                            "OK" => {
                                self.state
                                    .lock()
                                    .map(|mut state| *state = SocketState::Off)
                                    .unwrap_or_else(|e| {
                                        eprintln!("Error locking state mutex: {e}");
                                    });
                                println!("Socket {} {} is now OFF", self.id, self.name);
                            }
                            _ => {
                                eprintln!(
                                    "Unexpected response from socket simulator: {}",
                                    response
                                );
                            }
                        }
                    }
                } else {
                    eprintln!("Error SET OFF");
                }
            }
        };
        // self.update_power();
    }

    pub fn get_state(&mut self) -> Option<SocketState> {
        if self.connection.write("GET STATE".as_bytes()).is_ok() {
            let mut buffer = [0; 1024];
            let buf: &mut [u8; 1024] = &mut buffer;
            match self.connection.read(buf) {
                Ok(size) => match std::str::from_utf8(&buffer[..size]) {
                    Ok(state_str) => match state_str.trim() {
                        "ON" => self
                            .state
                            .lock()
                            .map(|mut state| *state = SocketState::On)
                            .unwrap_or_else(|e| {
                                eprintln!("Error locking state mutex: {e}");
                            }),
                        "OFF" => self
                            .state
                            .lock()
                            .map(|mut state| *state = SocketState::Off)
                            .unwrap_or_else(|e| {
                                eprintln!("Error locking state mutex: {e}");
                            }),
                        _ => {
                            eprintln!("Unknown state received: {state_str}");
                            return None;
                        }
                    },
                    Err(e) => eprintln!("Error parsing state: {e}"),
                },
                Err(e) => eprintln!("Error reading state: {e}"),
            }
        }
        match self.state.lock().map(|state| *state) {
            Ok(state) => Some(state),
            Err(e) => {
                eprintln!("Error locking state mutex for read state {e}");
                None
            }
        }
    }

    pub fn get_power(&mut self) -> f32 {
        self.update_power();
        self.power.lock().map(|power| *power).unwrap_or_else(|e| {
            eprintln!("Error locking power mutex: {e}");
            0.0
        })
    }

    pub fn update_power(&mut self) {
        println!("Updating power for socket {} {}", self.id, self.name);
        if self.connection.write("GET POWER".as_bytes()).is_ok() {
            let mut buffer = [0; 1024];
            if self.connection.read(&mut buffer).is_ok() {
                let recived = String::from_utf8_lossy(&buffer)
                    .chars()
                    .take_while(|&c| c != '\0')
                    .collect::<String>();
                println!("Received power from socket simulator: {}", recived);
                let power = match recived.trim().parse::<f32>() {
                    Ok(power) => power,
                    Err(e) => {
                        eprintln!("Error parsing power: {e}");
                        0.0
                    }
                };

                self.power
                    .lock()
                    .map(|mut p| *p = power)
                    .unwrap_or_else(|e| eprintln!("Error locking power mutex: {e}"));
            } else {
                eprintln!("Error reading power from socket simulator");
            }
        } else {
            let state = self.state.lock().map(|state| *state).unwrap_or_else(|e| {
                eprintln!("Error locking state mutex for update power: {e}");
                SocketState::Off
            });
            let power = match state {
                SocketState::Off => 0.0,
                SocketState::On => rand::random_range(0.0..self.max_power),
            };

            self.power
                .lock()
                .map(|mut p| *p = power)
                .unwrap_or_else(|e| eprintln!("Error locking power mutex: {e}"));
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
            "Socket id {}, name {}, address {}, state {:?},  power {}",
            self.id,
            self.name,
            self.address,
            self.state,
            self.power.lock().map(|p| *p).unwrap_or_else(|_| { 0.0 })
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
    use crate::{Device, Home, Room, Socket, SocketState, Thermometer};
    use std::collections::HashMap;

    static UPDATE_INTERVAL_MS: u64 = 1000_u64;
    #[test]
    fn thermometer_get_temperature_test() {
        let term1 = Thermometer::new(
            "1",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8001",
            UPDATE_INTERVAL_MS,
        );
        std::thread::sleep(std::time::Duration::from_millis(UPDATE_INTERVAL_MS));
        let temp1 = term1.get_temperature();
        std::thread::sleep(std::time::Duration::from_millis(UPDATE_INTERVAL_MS));
        let temp2 = term1.get_temperature();
        println!("Temperature 1: {temp1}, Temperature 2: {temp2}");
        assert!(temp1 != temp2);
    }

    #[test]
    fn pair_thermometer_get_temperature_test() {
        let term2 = Thermometer::new(
            "2",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8002",
            UPDATE_INTERVAL_MS,
        );
        let term3 = Thermometer::new(
            "3",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8003",
            UPDATE_INTERVAL_MS,
        );

        std::thread::sleep(std::time::Duration::from_millis(UPDATE_INTERVAL_MS));
        let temp2 = term2.get_temperature();
        let temp3 = term3.get_temperature();
        println!("Temperature 2: {temp2}, Temperature 3: {temp3}");
        assert!(temp2 != temp3);
    }

    #[test]
    fn socket_test() {
        let mut socket1 = Socket::new(
            "1",
            "Virtual socket",
            SocketState::On,
            1000.0,
            "127.0.0.1:7001",
        );
        let mut socket2 = Socket::new(
            "2",
            "Virtual socket",
            SocketState::Off,
            1000.0,
            "127.0.0.1:7002",
        );
        socket1.set_state(SocketState::On);
        socket2.set_state(SocketState::Off);
        socket1.update_power();
        socket2.update_power();
        assert!(socket1 != socket2);
        assert!(socket2.get_power() == 0.0);
        assert!(socket1.get_power() != 0.0);
    }

    #[test]
    fn room_test() {
        let term1 = Thermometer::new(
            "1",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8101",
            UPDATE_INTERVAL_MS,
        );
        let term2 = Thermometer::new(
            "2",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8102",
            UPDATE_INTERVAL_MS,
        );
        let socket1 = Socket::new(
            "3",
            "Virtual socket",
            SocketState::On,
            1000.0,
            "127.0.0.1:7001",
        );
        let socket2 = Socket::new(
            "4",
            "Virtual socket",
            SocketState::Off,
            1000.0,
            "127.0.0.1:7002",
        );
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
        let term1 = Thermometer::new(
            "1",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8201",
            UPDATE_INTERVAL_MS,
        );
        let term2 = Thermometer::new(
            "2",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8202",
            UPDATE_INTERVAL_MS,
        );

        let socket1 = Socket::new(
            "3",
            "Virtual socket",
            SocketState::On,
            1000.0,
            "127.0.0.1:7201",
        );
        let socket2 = Socket::new(
            "4",
            "Virtual socket",
            SocketState::Off,
            1000.0,
            "127.0.0.1:7202",
        );

        let mut room1 = Room::new("1", "Kitchen", HashMap::<String, Box<dyn Device>>::new());
        let term3 = Thermometer::new(
            "5",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8203",
            UPDATE_INTERVAL_MS,
        );
        let term4 = Thermometer::new(
            "6",
            "Virtual thermometer",
            -50.0,
            50.0,
            "127.0.0.1:8204",
            UPDATE_INTERVAL_MS,
        );
        let socket3 = Socket::new(
            "7",
            "Virtual socket",
            SocketState::On,
            1000.0,
            "127.0.0.1:7203",
        );
        let socket4 = Socket::new(
            "8",
            "Virtual socket",
            SocketState::Off,
            1000.0,
            "127.0.0.1:7204",
        );
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
}

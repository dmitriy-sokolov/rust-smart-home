use std::collections::HashMap;
use string_builder::Builder;

trait Device {
    fn report(&self) -> String;
}

struct PowerSocket {
    // описание
    _description: String,

    //  состояние вкл / выкл
    active: bool,

    // текущее потребление мощности
    value: f32,
}

struct Thermometer {
    // текущая температура
    value: f32,
}

impl Device for Thermometer {
    fn report(&self) -> String {
        format!("температура: {}", self.value)
    }
}

impl Device for PowerSocket {
    fn report(&self) -> String {
        format!(
            "состояние: {}",
            if self.active {
                format!("включено ({} Вт)", self.value)
            } else {
                "выключено".to_string()
            }
        )
    }
}

pub struct Home {
    _name: String,
    rooms: HashMap<String, HashMap<String, Box<dyn Device>>>,
}

struct ReportParam {
    room_name: String,
    device_name: String,
}

trait SmartHome {
    // возвращаем список помещений
    fn rooms_list(&self) -> Vec<String>;

    fn add_room(&mut self, room_name: &str) -> Result<(), String>;

    fn remove_room(&mut self, room_name: &str);

    fn devices_list(&self, room_name: &str) -> Result<Vec<String>, String>;

    fn add_device(
        &mut self,
        room_name: &str,
        device_name: &str,
        device: Box<dyn Device>,
    ) -> Result<(), String>;

    fn remove_device(&mut self, room_name: &str, device_name: &str);

    fn report(&self, filter: &[ReportParam]) -> String;
}

impl SmartHome for Home {
    fn rooms_list(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(self.rooms.len());
        for key in self.rooms.keys() {
            result.push(key.clone());
        }
        result
    }
    fn add_room(&mut self, room_name: &str) -> Result<(), String> {
        if self.rooms.contains_key(room_name) {
            Result::Err(format!("Комната {} уже зарегистрирована", room_name))
        } else {
            self.rooms.insert(room_name.to_string(), HashMap::new());
            Result::Ok(())
        }
    }
    fn remove_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }
    fn devices_list(&self, room_name: &str) -> Result<Vec<String>, String> {
        match self.rooms.get(room_name) {
            None => Result::Err(format!("Комната {} не зарегистрирована", room_name)),
            Some(devices) => {
                let mut result: Vec<String> = Vec::with_capacity(devices.len());
                for key in self.rooms.keys() {
                    result.push(key.clone());
                }
                Result::Ok(result)
            }
        }
    }
    fn add_device(
        &mut self,
        room_name: &str,
        device_name: &str,
        device: Box<dyn Device>,
    ) -> Result<(), String> {
        match self.rooms.get_mut(room_name) {
            None => Result::Err(format!("Комната {} не зарегистрирована", room_name)),
            Some(devices) => match devices.get(device_name) {
                None => {
                    devices.insert(device_name.to_string(), device);
                    Ok(())
                }
                Some(_) => Result::Err(format!(
                    "В комнате {} уже зарегистрировано устройство {}",
                    room_name, device_name
                )),
            },
        }
    }
    fn remove_device(&mut self, room_name: &str, device_name: &str) {
        match self.rooms.get_mut(room_name) {
            None => {}
            Some(devices) => {
                devices.remove(device_name);
            }
        }
    }

    fn report(&self, filter: &[ReportParam]) -> String {
        let mut builder = Builder::default();
        for param in filter {
            builder.append(format!("{}:", param.room_name));
            match self.rooms.get(&param.room_name) {
                None => builder.append(" нет в доме\n"),
                Some(devices) => {
                    builder.append("\n");
                    match devices.get(&param.device_name) {
                        None => {
                            builder.append(format!("- {}: не зарегистрировано", param.device_name))
                        }
                        Some(device) => builder.append(format!(
                            "- {}: {}\n",
                            param.device_name,
                            device.report()
                        )),
                    }
                }
            }
        }
        builder.string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::HashMap;
    use crate::Home;
    use crate::PowerSocket;
    use crate::ReportParam;
    use crate::SmartHome;
    use crate::Thermometer;

    #[test]
    fn it_works() {
        // let result = 2 + 2;
        // assert_eq!(result, 4);
        let power_socket = PowerSocket {
            _description: String::from(""),
            active: false,
            value: 0.0,
        };

        let thermometer = Thermometer { value: 20.0 };

        let mut home = Home {
            _name: String::from("квартира"),
            rooms: HashMap::new(),
        };

        let room = String::from("гостиная");

        home.add_room(&room);
        home.add_device(&room, &String::from("термометр"), Box::new(thermometer));
        home.add_device(&room, &String::from("подсветка"), Box::new(power_socket));

        let filter = [
            ReportParam {
                device_name: String::from("термометр"),
                room_name: String::from("гостиная"),
            },
            ReportParam {
                device_name: String::from("термометр"),
                room_name: String::from("фое"),
            },
        ];

        let report = home.report(&filter);
        println!("ОТЧЕТ \n{}", report);
    }
}

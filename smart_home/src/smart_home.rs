use std::collections::HashMap;

use string_builder::Builder;

use crate::device_info_provider::{DeviceId, DeviceInfoProvider};

pub struct ReportParam {
    pub room_name: String,
    pub device_name: String,
}

pub struct Home {
    _name: String,
    rooms: HashMap<String, HashMap<String, DeviceId>>,
}

impl Home {
    pub fn new(name: String) -> Self {
        Self {
            _name: name,
            rooms: HashMap::new(),
        }
    }
    pub fn get_name(&self) -> String {
        self._name.clone()
    }
    pub fn room_list(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(self.rooms.len());
        for key in self.rooms.keys() {
            result.push(key.clone());
        }
        result
    }
    pub fn room_add(&mut self, room_name: &str) -> Result<(), String> {
        if self.rooms.contains_key(room_name) {
            Result::Err(format!("Комната {} уже зарегистрирована", room_name))
        } else {
            self.rooms.insert(room_name.to_string(), HashMap::new());
            Result::Ok(())
        }
    }
    pub fn room_remove(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }
    pub fn device_list(&self, room_name: &str) -> Result<Vec<String>, String> {
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
    pub fn device_add(
        &mut self,
        room_name: &str,
        device_name: &str,
        device_id: DeviceId,
    ) -> Result<(), String> {
        match self.rooms.get_mut(room_name) {
            None => Result::Err(format!("Комната {} не зарегистрирована", room_name)),
            Some(devices) => match devices.get(device_name) {
                None => {
                    devices.insert(device_name.to_string(), device_id);
                    Ok(())
                }
                Some(_) => Result::Err(format!(
                    "В комнате {} уже зарегистрировано устройство {}",
                    room_name, device_name
                )),
            },
        }
    }
    pub fn device_remove(&mut self, room_name: &str, device_name: &str) {
        match self.rooms.get_mut(room_name) {
            None => {}
            Some(devices) => {
                devices.remove(device_name);
            }
        }
    }

    pub fn report(&self, filter: &[ReportParam], provider: Box<dyn DeviceInfoProvider>) -> String {
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
                        Some(device) => match provider.get_info(device) {
                            None => builder
                                .append(format!("- рассинхронизация данных: устройство с идентификатором {} не зарегистрировано среди {}, но числиться как {}\n", device.1, device.0, param.device_name)),
                            Some(info) => builder.append(format!(
                                "- {}: {}\n",
                                param.device_name,
                                info
                            )),
                        },
                    }
                }
            }
        }
        builder.string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{device_info_provider::DeviceId, smart_home::Home};

    #[test]
    fn create() {
        let source = Home::new("test".into());
        assert!(source.get_name().eq("test"));
    }
    #[test]
    fn room_add_success() {
        let mut source = Home::new("test".into());
        let result = source.room_add("room_name").unwrap();
        assert!(result == ());
    }
    #[test]
    fn room_add_fail() {
        let mut source = Home::new("test".into());
        source.room_add("room_name").unwrap();
        let result = source.room_add("room_name");
        assert!(result != Ok(()));
    }
    #[test]
    fn room_list() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source.room_add("гостиная").unwrap();
        source.room_add("детская").unwrap();
        source.room_add("спальня").unwrap();
        source.room_add("гардероб").unwrap();
        source.room_add("кладовка").unwrap();
        source.room_add("спорт зал").unwrap();
        let result = source.room_list();
        assert!(result.len() == 7);
    }
    #[test]
    fn room_remove() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source.room_remove("кухня");
        let result = source.room_list();
        assert!(result.len() == 0);
    }
    #[test]
    fn room_remove_skip() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source.room_remove("гардероб");
        let result = source.room_list();
        assert!(result.len() == 1);
    }
    #[test]
    fn device_add() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source
            .device_add("кухня", "device", DeviceId("type".into(), 0))
            .unwrap();
        let result = source.device_list("кухня").unwrap();
        assert!(result.len() == 1);
    }
    #[test]
    fn device_add_fail() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        let result = source.device_add("гардероб", "device", DeviceId("type".into(), 0));
        assert!(result != Ok(()));
    }
    #[test]
    fn device_add_duplicate() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source
            .device_add("кухня", "device", DeviceId("type".into(), 0))
            .unwrap();
        let result = source.device_add("кухня", "device", DeviceId("type".into(), 1));
        assert!(result != Ok(()));
    }
    #[test]
    fn device_list() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        let result = source.device_list("кухня").unwrap();
        assert!(result.len() == 1);
    }
    #[test]
    fn device_list_with_fake_room() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        if let Result::Ok(_) = source.device_list("гардероб") {
            assert!(false)
        }
    }
    #[test]
    fn device_remove() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source
            .device_add("кухня", "device", DeviceId("type".into(), 0))
            .unwrap();
        source.device_remove("кухня", "device");
        let result = source.device_list("кухня").unwrap();
        assert!(result.len() == 1);
    }
}

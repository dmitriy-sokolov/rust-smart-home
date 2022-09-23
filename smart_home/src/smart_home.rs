use std::collections::HashMap;

use string_builder::Builder;

use crate::{
    device_info_provider::{DeviceId, DeviceInfoProvider},
    error::{DeviceError, RoomError},
};

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
    pub fn room_add(&mut self, room_name: &str) -> Result<(), RoomError> {
        if self.rooms.contains_key(room_name) {
            Result::Err(RoomError::Duplicated(room_name.to_string()))
        } else {
            self.rooms.insert(room_name.to_string(), HashMap::new());
            Result::Ok(())
        }
    }
    pub fn room_remove(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }
    pub fn device_list(&self, room_name: &str) -> Result<Vec<String>, RoomError> {
        match self.rooms.get(room_name) {
            None => Result::Err(RoomError::NoRoom(room_name.to_string())),
            Some(devices) => {
                let mut result: Vec<String> = Vec::with_capacity(devices.len());
                for key in devices.keys() {
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
    ) -> Result<(), DeviceError> {
        match self.rooms.get_mut(room_name) {
            None => Result::Err(DeviceError::NoRoom(room_name.to_string())),
            Some(devices) => match devices.get(device_name) {
                None => {
                    devices.insert(device_name.to_string(), device_id);
                    Ok(())
                }
                Some(_) => Result::Err(DeviceError::DeviceDuplicated(
                    room_name.to_string(),
                    device_name.to_string(),
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
    use crate::{
        device_info_provider::DeviceId,
        error::{DeviceError, RoomError},
        smart_home::Home,
    };

    #[test]
    fn create() {
        let source = Home::new("test".into());
        assert!(source.get_name().eq("test"));
    }
    #[test]
    fn room_add_success() {
        let mut source = Home::new("test".into());
        let result = source.room_add("room_name");
        match result {
            Result::Ok(_) => {}
            _ => panic!(),
        }
    }
    #[test]
    fn room_add_fail() {
        let mut source = Home::new("test".into());
        source.room_add("room_name").unwrap();
        match source.room_add("room_name") {
            Result::Err(RoomError::Duplicated(_)) => {}
            _ => panic!(),
        };
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
        assert!(result.is_empty());
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
        match result {
            Result::Err(DeviceError::NoRoom(_)) => {}
            _ => panic!(),
        }
    }
    #[test]
    fn device_add_duplicate() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source
            .device_add("кухня", "device", DeviceId("type".into(), 0))
            .unwrap();
        let result = source.device_add("кухня", "device", DeviceId("type".into(), 1));
        match result {
            Result::Err(DeviceError::DeviceDuplicated(_, _)) => {}
            _ => panic!(),
        }
    }
    #[test]
    fn device_list() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        source
            .device_add("кухня", "device", DeviceId("type".into(), 0))
            .unwrap();
        let result = source.device_list("кухня").unwrap();
        assert!(result.len() == 1);
    }
    #[test]
    fn device_list_with_fake_room() {
        let mut source = Home::new("test".into());
        source.room_add("кухня").unwrap();
        if source.device_list("гардероб").is_ok() {
            panic!()
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
        assert!(result.len() == 0);
    }
}

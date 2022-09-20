use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Комната {0} не зарегистрирована")]
    NoRoom(String),
    #[error("В комнате {0} уже зарегистрировано устройство {1}")]
    DeviceDuplicated(String, String),
}

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum RoomError {
    #[error("Комната {0} уже зарегистрирована")]
    Duplicated(String),
    #[error("Комната {0} не зарегистрирована")]
    NoRoom(String),
}

/**
 * Идентификатор устройства состоит из
 * 0 - идентификатора провайдера;
 * 1 - номер устройства
 */
pub struct DeviceId(pub String, pub u8);

pub trait DeviceInfoProvider {
    fn get_info(&self, device_id: &DeviceId) -> Option<String>;
}

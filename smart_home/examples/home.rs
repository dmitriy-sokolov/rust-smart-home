use power_socket::PowerSocket;
use provider::Provider;
use smart_home::device_info_provider::{DeviceId, DeviceInfoProvider};
use smart_home::smart_home::{Home, ReportParam};
use thermometer::Thermometer;

struct SmartHomeDeviceProvider {
    provider_power_socket: Provider<PowerSocket>,
    provider_thermometer: Provider<Thermometer>,
}

impl DeviceInfoProvider for SmartHomeDeviceProvider {
    fn get_info(&self, device_id: &DeviceId) -> Option<String> {
        if device_id.0.eq(self.provider_power_socket.get_name()) {
            let res = self.provider_power_socket.get(&device_id.1)?;
            Some(format!("{}", res))
        } else if device_id.0.eq(self.provider_thermometer.get_name()) {
            let res = self.provider_thermometer.get(&device_id.1)?;
            Some(format!("{}", res))
        } else {
            None
        }
    }
}

fn main() {
    let group_thermometer_name = "thermometer";
    let group_power_socket_name = "power_socket";

    let mut provider = SmartHomeDeviceProvider {
        provider_power_socket: Provider::new(group_power_socket_name.into()),
        provider_thermometer: Provider::new(group_thermometer_name.into()),
    };

    let power_socket = PowerSocket::default();

    let thermometer = Thermometer::default();

    let power_socket_info = DeviceId(
        group_power_socket_name.into(),
        provider.provider_power_socket.add(power_socket),
    );

    let thermometer_info = DeviceId(
        group_power_socket_name.into(),
        provider.provider_thermometer.add(thermometer),
    );

    let mut home = Home::new("Квартира".into());
    home.room_add("гостиная").unwrap();
    home.device_add("гостиная", "термометр", thermometer_info)
        .unwrap();
    home.device_add("гостиная", "подсветка", power_socket_info)
        .unwrap();

    let filter = [
        ReportParam {
            device_name: "термометр".to_string(),
            room_name: "гостиная".to_string(),
        },
        ReportParam {
            device_name: "термометр".to_string(),
            room_name: "фое".to_string(),
        },
    ];

    let report = home.report(&filter, Box::new(provider));
    println!("ОТЧЕТ \n{}", report);
}

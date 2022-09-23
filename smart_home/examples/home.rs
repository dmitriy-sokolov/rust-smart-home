use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use power_socket::PowerSocket;
use provider::Provider;
use serde::{Deserialize, Serialize};
use smart_home::device_info_provider::{DeviceId, DeviceInfoProvider};
use smart_home::smart_home::Home;
use thermometer::Thermometer;
use tokio::sync::Mutex;

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

pub struct Shared {
    home: Home,
    provider: SmartHomeDeviceProvider,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Device {
    name: String,
    kind: u8, // 0 - none, 1 - Thermometer , 2 - PowerSocket
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
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

        App::new()
            .app_data(Data::new(Mutex::new(Shared { home, provider })))
            .service(web::resource("/home").route(web::get().to(web_home::detail)))
            .service(web::resource("/home/rooms").route(web::get().to(web_home::rooms)))
            .service(
                web::resource("/home/rooms/{room_name}").route(web::post().to(web_home::room_add)),
            )
            .service(
                web::resource("/home/rooms/{room_name}")
                    .route(web::delete().to(web_home::room_remove)),
            )
            .service(
                web::resource("/home/rooms/{room_name}/devices")
                    .route(web::get().to(web_home::devices)),
            )
            .service(
                web::resource("/home/rooms/{room_name}/devices")
                    .route(web::post().to(web_home::device_add)),
            )
            .service(
                web::resource("/home/rooms/{room_name}/devices/{device_name}")
                    .route(web::delete().to(web_home::device_remove)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub mod web_home {
    use actix_web::{web::Data, HttpResponse};

    use super::*;

    pub async fn detail(data: Data<Mutex<Shared>>) -> HttpResponse {
        let shared = data.lock().await;
        HttpResponse::Ok().json(shared.home.get_name())
    }

    pub async fn rooms(data: Data<Mutex<Shared>>) -> HttpResponse {
        let shared = data.lock().await;
        HttpResponse::Ok().json(shared.home.room_list())
    }

    pub async fn room_add(data: Data<Mutex<Shared>>, room_name: web::Path<String>) -> HttpResponse {
        let mut shared = data.lock().await;
        match shared.home.room_add(&room_name) {
            Ok(()) => HttpResponse::Ok().json(room_name.to_string()),
            Err(err) => HttpResponse::InternalServerError().json(err),
        }
    }

    pub async fn room_remove(
        data: Data<Mutex<Shared>>,
        room_name: web::Path<String>,
    ) -> HttpResponse {
        let mut shared = data.lock().await;
        shared.home.room_remove(room_name.into_inner().as_str());
        HttpResponse::Ok().json(())
    }

    pub async fn devices(data: Data<Mutex<Shared>>, room_name: web::Path<String>) -> HttpResponse {
        let shared = data.lock().await;
        HttpResponse::Ok().json(shared.home.device_list(room_name.into_inner().as_str()))
    }

    pub async fn device_add(
        data: Data<Mutex<Shared>>,
        room_name: web::Path<String>,
        source: web::Json<Device>,
    ) -> HttpResponse {
        let mut shared = data.lock().await;
        let device = source.into_inner();
        let id = match device.kind {
            1 => {
                let thermometer = Thermometer::default();
                Some(DeviceId(
                    shared.provider.provider_thermometer.get_name().to_string(),
                    shared.provider.provider_thermometer.add(thermometer),
                ))
            }
            2 => {
                let power_socket = PowerSocket::default();
                Some(DeviceId(
                    shared.provider.provider_power_socket.get_name().to_string(),
                    shared.provider.provider_power_socket.add(power_socket),
                ))
            }
            _ => None,
        };

        if let Some(device_id) = id {
            match shared.home.device_add(&room_name, &device.name, device_id) {
                Ok(()) => HttpResponse::Ok().json(()),
                Err(_err) => HttpResponse::InternalServerError()
                    .json("There were problems when adding the device"),
            }
        } else {
            HttpResponse::InternalServerError().json("There were problems when adding the device")
        }
    }

    pub async fn device_remove(
        data: Data<Mutex<Shared>>,
        param: web::Path<(String, String)>,
    ) -> HttpResponse {
        let mut shared = data.lock().await;
        shared.home.device_remove(&param.0, &param.1);
        HttpResponse::Ok().json(param.1.to_string())
    }
}

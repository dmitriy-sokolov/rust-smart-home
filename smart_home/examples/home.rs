use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use power_socket::PowerSocket;
use provider::Provider;
use smart_home::device_info_provider::{DeviceId, DeviceInfoProvider};
use smart_home::smart_home::Home;
use std::sync::Arc;
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

#[derive(Clone)]
pub struct Shared {
    home: Arc<Home>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    // let home = Arc::new(Mutex::new(home));
    let data = Shared { home: Arc::new(home) };
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new( data))
            .service(web::resource("/home").route(web::get().to(web_home::detail)))
        // .service(web::resource("/home/rooms").route(web::get().to(web_home::rooms)))
        // .service(web::resource("/home/rooms").route(web::post().to(web_home::room_add)))
        // .service(
        //     web::resource("/home/rooms/{room_name}")
        //         .route(web::delete().to(web_home::room_remove)),
        // )
        // .service(
        //     web::resource("/home/rooms/{room_name}/devices")
        //         .route(web::get().to(web_home::devices)),
        // )
        // .service(
        //     web::resource("/home/rooms/{room_name}/devices")
        //         .route(web::post().to(web_home::device_add)),
        // )
        // .service(
        //     web::resource("/home/rooms/{room_name}/devices/{device_name}")
        //         .route(web::delete().to(web_home::device_remove)),
        // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub mod web_home {
    use actix_web::{web::Data, HttpResponse};

    use super::*;

    pub async fn detail(_home: Data<Shared>) -> HttpResponse {
        HttpResponse::Ok().json("test")
    }

    // pub async fn detail(home: Data<Mutex<Home>>) -> HttpResponse {
    //     let home = home.lock().await;
    //     HttpResponse::Ok().json(home.get_name())
    // }

    // pub async fn rooms(home: Data<Mutex<Home>>) -> HttpResponse {
    //     let home = home.lock().await;
    //     HttpResponse::Ok().json(home.room_list())
    // }

    // pub async fn room_add(home: Data<Mutex<Home>>, room_name: web::Json<String>) -> HttpResponse {
    //     let home = home.lock().unwrap();
    //     match home.room_add(&room_name.into_inner()) {
    //         Ok(()) => HttpResponse::Ok().json(()),
    //         Err(err) => HttpResponse::InternalServerError().json(err),
    //     }
    // }

    // pub async fn room_remove(home: Data<Mutex<Home>>, room_name: web::Path<String>) -> HttpResponse {
    //     let home = home.lock().unwrap();
    //     home.room_remove(room_name.into_inner().as_str());
    //     HttpResponse::Ok().json(())
    // }

    // pub async fn devices(home: Data<Mutex<Home>>, room_name: web::Path<String>) -> HttpResponse {
    //     HttpResponse::Ok().json(home.device_list(room_name.into_inner().as_str()))
    // }

    // pub async fn device_add(home: Data<Mutex<Home>>, room_name: web::Json<String>) -> HttpResponse {
    //     match home.device_add(&room_name.into_inner()) {
    //         Ok(()) => HttpResponse::Ok().json(()),
    //         Err(err) => HttpResponse::InternalServerError().json(err),
    //     }
    // }

    // pub async fn device_remove(home: Data<Mutex<Home>>, room_name: web::Path<String>, device_name: web::Path<String>) -> HttpResponse {
    //     home.device_remove(room_name.into_inner().as_str(), device_name.into_inner().as_str());
    //     HttpResponse::Ok().json(())
    // }
}

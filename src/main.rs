struct PowerSocket {
    // описание
    description: String,

    //  состояние вкл / выкл
    active: bool,

    // текущее потребление мощности
    value: f32,
}

struct _Thermometer {
    // текущая температура
    value: f32,
}

pub trait Active {
    fn on(&mut self);
    fn off(&mut self);
}

impl Active for PowerSocket {
    fn on(&mut self) {
        self.active = true;
        self.value = 220.0;
    }

    fn off(&mut self) {
        self.active = false;
        self.value = 0.0;
    }
}

fn main() {
    let power_socket = &mut PowerSocket {
        description: String::from("testing"),
        active: false,
        value: 0.0,
    };
    println!(
        "Start: \n  active is {}\n  description is {}\n  power is {}",
        power_socket.active, power_socket.description, power_socket.value
    );
    power_socket.on();
    println!(
        "After on: \n  active is {}\n  description is {}\n  power is {}",
        power_socket.active, power_socket.description, power_socket.value
    );
}

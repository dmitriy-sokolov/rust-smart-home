use std::fmt::Display;

#[derive(Default)]
pub struct PowerSocket {
    //  состояние вкл / выкл
    active: bool,

    // текущее потребление мощности
    value: f32,
}

impl PowerSocket {
    pub fn power(&self) -> f32 {
        self.value
    }

    pub fn turn_on(&mut self) {
        self.active = true;
        self.value = 700.0
    }

    pub fn turn_off(&mut self) {
        self.active = false;
        self.value = 0.0;
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

impl Display for PowerSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "состояние: {}",
            if self.active {
                format!("включено ({} Вт)", self.value)
            } else {
                "выключено".to_string()
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::PowerSocket;

    #[test]
    fn default() {
        let source = PowerSocket::default();
        assert!(!source.active());
        assert!(source.power() == 0.0);
    }

    #[test]
    fn turn_on() {
        let mut source = PowerSocket::default();
        source.turn_on();

        assert!(source.active());
    }

    #[test]
    fn turn_off() {
        let mut source = PowerSocket::default();
        source.turn_on();
        source.turn_off();

        assert!(!source.active());
    }
}

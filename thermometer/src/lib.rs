use std::fmt::Display;

#[derive(Default)]
pub struct Thermometer {
    // текущая температура
    value: f32,
}

impl Thermometer {
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "температура: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::Thermometer;

    #[test]
    fn default() {
        let source = Thermometer::default();
        assert!(source.value() == 0.0);
    }
}

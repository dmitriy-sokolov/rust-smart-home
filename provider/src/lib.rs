use std::collections::HashMap;

pub struct Provider<T> {
    common_counter: u8,
    _name: String,
    store: HashMap<u8, T>,
}

impl<T> Provider<T> {
    pub fn new(name: String) -> Self {
        Self {
            common_counter: 0,
            _name: name,
            store: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self._name
    }

    pub fn add(&mut self, item: T) -> u8 {
        let id = self.common_counter;
        self.store.insert(id, item);
        self.common_counter += 1;
        id
    }

    pub fn get(&self, id: &u8) -> Option<&T> {
        self.store.get(id)
    }

    pub fn remove(&mut self, id: &u8) {
        self.store.remove(id);
    }
}

#[cfg(test)]
mod tests {
    use super::Provider;

    #[test]
    fn create() {
        let source = Provider::<String>::new(String::from("test"));
        let result = source.get_name();
        assert!(result.eq("test"));
    }

    #[test]
    fn add_first() {
        let mut source = Provider::<String>::new(String::from("test"));
        let param = "value".into();

        let result = source.add(param);
        assert!(result == 0);
    }

    #[test]
    fn get() {
        let mut source = Provider::<String>::new(String::from("test"));
        let param = "value".into();

        let idx = source.add(param);

        let result = source.get(&idx).unwrap();
        assert!(result.eq("value"));
    }

    #[test]
    fn remove() {
        let mut source = Provider::<String>::new(String::from("test"));
        let param = "value".into();

        let idx = source.add(param);

        source.remove(&idx);
        let result = source.get(&idx);
        assert!(result == None);
    }
}

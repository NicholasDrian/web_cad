use std::{cell::RefCell, collections::HashMap};

pub struct Stats {
    map: RefCell<HashMap<&'static str, f64>>,
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.map.borrow().iter() {
            write!(f, "{}: {:.1}\n", key, value)?;
        }
        Ok(())
    }
}

impl Stats {
    pub fn new() -> Self {
        Self {
            map: RefCell::new(HashMap::new()),
        }
    }
    pub fn add(&self, key: &'static str, val: f64) {
        self.map
            .borrow_mut()
            .entry(key)
            .and_modify(|count| *count += val)
            .or_insert(val);
    }
}

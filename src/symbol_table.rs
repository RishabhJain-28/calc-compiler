type SymbolEntry = (String, f64);
#[derive(Debug)]

pub struct SymbolTable {
    entities: Vec<SymbolEntry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        return SymbolTable {
            entities: Vec::new(),
        };
    }

    //imporve
    pub fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        if self
            .entities
            .iter()
            .find(|val| val.0 == identifier)
            .is_some()
        {
            Err(format!(
                "Error: Identifier '{}' declared several times.",
                identifier
            ))
        } else {
            self.entities.push((identifier.to_string(), 0.0));
            Ok(self.entities.len() - 1)
        }
    }
    //imporve

    pub fn find_symbol(&self, identifier: &str) -> Result<usize, String> {
        if let Some(pos) = self.entities.iter().position(|val| val.0 == identifier) {
            Ok(pos)
        } else {
            Err(format!(
                "Error: Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
    pub fn get_value(&self, handle: usize) -> f64 {
        self.entities[handle].1
    }
    pub fn set_value(&mut self, handle: usize, value: f64) {
        self.entities[handle].1 = value;
    }
    pub fn iter(&self) -> std::slice::Iter<SymbolEntry> {
        self.entities.iter()
    }
    pub fn get_name(&self, handle: usize) -> String {
        self.entities[handle].0.clone()
    }
}

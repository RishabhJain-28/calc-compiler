#[derive(Debug)]
pub struct SymbolTable {
    entities: Vec<(String, f64)>,
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
}

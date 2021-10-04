//! Generate unique identifiers and give them human-readable prefixes.

#[derive(Debug)]
pub struct IdGenerator {
    serial_number: u64,
}
impl IdGenerator {
    pub fn new(starting_serial_number: u64) -> Self {
        Self {
            serial_number: starting_serial_number,
        }
    }
    fn generate_raw(&mut self) -> u64 {
        let current = self.serial_number;
        self.serial_number += 1;
        current
    }
    pub fn generate_for_hero(&mut self) -> String {
        format!("hero-{}", &self.generate_raw())
    }
    pub fn generate_for_wall(&mut self) -> String {
        format!("wall-{}", &self.generate_raw())
    }
    pub fn generate_for_field_effect(&mut self) -> String {
        format!("fe-{}", &self.generate_raw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut id_generator = IdGenerator::new(1);
        assert_eq!(id_generator.generate_for_hero(), String::from("hero-1"));
        assert_eq!(id_generator.generate_for_wall(), String::from("wall-2"));
        assert_eq!(id_generator.generate_for_field_effect(), String::from("fe-3"));
    }
}

use std::collections::HashMap;
use scada_core::{ScadaError, ScadaResult};

pub struct PlcSimulator {

    pub registers: HashMap<u16, u16>,
    pub device_id: u8 
}

impl PlcSimulator{
    pub fn new(device_id:u8) -> Self {
        let mut registers = HashMap::new();
        registers.insert(40001, 850);
        registers.insert(40002, 10 );
        registers.insert(40003, 250);

        Self { device_id, registers }

    }

    pub fn read_register(&self, address: u16) -> ScadaResult<u16> {
        match self.registers.get(&address){
            Some(value) => Ok(*value),
            None => Err(ScadaError::InvalidAddress(address)),
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn read_existing_registe(){
        let sim = PlcSimulator::new(1);
        let result = sim.read_register(40001);
        assert_eq!(result.unwrap(), 850);
    }

    #[test]
    fn read_invalid_register(){
        let sim = PlcSimulator::new(2);
        let result = sim.read_register(65535);
        assert!(result.is_err());
    }
}
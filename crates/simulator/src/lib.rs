use modbus::ModbusFrame;
use rand::RngExt;
use scada_core::{ScadaError, ScadaResult};
use std::collections::HashMap;

pub struct PlcSimulator {
    pub registers: HashMap<u16, u16>,
    pub device_id: u8,
}

impl PlcSimulator {
    pub fn new(device_id: u8) -> Self {
        let mut registers = HashMap::new();
        registers.insert(40001, 850);
        registers.insert(40002, 10);
        registers.insert(40003, 250);

        Self {
            device_id,
            registers,
        }
    }

    pub fn read_register(&self, address: u16) -> ScadaResult<u16> {
        match self.registers.get(&address) {
            Some(value) => Ok(*value),
            None => Err(ScadaError::InvalidAddress(address)),
        }
    }

    pub fn process_request(&self, frame: &ModbusFrame) -> ScadaResult<u16> {
        if frame.data.len() < 2 {
            return Err(ScadaError::ParseError("data trop court".to_string()));
        }

        if frame.function_code != 3 {
            return Err(ScadaError::UnsupportedFunctionCode(frame.function_code));
        }

        let address = ((frame.data[0] as u16) << 8) | (frame.data[1] as u16);

        self.read_register(address)
    }

    pub fn update_registers(&mut self) {
        let mut rng = rand::rng();
        let variation: i32 = rng.random_range(-10..=10);

        if let Some(val) = self.registers.get_mut(&40001) {
            *val = ((*val as i32) + variation) as u16;
        }
        if let Some(val) = self.registers.get_mut(&40002) {
            *val = ((*val as i32) + variation) as u16;
        }
        if let Some(val) = self.registers.get_mut(&40003) {
            *val = ((*val as i32) + variation) as u16;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use modbus::parse_frame;

    #[test]
    fn read_existing_registe() {
        let sim = PlcSimulator::new(1);
        let result = sim.read_register(40001);
        assert_eq!(result.unwrap(), 850);
    }

    #[test]
    fn read_invalid_register() {
        let sim = PlcSimulator::new(2);
        let result = sim.read_register(65535);
        assert!(result.is_err());
    }

    #[test]
    fn process_valid_request() {
        let raw: &[u8] = &[
            0x00, 0x01, //transaction_id = 1
            0x00, 0x00, // protocol_id = 0
            0x01, 0x06, // length = 6
            0x01, // unit_id = 1
            0x03, // function_code =3
            0x9c, 0x41, // adresse 40001
            0x00, 0x01, // nombre de registres = 1
        ];
        let frame = parse_frame(raw).unwrap();
        let sim = PlcSimulator::new(1);
        let result = sim.process_request(&frame);
        assert_eq!(result.unwrap(), 850);
    }
}

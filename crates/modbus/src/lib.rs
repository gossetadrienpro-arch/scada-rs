use scada_core::{ScadaError, ScadaResult};

pub struct ModbusFrame {
    pub transaction_id: u16,
    pub protocol_id: u16,
    pub length: u16,
    pub unit_id: u8,
    pub function_code: u8,
    pub data: Vec<u8>,
}

pub fn parse_frame(raw: &[u8]) -> ScadaResult<ModbusFrame> {
    if raw.len() < 7 {
        return Err(ScadaError::ParseError("trame trop courte".to_string()));
    }

    let transaction_id = ((raw[0] as u16) << 8) | (raw[1] as u16);
    let protocol_id = ((raw[2] as u16) << 8) | (raw[3] as u16);
    let length = ((raw[4] as u16) << 8) | (raw[5] as u16);
    let unit_id = raw[6];
    let function_code = raw[7];
    let data = raw[8..].to_vec();

    Ok(ModbusFrame {
        transaction_id,
        protocol_id,
        length,
        unit_id,
        function_code,
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_frame() {
        let raw: &[u8] = &[
            0x00, 0x01, // transaction_id = 1
            0x00, 0x00, // protocol_id = 0
            0x00, 0x06, // length = 6
            0x01, // unit_id = 1
            0x03, // function_code = 3 (Read Holding Registers)
            0x9C, 0x41, // adresse registre 40001
            0x00, 0x02, // nombre de registres = 2
        ];

        let frame = parse_frame(raw).unwrap();
        assert_eq!(frame.transaction_id, 1);
        assert_eq!(frame.function_code, 3);
        assert_eq!(frame.unit_id, 1);
    }

    #[test]
    fn parse_too_short() {
        let raw: &[u8] = &[0x00, 0x01, 0x00, 0x00];
        let result = parse_frame(raw);
        assert!(result.is_err());
    }
}

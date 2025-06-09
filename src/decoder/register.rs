use crate::decoder::DecoderError;

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    // 8-bit registers
    AL, // 000
    CL, // 001
    DL, // 010
    BL, // 011
    AH, // 100
    CH, // 101
    DH, // 110
    BH, // 111

    // 16-bit registers
    AX, // 000
    CX, // 001
    DX, // 010
    BX, // 011
    SP, // 100
    BP, // 101
    SI, // 110
    DI, // 111

        // segment registers
        // ES, // 00
        // CS, // 01
        // SS, // 10
        // DS, // 11
}

impl Register {
    pub fn from_u8(value: u8, w: bool) -> Result<Self, DecoderError> {
        let reg_code = value | ((w as u8) << 3);

        match reg_code {
            // 8-bit registers
            0b0000 => Ok(Register::AL),
            0b0001 => Ok(Register::CL),
            0b0010 => Ok(Register::DL),
            0b0011 => Ok(Register::BL),
            0b0100 => Ok(Register::AH),
            0b0101 => Ok(Register::CH),
            0b0110 => Ok(Register::DH),
            0b0111 => Ok(Register::BH),

            // 16-bit registers
            0b1000 => Ok(Register::AX),
            0b1001 => Ok(Register::CX),
            0b1010 => Ok(Register::DX),
            0b1011 => Ok(Register::BX),
            0b1100 => Ok(Register::SP),
            0b1101 => Ok(Register::BP),
            0b1110 => Ok(Register::SI),
            0b1111 => Ok(Register::DI),

            _ => Err(DecoderError::InvalidRegister(value)),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            // 8-bit registers
            Register::AL => 0b0000,
            Register::CL => 0b0001,
            Register::DL => 0b0010,
            Register::BL => 0b0011,
            Register::AH => 0b0100,
            Register::CH => 0b0101,
            Register::DH => 0b0110,
            Register::BH => 0b0111,

            // 16-bit registers
            Register::AX => 0b0000,
            Register::CX => 0b0001,
            Register::DX => 0b0010,
            Register::BX => 0b0011,
            Register::SP => 0b0100,
            Register::BP => 0b0101,
            Register::SI => 0b0110,
            Register::DI => 0b0111,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_from_u8() {
        assert_eq!(Register::from_u8(0b0000, false), Ok(Register::AL));
        assert_eq!(Register::from_u8(0b0001, false), Ok(Register::CL));
        assert_eq!(Register::from_u8(0b0010, false), Ok(Register::DL));
        assert_eq!(Register::from_u8(0b0011, false), Ok(Register::BL));
        assert_eq!(Register::from_u8(0b0100, false), Ok(Register::AH));
        assert_eq!(Register::from_u8(0b0101, false), Ok(Register::CH));
        assert_eq!(Register::from_u8(0b0110, false), Ok(Register::DH));
        assert_eq!(Register::from_u8(0b0111, false), Ok(Register::BH));

        assert_eq!(Register::from_u8(0b1000, true), Ok(Register::AX));
        assert_eq!(Register::from_u8(0b1001, true), Ok(Register::CX));
        assert_eq!(Register::from_u8(0b1010, true), Ok(Register::DX));
        assert_eq!(Register::from_u8(0b1011, true), Ok(Register::BX));
        assert_eq!(Register::from_u8(0b1100, true), Ok(Register::SP));
        assert_eq!(Register::from_u8(0b1101, true), Ok(Register::BP));
        assert_eq!(Register::from_u8(0b1110, true), Ok(Register::SI));
        assert_eq!(Register::from_u8(0b1111, true), Ok(Register::DI));

        assert_eq!(
            Register::from_u8(0b0001_0000, false),
            Err(DecoderError::InvalidRegister(0b0001_0000))
        );
        assert_eq!(
            Register::from_u8(0b0001_0000, true),
            Err(DecoderError::InvalidRegister(0b0001_0000))
        );

        assert_eq!(
            Register::from_u8(0b1111_1111, false),
            Err(DecoderError::InvalidRegister(0b1111_1111))
        );
        assert_eq!(
            Register::from_u8(0b1111_1111, true),
            Err(DecoderError::InvalidRegister(0b1111_1111))
        );
    }

    #[test]
    fn test_register_to_u8() {
        assert_eq!(Register::AL.to_u8(), 0b0000);
        assert_eq!(Register::CL.to_u8(), 0b0001);
        assert_eq!(Register::DL.to_u8(), 0b0010);
        assert_eq!(Register::BL.to_u8(), 0b0011);
        assert_eq!(Register::AH.to_u8(), 0b0100);
        assert_eq!(Register::CH.to_u8(), 0b0101);
        assert_eq!(Register::DH.to_u8(), 0b0110);
        assert_eq!(Register::BH.to_u8(), 0b0111);

        assert_eq!(Register::AX.to_u8(), 0b0000);
        assert_eq!(Register::CX.to_u8(), 0b0001);
        assert_eq!(Register::DX.to_u8(), 0b0010);
        assert_eq!(Register::BX.to_u8(), 0b0011);
        assert_eq!(Register::SP.to_u8(), 0b0100);
        assert_eq!(Register::BP.to_u8(), 0b0101);
        assert_eq!(Register::SI.to_u8(), 0b0110);
        assert_eq!(Register::DI.to_u8(), 0b0111);
    }
}

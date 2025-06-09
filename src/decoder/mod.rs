pub mod instruction;
pub mod opcode;
pub mod register;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DecoderError {
    #[error("Invalid Opcode: {:#010b}", _0)]
    InvalidRegister(u8),
}

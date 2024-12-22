//! Definitions for the headers of messages

use super::ParseMessageError;

/// The magic used to identify the protocol
const MAGIC: [u8; 3] = *b"BJK";

/// The message was sent from the server to the client
const TO_CLIENT_DIRECTION: u8 = 0;

/// The message was sent fron the client to the server
const TO_SERVER_DIRECTION: u8 = 1;

/// The offset of the magic in the header
const MAGIC_OFFSET: usize = 0;

/// The offset of the direction indicator in the header
const DIRECTION_OFFSET: usize = MAGIC_OFFSET + MAGIC.len();

/// The offset of the message tag in the header
const TAG_OFFSET: usize = DIRECTION_OFFSET + 1;

/// The offset of the length of the body in the header
const LEN_OFFSET: usize = TAG_OFFSET + 2;

/// The size of the header
pub const HEADER_SIZE: usize = LEN_OFFSET + 2;

/// Validates the contents of the header, returning `(tag, body_length)`
pub fn validate(header: &[u8], client: bool) -> Result<(u16, u16), ParseMessageError> {
    assert!(header.len() == HEADER_SIZE);

    if header[..MAGIC.len()] != MAGIC {
        return Err(ParseMessageError);
    }

    let desired_direction = if client {
        TO_CLIENT_DIRECTION
    } else {
        TO_SERVER_DIRECTION
    };

    if header[DIRECTION_OFFSET] != desired_direction {
        return Err(ParseMessageError);
    }

    let tag = u16::from_be_bytes([header[TAG_OFFSET], header[TAG_OFFSET + 1]]);
    let body_length = u16::from_be_bytes([header[LEN_OFFSET], header[LEN_OFFSET + 1]]);

    return Ok((tag, body_length));
}

/// Generates the header for the given parameters
pub fn generate(output: &mut [u8], client: bool, tag: u16) {
    assert!(output.len() >= HEADER_SIZE);
    assert!(output.len() <= u16::MAX as usize + HEADER_SIZE);

    let body_length = (output.len() - HEADER_SIZE) as u16;

    output[..MAGIC.len()].copy_from_slice(&MAGIC);
    output[DIRECTION_OFFSET] = if client {
        TO_SERVER_DIRECTION
    } else {
        TO_CLIENT_DIRECTION
    };
    output[TAG_OFFSET..LEN_OFFSET].copy_from_slice(&tag.to_be_bytes());
    output[LEN_OFFSET..HEADER_SIZE].copy_from_slice(&body_length.to_be_bytes());
}

use crate::message::Error::MessageSizeTooLarge;

#[derive(Debug)]
pub enum Error {
    MessageSizeTooLarge(usize)
}

#[derive(Debug)]
pub struct MessageHeader {
    _msg_size: u16,
    _msg_type: u8,
}

pub struct Header;
pub struct FlagBits;

#[derive(Debug)]
pub struct Format {
    pub header: MessageHeader,
    /*
    I would prefer is this were a &'static str since runtime heap allocations are not really great
    for flight controllers, but since this is created during the creation of the logger, which
    should be done before flight, it should be okay.

    It's possible to make this a compile time constant as the derive function should have all the
    information to build it.
     */
    pub format: String,
}

impl Format {
    pub fn new(format: String) -> Result<Self, Error> {
        let msg_size = match format.len() {
            x if x <= u16::MAX as usize => x as u16,
            x => return Err(MessageSizeTooLarge(x)),
        };

        Ok(Format {
            header: MessageHeader {
                _msg_size: msg_size,
                _msg_type: b'F',
            },
            format,
        })
    }
}

pub struct Info;
pub struct InfoMultipleHeader;
pub struct ParameterDefaultHeader;
pub struct AddLogged;
pub struct RemoveLogged;
pub struct Data;
pub struct Logging;
pub struct LoggingTagged;
pub struct Sync;
pub struct Dropout;

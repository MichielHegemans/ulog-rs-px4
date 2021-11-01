use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    MessageSizeTooLarge(usize),
    SerializeError(String),
}

#[derive(Debug, Serialize)]
pub struct MessageHeader {
    _msg_size: u16,
    _msg_type: u8,
}

pub struct Header;
pub struct FlagBits;

#[derive(Debug, Serialize)]
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
            x => return Err(Error::MessageSizeTooLarge(x)),
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

#[derive(Debug, Serialize)]
pub struct Data {
    pub header: MessageHeader,
    pub msg_id: u16,
    pub data: Vec<u8>,
}

impl Data {
    pub fn new(data: Vec<u8>) -> Result<Self, Error> {
        let msg_size = match data.len() {
            x if x <= (u16::MAX - 2) as usize => x as u16,
            x => return Err(Error::MessageSizeTooLarge(x))
        };

        Ok(Self {
            header: MessageHeader {
                _msg_size: msg_size + 2,
                _msg_type: b'D'
            },
            msg_id: 0,
            data,
        })
    }
}

pub struct Logging;
pub struct LoggingTagged;
pub struct Sync;
pub struct Dropout;

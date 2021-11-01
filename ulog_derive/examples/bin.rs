use ulog_derive::Data;
use ulog::Data;
use ulog::message::Format;
use serde::Serialize;

#[derive(Data, Serialize)]
struct Stuff {
    capacity: i32,
    charge: i32,
    nice: u8,
}

fn main() {
    let stuff = Stuff { capacity: 12, charge: 42, nice: 9 };

    println!("{:?}", Stuff::message_format().unwrap());
    println!("{:?}", stuff.message().unwrap())
}

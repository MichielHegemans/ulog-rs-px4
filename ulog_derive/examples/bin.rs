use ulog_derive::Data;
use ulog::Data;
use ulog::message::Format;

#[derive(Data)]
struct Stuff {
    _capacity: i32,
    _charge: i32,
}

fn main() {
    println!("{:?}", Stuff::message_format().unwrap());
}

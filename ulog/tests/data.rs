use serde::Serialize;
use ulog_derive::Data;
use ulog::message::Format;
use ulog::Data;

#[derive(Data, Serialize)]
struct TestData {
    test: u32,
    test2: i32,
}

#[test]
fn message_test() {
    let data = TestData { test: 1, test2: -1 };
    assert_eq!(
        vec![1, 0, 0, 0, 255, 255, 255, 255],
        data.message().unwrap().data,
    );
}

#[test]
fn message_format_test() {
    let format = TestData::message_format().unwrap();
    assert_eq!("testdata:uint32_t test;int32_t test2", format.format);
}

#[test]
fn message_name_test() {
    assert_eq!("testdata", TestData::message_name());
}

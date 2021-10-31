pub mod message;

pub trait Data {
    fn message_name() -> &'static str;
    fn message_format() -> Result<message::Format, message::Error>;
}

pub trait DataType {
    fn type_string() -> &'static str;
}

macro_rules! impl_data_type {
    ($t:ident, $s:expr) => {
        impl DataType for $t {
            fn type_string() -> &'static str {
                $s
            }
        }
    }
}

impl_data_type!(u8, "uint8_t");
impl_data_type!(u16, "uint16_t");
impl_data_type!(u32, "uint32_t");
impl_data_type!(u64, "uint64_t");
impl_data_type!(i8, "int8_t");
impl_data_type!(i16, "int16_t");
impl_data_type!(i32, "int32_t");
impl_data_type!(i64, "int64_t");
impl_data_type!(f32, "float");
impl_data_type!(f64, "double");

/*
According to the spec defined here: https://docs.px4.io/master/en/dev_log/ulog_file_format.html
the message type must be one of the primitives (defined above), or a message name (blanket impl
below)
 */

impl<T: Data> DataType for T {
    fn type_string() -> &'static str {
        T::message_name()
    }
}

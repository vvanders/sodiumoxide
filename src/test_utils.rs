#![cfg(all(test, feature = "default"))]
extern crate serde_json;
extern crate rmp_serde;
extern crate core;
use serde::{Serialize, Deserialize};

// Encodes then decodes `value` using JSON
pub fn round_trip<T>(value: T)
    where T: Serialize + Deserialize + Eq + core::fmt::Debug
{
    let encoded_value = serde_json::to_string(&value).unwrap();
    let decoded_value = serde_json::from_str(&encoded_value).unwrap();
    assert_eq!(value, decoded_value);

    let mut buf = Vec::new();
    value.serialize(&mut rmp_serde::Serializer::new(&mut buf)).unwrap();
    let mut de = rmp_serde::Deserializer::new(&buf[..]);
    let decoded_value = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!(value, decoded_value);
}

use serde::{Deserialize, Deserializer};
use crate::heap_address::HeapAddress;

pub struct DeserializeUtils {}
impl DeserializeUtils {
    pub fn hex_to_heap_address(value: String) -> Option<HeapAddress> {
        let hex_number = value.replace("\"", "").replace("0x", "");

        let uvalue = u64::from_str_radix(hex_number.as_ref(), 16);
        if uvalue.is_err() {
            return None;
        }
        Some(uvalue.unwrap())
    }

    pub fn from_hex_opt<'de, D>(deserializer: D) -> Result<Option<HeapAddress>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = Deserialize::deserialize(deserializer)?;
        Ok(DeserializeUtils::hex_to_heap_address(String::from(value)))
    }

    pub fn from_hex<'de, D>(deserializer: D) -> Result<HeapAddress, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = Deserialize::deserialize(deserializer)?;

        match DeserializeUtils::hex_to_heap_address(String::from(value)) {
            Some(heap_address) => Ok(heap_address),
            None => Ok(0 as HeapAddress)
        }
    }

    pub fn from_hex_array<'de, D>(deserializer: D) -> Result<Vec<HeapAddress>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut heap_addresses : Vec<HeapAddress> = vec!();
        let values: Vec<&str> = Deserialize::deserialize(deserializer)?;

        for value in values {
            let hao = DeserializeUtils::hex_to_heap_address(String::from(value));
            if hao.is_none() { continue; }
            heap_addresses.push(hao.unwrap());
        }
        Ok(heap_addresses)
    }
}
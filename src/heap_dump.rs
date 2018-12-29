use serde_json::{Error, Value};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type HeapAddress = u64;

type HeapObjectFlags = Vec<String>;

struct ConversionUtils {}
impl ConversionUtils {
    pub fn hex_to_heap_address(value: String) -> Option<HeapAddress> {
        let hex_number = value.replace("\"", "").replace("0x", "");

        let uvalue = u64::from_str_radix(hex_number.as_ref(), 16);
        if uvalue.is_err() {
            return None;
        }
        Some(uvalue.unwrap())
    }

    pub fn dejsonify_string(value: String) -> String {
        value.replace("\"", "")
    }

    pub fn references(references: &Vec<Value>) -> Vec<HeapAddress> {
        let mut heap_addresses = Vec::with_capacity(references.len());

        for value in references {
            let address = value.to_string();
            let heap_address = Self::hex_to_heap_address(address);
            if heap_address.is_none() {
                continue;
            }
            heap_addresses.push(heap_address.unwrap());
        }

        heap_addresses
    }

    pub fn from_hex<'de, D>(deserializer: D) -> Result<HeapAddress, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = Deserialize::deserialize(deserializer)?;

        match ConversionUtils::hex_to_heap_address(String::from(value)) {
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
            let hao = ConversionUtils::hex_to_heap_address(String::from(value));
            if hao.is_none() { continue; }
            heap_addresses.push(hao.unwrap());
        }
        Ok(heap_addresses)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct RootObject {
    root: String,
    #[serde(deserialize_with = "ConversionUtils::from_hex_array")]
    references: Vec<HeapAddress>,
}

impl RootObject {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }

    fn from_value(value: Value) -> Self {
        let references_array = value["references"].as_array();

        let references = match references_array {
            Some(refs) => ConversionUtils::references(refs),
            None => vec![],
        };

        RootObject {
            root: ConversionUtils::dejsonify_string(value["root"].to_string()),
            references: references,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct HeapObject {
    address: HeapAddress,
    object_type: String,
    class: Option<HeapAddress>,
    references: Vec<HeapAddress>,
    file: String,
    line: usize,
    method: String,
    generation: usize,
    memsize: usize,
}

impl HeapObject {
    fn from_value(value: Value) -> Option<Self> {
        let address = ConversionUtils::hex_to_heap_address(value["address"].to_string());
        let class = ConversionUtils::hex_to_heap_address(value["class"].to_string());
        let references_array = value["references"].as_array();

        let references = match references_array {
            Some(refs) => ConversionUtils::references(refs),
            None => vec![],
        };

        let heap_object = HeapObject {
            address: address.unwrap(),
            object_type: ConversionUtils::dejsonify_string(value["type"].to_string()),
            class: class,
            references: references,
            file: ConversionUtils::dejsonify_string(value["file"].to_string()),
            line: value["line"].as_u64().unwrap_or_default() as usize,
            method: ConversionUtils::dejsonify_string(value["method"].to_string()),
            generation: value["generation"].as_u64().unwrap_or_default() as usize,
            memsize: value["memsize"].as_u64().unwrap_or_default() as usize,
        };

        Some(heap_object)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct StringObject {
    #[serde(deserialize_with = "ConversionUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "ConversionUtils::from_hex")]
    class: HeapAddress,
    #[serde(default)]
    frozen: bool,
    #[serde(default)]
    fstring: bool,
    bytesize: usize,
    #[serde(default)]
    capacity: usize,
    value: String,
    encoding: String,
    memsize: usize,
    #[serde(default)]
    #[serde(deserialize_with = "ConversionUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    // flags: HashMap<String, bool>
}

impl StringObject {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

pub struct HeapDump {
    root_objects: HashMap<String, RootObject>,
    objects: HashMap<HeapAddress, HeapObject>,
}

impl HeapDump {
    pub fn load_file(file: File) -> Self {
        let mut heap_dump = HeapDump { objects: HashMap::new(), root_objects: HashMap::new() };
        let mut buf_reader = BufReader::new(file);

        for line in buf_reader.lines() { heap_dump.add_line(line.unwrap()); }

        println!("Loaded {} objects", heap_dump.objects.len());
        heap_dump
    }

    pub fn add_line(&mut self, line: String) {
        if line.contains("\"type\":\"ROOT\"") {
            let root_object_result = RootObject::from_str(&line);
            if root_object_result.is_err() {
                println!("Root object err {:?}", root_object_result);
                return;
            }
            let root_object = root_object_result.unwrap();
            self.root_objects.insert(root_object.root.clone(), root_object);
            return
        }
        if line.contains("\"type\":\"STRING\"") {
            let string_object_result = StringObject::from_str(&line);
            if string_object_result.is_err() {
                println!("String object err {:?}", string_object_result);
                return;
            } else {
                println!("String object decoded {:?}", string_object_result);
                return;
            }
        }

        let parse_result: Result<Value, Error> = serde_json::from_str(&line);

        if parse_result.is_err() {
            println!("Error: {:?} for {}", parse_result, line);
            return;
        }
        let value = parse_result.unwrap();
        self.add_value(value);
    }

    pub fn add_value(&mut self, value: Value) {
        let value_type = value["type"].as_str();
        match value_type {
            Some("ROOT") => {
                let root_object = RootObject::from_value(value);
                self.root_objects
                    .insert(root_object.root.clone(), root_object);
            }
            Some(_) => {
                let heap_object_option = HeapObject::from_value(value);
                if heap_object_option.is_none() {
                    return;
                }
                let heap_object = heap_object_option.unwrap();
                self.objects.insert(heap_object.address, heap_object);
            }
            None => {}
        }
    }

    pub fn print_roots(&self) {
        for (_key, root) in &self.root_objects {
            println!("{:?}", root);
            for address in &root.references {
                let heap_object_option = self.objects.get(&address);
                if heap_object_option.is_none() {
                    println!("  Missing heap object {:?}", address);
                    continue;
                }

                println!("  {:?}", heap_object_option.unwrap());
            }
            println!("--")
        }
    }
}

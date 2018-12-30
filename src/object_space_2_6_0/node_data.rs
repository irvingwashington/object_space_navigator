use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

// Wrapped C pointers
#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeData {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_opt")]
    class: Option<HeapAddress>,
    #[serde(rename="struct")]
    struct_type: Option<String>,
    memsize: usize,
    pub flags: Flags,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
    frozen: Option<bool>,
}

impl NodeData {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"DATA", "class":"0x7f80912f5800", "struct":"mutex", "file":"/gems/concurrent-ruby-1.1.3/lib/concurrent/collection/map/mri_map_backend.rb", "line":14, "method":"new", "generation":63, "memsize":72, "flags":{"uncollectible":true, "marked":true}}"#;
    let node_data_res = NodeData::from_str(json_str);
    assert_eq!(node_data_res.is_ok(), true);

    let node_data = node_data_res.unwrap();
    assert_eq!(node_data.address, 140190151992440 as HeapAddress);
    assert_eq!(node_data.class, Some(140190168340480 as HeapAddress));
    assert_eq!(node_data.references, vec![] as Vec<HeapAddress>);

    assert_eq!(node_data.memsize, 72);
    assert_eq!(node_data.struct_type, Some(String::from("mutex")));

    assert_eq!(node_data.file, Some(String::from("/gems/concurrent-ruby-1.1.3/lib/concurrent/collection/map/mri_map_backend.rb")));
    assert_eq!(node_data.line, Some(14 as usize));
    assert_eq!(node_data.method, Some(String::from("new")));
    assert_eq!(node_data.generation, Some(63 as usize));
    assert_eq!(node_data.frozen, None);

    let flags = node_data.flags;
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "memsize":72, "frozen": true, "flags":{}, "references": ["0x7f80912f5800"]}"#;
    let node_data_res = NodeData::from_str(json_str);
    assert_eq!(node_data_res.is_ok(), true);

    let node_data = node_data_res.unwrap();
    assert_eq!(node_data.address, 140190151992440 as HeapAddress);
    assert_eq!(node_data.class, None);
    assert_eq!(node_data.references, vec![140190168340480 as HeapAddress]);

    assert_eq!(node_data.memsize, 72);
    assert_eq!(node_data.struct_type, None);

    assert_eq!(node_data.file, None);
    assert_eq!(node_data.line, None);
    assert_eq!(node_data.method, None);
    assert_eq!(node_data.generation, None);
    assert_eq!(node_data.frozen, Some(true));

    let flags = node_data.flags;
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_data_res = NodeData::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_data_res.is_ok(), false);
  }
}
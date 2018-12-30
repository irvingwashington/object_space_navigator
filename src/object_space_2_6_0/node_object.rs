use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeObject {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    ivars: usize,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    memsize: usize,
    pub flags: Flags,
    frozen: Option<bool>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeObject {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"OBJECT", "class":"0x7f80912f5800", "ivars":3, "references":["0x7f808f8e4970"], "file":"/gems/actionpack-5.2.2/lib/action_dispatch/journey/parser.rb", "line":190, "method":"new", "generation":49, "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_object_res = NodeObject::from_str(json_str);
    assert_eq!(node_object_res.is_ok(), true);

    let node_object = node_object_res.unwrap();
    assert_eq!(node_object.address, 140190151992440 as HeapAddress);
    assert_eq!(node_object.class, 140190168340480 as HeapAddress);
    assert_eq!(node_object.ivars, 3);
    assert_eq!(node_object.references, &[140190141008240 as HeapAddress]);
    assert_eq!(node_object.memsize, 40);

    assert_eq!(node_object.file, Some(String::from("/gems/actionpack-5.2.2/lib/action_dispatch/journey/parser.rb")));
    assert_eq!(node_object.line, Some(190 as usize));
    assert_eq!(node_object.method, Some(String::from("new")));
    assert_eq!(node_object.generation, Some(49 as usize));

    let flags = node_object.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);

    assert_eq!(node_object.frozen, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "class":"0x7f80912f5800", "ivars":3, "memsize":40, "flags":{}, "frozen":true}"#;
    let node_object_res = NodeObject::from_str(json_str);
    assert_eq!(node_object_res.is_ok(), true);

    let node_object = node_object_res.unwrap();
    assert_eq!(node_object.address, 140190151992440 as HeapAddress);
    assert_eq!(node_object.class, 140190168340480 as HeapAddress);

    assert_eq!(node_object.frozen, Some(true));
    assert_eq!(node_object.memsize, 40);
    assert_eq!(node_object.ivars, 3);

    assert_eq!(node_object.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_object.file, None);
    assert_eq!(node_object.line, None);
    assert_eq!(node_object.method, None);
    assert_eq!(node_object.generation, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_object_res = NodeObject::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_object_res.is_ok(), false);
  }
}
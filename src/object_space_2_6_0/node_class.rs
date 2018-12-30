use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeClass {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_opt")]
    class: Option<HeapAddress>,
    name: Option<String>,
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

impl NodeClass {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"CLASS", "class":"0x7f80912f5800", "name":"Class", "references":["0x7fc974138ae0"], "file":"/gems/actionview-5.2.2/lib/action_view/rendering.rb", "line":44, "method":"new", "generation":65, "memsize":1408, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marking":true, "marked":true}}"#;
    let node_class_res = NodeClass::from_str(json_str);
    assert_eq!(node_class_res.is_ok(), true);

    let node_class = node_class_res.unwrap();
    assert_eq!(node_class.address, 140190151992440 as HeapAddress);
    assert_eq!(node_class.class, Some(140190168340480 as HeapAddress));
    assert_eq!(node_class.name, Some(String::from("Class")));
    assert_eq!(node_class.references, &[140503212591840 as HeapAddress]);
    assert_eq!(node_class.memsize, 1408 as usize);

    assert_eq!(node_class.file, Some(String::from("/gems/actionview-5.2.2/lib/action_view/rendering.rb")));
    assert_eq!(node_class.line, Some(44 as usize));
    assert_eq!(node_class.method, Some(String::from("new")));
    assert_eq!(node_class.generation, Some(65 as usize));

    let flags = node_class.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, Some(true));

    assert_eq!(node_class.frozen, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "memsize":1408, "flags":{}, "frozen":true}"#;
    let node_class_res = NodeClass::from_str(json_str);
    assert_eq!(node_class_res.is_ok(), true);

    let node_class = node_class_res.unwrap();
    assert_eq!(node_class.address, 140190151992440 as HeapAddress);
    assert_eq!(node_class.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_class.memsize, 1408 as usize);
    assert_eq!(node_class.frozen, Some(true));

    assert_eq!(node_class.class, None);
    assert_eq!(node_class.name, None);
    assert_eq!(node_class.file, None);
    assert_eq!(node_class.line, None);
    assert_eq!(node_class.method, None);
    assert_eq!(node_class.generation, None);

    let flags = node_class.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_class_res = NodeClass::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_class_res.is_ok(), false);
  }
}

use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeModule {
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

impl NodeModule {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"MODULE", "class":"0x7f80912f5800", "name":"Class", "references":["0x7fc974138ae0"], "file":"/gems/actionview-5.2.2/lib/action_view/rendering.rb", "line":44, "method":"new", "generation":65, "memsize":1408, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marking":true, "marked":true}}"#;
    let node_module_res = NodeModule::from_str(json_str);
    assert_eq!(node_module_res.is_ok(), true);

    let node_module = node_module_res.unwrap();
    assert_eq!(node_module.address, 140190151992440 as HeapAddress);
    assert_eq!(node_module.class, Some(140190168340480 as HeapAddress));
    assert_eq!(node_module.name, Some(String::from("Class")));
    assert_eq!(node_module.references, &[140503212591840 as HeapAddress]);
    assert_eq!(node_module.memsize, 1408 as usize);

    assert_eq!(node_module.file, Some(String::from("/gems/actionview-5.2.2/lib/action_view/rendering.rb")));
    assert_eq!(node_module.line, Some(44 as usize));
    assert_eq!(node_module.method, Some(String::from("new")));
    assert_eq!(node_module.generation, Some(65 as usize));

    let flags = node_module.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, Some(true));

    assert_eq!(node_module.frozen, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "memsize":1408, "flags":{}, "frozen":true}"#;
    let node_module_res = NodeModule::from_str(json_str);
    assert_eq!(node_module_res.is_ok(), true);

    let node_module = node_module_res.unwrap();
    assert_eq!(node_module.address, 140190151992440 as HeapAddress);
    assert_eq!(node_module.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_module.memsize, 1408 as usize);
    assert_eq!(node_module.frozen, Some(true));

    assert_eq!(node_module.class, None);
    assert_eq!(node_module.name, None);
    assert_eq!(node_module.file, None);
    assert_eq!(node_module.line, None);
    assert_eq!(node_module.method, None);
    assert_eq!(node_module.generation, None);

    let flags = node_module.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_module_res = NodeModule::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_module_res.is_ok(), false);
  }
}

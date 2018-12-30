use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeIclass {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    memsize: usize,
    pub flags: Option<Flags>,
    frozen: Option<bool>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeIclass {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"ICLASS", "class":"0x7f80912f5800", "references":["0x7fc974138ae0"], "file":"/gems/activesupport-5.2.2/lib/active_support/concern.rb", "line":120, "method":"append_features", "generation":65, "memsize":40}"#;
    let node_iclass_res = NodeIclass::from_str(json_str);
    assert_eq!(node_iclass_res.is_ok(), true);

    let node_iclass = node_iclass_res.unwrap();
    assert_eq!(node_iclass.address, 140190151992440 as HeapAddress);
    assert_eq!(node_iclass.class, 140190168340480 as HeapAddress);
    assert_eq!(node_iclass.references, &[140503212591840 as HeapAddress]);
    assert_eq!(node_iclass.memsize, 40 as usize);

    assert_eq!(node_iclass.file, Some(String::from("/gems/activesupport-5.2.2/lib/active_support/concern.rb")));
    assert_eq!(node_iclass.line, Some(120 as usize));
    assert_eq!(node_iclass.method, Some(String::from("append_features")));
    assert_eq!(node_iclass.generation, Some(65 as usize));

    let flags_opt = node_iclass.flags;
    assert_eq!(flags_opt, None);
    assert_eq!(node_iclass.frozen, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "class":"0x7f80912f5800", "memsize":1408, "flags":{}, "frozen":true}"#;
    let node_iclass_res = NodeIclass::from_str(json_str);
    assert_eq!(node_iclass_res.is_ok(), true);

    let node_iclass = node_iclass_res.unwrap();
    assert_eq!(node_iclass.address, 140190151992440 as HeapAddress);
    assert_eq!(node_iclass.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_iclass.memsize, 1408 as usize);
    assert_eq!(node_iclass.frozen, Some(true));

    assert_eq!(node_iclass.file, None);
    assert_eq!(node_iclass.line, None);
    assert_eq!(node_iclass.method, None);
    assert_eq!(node_iclass.generation, None);

    let flags_opt = node_iclass.flags;
    assert_eq!(flags_opt.is_some(), true);

    let flags = flags_opt.unwrap();
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_iclass_res = NodeIclass::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_iclass_res.is_ok(), false);
  }
}

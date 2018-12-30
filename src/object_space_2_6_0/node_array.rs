use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeArray {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(default)]
    #[serde(deserialize_with = "DeserializeUtils::from_hex_opt")]
    class: Option<HeapAddress>,
    frozen: Option<bool>,
    length: usize,
    embedded: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    memsize: usize,
    pub flags: Option<Flags>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
    shared: Option<bool>,
}

impl NodeArray {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9748a3c30", "type":"ARRAY", "class":"0x7fc9690af8e0", "length":2, "embedded":true, "references":["0x7fc96937b0d8"], "file":"/gems/i18n-1.1.1/lib/i18n.rb", "line":363, "method":"split", "generation":63, "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_array_res = NodeArray::from_str(json_str);
    assert_eq!(node_array_res.is_ok(), true);

    let node_array = node_array_res.unwrap();
    assert_eq!(node_array.address, 140503220370480 as HeapAddress);
    assert_eq!(node_array.class, Some(140503027480800 as HeapAddress));
    assert_eq!(node_array.length, 2);
    assert_eq!(node_array.embedded, Some(true));
    assert_eq!(node_array.references, &[140503030411480 as HeapAddress]);
    assert_eq!(node_array.file, Some(String::from("/gems/i18n-1.1.1/lib/i18n.rb")));
    assert_eq!(node_array.line, Some(363 as usize));
    assert_eq!(node_array.method, Some(String::from("split")));
    assert_eq!(node_array.generation, Some(63 as usize));
    assert_eq!(node_array.memsize, 40);

    assert_eq!(node_array.frozen, None);
    assert_eq!(node_array.shared, None);

    let flags_option = node_array.flags;
    assert_eq!(flags_option.is_some(), true);

    let flags = flags_option.unwrap();
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9748a3c30", "length":2, "memsize":40, "frozen":true, "shared":true, "flags":{"wb_protected":true}}"#;
    let node_array_res = NodeArray::from_str(json_str);
    assert_eq!(node_array_res.is_ok(), true);

    let node_array = node_array_res.unwrap();
    assert_eq!(node_array.address, 140503220370480 as HeapAddress);
    assert_eq!(node_array.class, None);

    assert_eq!(node_array.frozen, Some(true));
    assert_eq!(node_array.shared, Some(true));
    assert_eq!(node_array.memsize, 40);
    assert_eq!(node_array.length, 2);

    assert_eq!(node_array.embedded, None);
    assert_eq!(node_array.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_array.file, None);
    assert_eq!(node_array.line, None);
    assert_eq!(node_array.method, None);
    assert_eq!(node_array.generation, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_array_res = NodeArray::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_array_res.is_ok(), false);
  }
}
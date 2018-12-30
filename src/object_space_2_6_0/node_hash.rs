use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeHash {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    frozen: Option<bool>,
    size: usize,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_opt")]
    class: Option<HeapAddress>,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_opt")]
    default: Option<HeapAddress>,
    memsize: usize,
    pub flags: Flags,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeHash {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"HASH", "class":"0x7f80912f5800", "size":0, "default":"0x7f80912f5800", "references":["0x7f80912f5800"], "file":"/gems/activesupport-5.2.2/lib/active_support/ordered_options.rb", "line":82, "method":"new", "generation":65, "memsize":232, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_hash_res = NodeHash::from_str(json_str);
    assert_eq!(node_hash_res.is_ok(), true);

    let node_hash = node_hash_res.unwrap();
    assert_eq!(node_hash.address, 140190151992440 as HeapAddress);
    assert_eq!(node_hash.class, Some(140190168340480 as HeapAddress));
    assert_eq!(node_hash.size, 0);
    assert_eq!(node_hash.references, &[140190168340480 as HeapAddress]);
    assert_eq!(node_hash.memsize, 232);
    assert_eq!(node_hash.default, Some(140190168340480 as HeapAddress));

    assert_eq!(node_hash.file, Some(String::from("/gems/activesupport-5.2.2/lib/active_support/ordered_options.rb")));
    assert_eq!(node_hash.line, Some(82 as usize));
    assert_eq!(node_hash.method, Some(String::from("new")));
    assert_eq!(node_hash.generation, Some(65 as usize));

    let flags = node_hash.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);

    assert_eq!(node_hash.frozen, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "size":0, "frozen":true, "memsize":40, "flags":{"marking":true}}"#;
    let node_hash_res = NodeHash::from_str(json_str);
    assert_eq!(node_hash_res.is_ok(), true);

    let node_hash = node_hash_res.unwrap();
    assert_eq!(node_hash.address, 140190151992440 as HeapAddress);
    assert_eq!(node_hash.class, None);
    assert_eq!(node_hash.size, 0);
    assert_eq!(node_hash.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_hash.memsize, 40);
    assert_eq!(node_hash.default, None);

    assert_eq!(node_hash.file, None);
    assert_eq!(node_hash.line, None);
    assert_eq!(node_hash.method, None);
    assert_eq!(node_hash.generation, None);

    let flags = node_hash.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, Some(true));

    assert_eq!(node_hash.frozen, Some(true));

  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_hash_res = NodeHash::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_hash_res.is_ok(), false);
  }
}
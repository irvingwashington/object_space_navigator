use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeImemo {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex_opt")]
    #[serde(default)]
    class: Option<HeapAddress>,
    imemo_type: String, // TODO: enum
    #[serde(default)]
    #[serde(deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    memsize: usize,
    pub flags: Flags,
    frozen: Option<bool>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeImemo {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9748b34f0", "type":"IMEMO", "imemo_type":"iseq", "references":["0x7fc9690cfb90"], "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_imemo_res = NodeImemo::from_str(json_str);
    assert_eq!(node_imemo_res.is_ok(), true);

    let node_imemo = node_imemo_res.unwrap();
    assert_eq!(node_imemo.address, 140503220434160 as HeapAddress);
    assert_eq!(node_imemo.class, None);
    assert_eq!(node_imemo.imemo_type, String::from("iseq"));
    assert_eq!(node_imemo.references, &[140503027612560 as HeapAddress]);
    assert_eq!(node_imemo.memsize, 40);

    let flags = node_imemo.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));

    assert_eq!(flags.marking, None);
    assert_eq!(node_imemo.frozen, None);
    assert_eq!(node_imemo.file, None);
    assert_eq!(node_imemo.line, None);
    assert_eq!(node_imemo.method, None);
    assert_eq!(node_imemo.generation, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9748b34f0", "class":"0x7fc9690cfb90", "frozen":true, "file":"test", "line":1, "method":"dup", "generation":1, "imemo_type":"iseq", "memsize":40, "flags":{"wb_protected":true}}"#;
    let node_imemo_res = NodeImemo::from_str(json_str);
    assert_eq!(node_imemo_res.is_ok(), true);

    let node_imemo = node_imemo_res.unwrap();
    assert_eq!(node_imemo.address, 140503220434160 as HeapAddress);
    assert_eq!(node_imemo.class, Some(140503027612560 as HeapAddress));

    assert_eq!(node_imemo.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_imemo.frozen, Some(true));
    assert_eq!(node_imemo.file, Some(String::from("test")));
    assert_eq!(node_imemo.line, Some(1 as usize));
    assert_eq!(node_imemo.method, Some(String::from("dup")));
    assert_eq!(node_imemo.generation, Some(1 as usize));
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_imemo_res = NodeImemo::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_imemo_res.is_ok(), false);
  }
}

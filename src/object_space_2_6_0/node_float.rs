use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeFloat {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    frozen: bool,
    memsize: usize,
    value: String, // "nan", "inf", "-inf" "1.79769e+308"
    pub flags: Flags,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeFloat {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9725fad00", "type":"FLOAT", "class":"0x7fc9690cc788", "frozen":true, "value":"1e+100", "file":"(irb)", "line":11, "method":"*", "generation":23, "memsize":40, "flags":{"wb_protected":true}}"#;
    let node_float_res = NodeFloat::from_str(json_str);
    assert_eq!(node_float_res.is_ok(), true);

    let node_float = node_float_res.unwrap();
    assert_eq!(node_float.address, 140503184026880 as HeapAddress);
    assert_eq!(node_float.class, 140503027599240 as HeapAddress);
    assert_eq!(node_float.value, String::from("1e+100"));

    assert_eq!(node_float.file, Some(String::from("(irb)")));
    assert_eq!(node_float.line, Some(11 as usize));
    assert_eq!(node_float.method, Some(String::from("*")));
    assert_eq!(node_float.generation, Some(23 as usize));
    assert_eq!(node_float.memsize, 40);
    assert_eq!(node_float.frozen, true);

    let flags = node_float.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9725fad00", "class":"0x7fc9690cc788", "frozen":true, "value":"1e+100", "memsize":40, "flags":{}}"#;
    let node_float_res = NodeFloat::from_str(json_str);
    assert_eq!(node_float_res.is_ok(), true);

    let node_float = node_float_res.unwrap();
    assert_eq!(node_float.address, 140503184026880 as HeapAddress);
    assert_eq!(node_float.class, 140503027599240 as HeapAddress);
    assert_eq!(node_float.value, String::from("1e+100"));

    assert_eq!(node_float.file, None);
    assert_eq!(node_float.line, None);
    assert_eq!(node_float.method, None);
    assert_eq!(node_float.generation, None);
    assert_eq!(node_float.memsize, 40);
    assert_eq!(node_float.frozen, true);

    let flags = node_float.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_float_res = NodeFloat::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_float_res.is_ok(), false);
  }
}

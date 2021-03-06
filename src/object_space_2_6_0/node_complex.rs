use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeComplex {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    frozen: bool,
    memsize: usize,
    pub flags: Flags,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeComplex {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9725fad00", "type":"COMPLEX", "class":"0x7fc9690cc788", "frozen":true, "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_complex_res = NodeComplex::from_str(json_str);
    assert_eq!(node_complex_res.is_ok(), true);

    let node_complex = node_complex_res.unwrap();
    assert_eq!(node_complex.address, 140503184026880 as HeapAddress);
    assert_eq!(node_complex.class, 140503027599240 as HeapAddress);

    assert_eq!(node_complex.file, None);
    assert_eq!(node_complex.line, None);
    assert_eq!(node_complex.method, None);
    assert_eq!(node_complex.generation, None);
    assert_eq!(node_complex.memsize, 40);
    assert_eq!(node_complex.frozen, true);

    let flags = node_complex.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9725fad00", "class":"0x7fc9690cc788", "frozen":true, "memsize":40, "flags":{}, "file":"/test", "method":"test", "line":10, "generation":20}"#;
    let node_complex_res = NodeComplex::from_str(json_str);
    assert_eq!(node_complex_res.is_ok(), true);

    let node_complex = node_complex_res.unwrap();
    assert_eq!(node_complex.address, 140503184026880 as HeapAddress);
    assert_eq!(node_complex.class, 140503027599240 as HeapAddress);

    assert_eq!(node_complex.file, Some(String::from("/test")));
    assert_eq!(node_complex.line, Some(10 as usize));
    assert_eq!(node_complex.method, Some(String::from("test")));
    assert_eq!(node_complex.generation, Some(20 as usize));
    assert_eq!(node_complex.memsize, 40);
    assert_eq!(node_complex.frozen, true);

    let flags = node_complex.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_complex_res = NodeComplex::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_complex_res.is_ok(), false);
  }
}

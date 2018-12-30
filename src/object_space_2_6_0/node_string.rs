use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeString {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    frozen: Option<bool>,
    embedded: Option<bool>,
    fstring: Option<bool>,
    bytesize: Option<usize>,
    value: Option<String>,
    encoding: Option<String>,
    memsize: usize,
    pub flags: Flags,
    capacity: Option<usize>,
    shared: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeString {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9748b34f0", "type":"STRING", "class":"0x7fc9690cfb90", "shared":true, "encoding":"UTF-8", "references":["0x7fc9748b3388"], "file":"/gems/actionview-5.2.2/lib/action_view/template/resolver.rb", "line":19, "method":"dup", "generation":63, "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_string_res = NodeString::from_str(json_str);
    assert_eq!(node_string_res.is_ok(), true);

    let node_string = node_string_res.unwrap();
    assert_eq!(node_string.address, 140503220434160 as HeapAddress);
    assert_eq!(node_string.class, 140503027612560 as HeapAddress);
    assert_eq!(node_string.shared, Some(true));
    assert_eq!(node_string.encoding, Some(String::from("UTF-8")));
    assert_eq!(node_string.references, &[140503220433800 as HeapAddress]);
    assert_eq!(node_string.file, Some(String::from("/gems/actionview-5.2.2/lib/action_view/template/resolver.rb")));
    assert_eq!(node_string.line, Some(19 as usize));
    assert_eq!(node_string.method, Some(String::from("dup")));
    assert_eq!(node_string.generation, Some(63 as usize));
    assert_eq!(node_string.memsize, 40);

    assert_eq!(node_string.frozen, None);
    assert_eq!(node_string.embedded, None);
    assert_eq!(node_string.fstring, None);
    assert_eq!(node_string.bytesize, None);
    assert_eq!(node_string.value, None);
    assert_eq!(node_string.capacity, None);

    let flags = node_string.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9748b34f0", "class":"0x7fc9690cfb90", "frozen":true, "embedded":true, "fstring":true, "bytesize":17, "value":"01234567891234567", "capacity":20, "memsize":40, "flags":{"wb_protected":true}}"#;
    let node_string_res = NodeString::from_str(json_str);
    assert_eq!(node_string_res.is_ok(), true);

    let node_string = node_string_res.unwrap();
    assert_eq!(node_string.address, 140503220434160 as HeapAddress);
    assert_eq!(node_string.class, 140503027612560 as HeapAddress);

    assert_eq!(node_string.frozen, Some(true));
    assert_eq!(node_string.embedded, Some(true));
    assert_eq!(node_string.fstring, Some(true));
    assert_eq!(node_string.bytesize, Some(17 as usize));
    assert_eq!(node_string.value, Some(String::from("01234567891234567")));
    assert_eq!(node_string.capacity, Some(20 as usize));
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_string_res = NodeString::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_string_res.is_ok(), false);
  }
}

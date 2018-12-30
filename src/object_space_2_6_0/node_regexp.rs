use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeRegexp {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
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

impl NodeRegexp {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"REGEXP", "class":"0x7f80912f5800", "references":["0x7f808fd575e8"], "file":"/gems/actionpack-5.2.2/lib/action_dispatch/journey/path/pattern.rb", "line":80, "method":"accept", "generation":49, "memsize":780, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_regexp_res = NodeRegexp::from_str(json_str);
    assert_eq!(node_regexp_res.is_ok(), true);

    let node_regexp = node_regexp_res.unwrap();
    assert_eq!(node_regexp.address, 140190151992440 as HeapAddress);
    assert_eq!(node_regexp.class, 140190168340480 as HeapAddress);
    assert_eq!(node_regexp.references, &[140190145672680 as HeapAddress]);
    assert_eq!(node_regexp.memsize, 780);

    assert_eq!(node_regexp.file, Some(String::from("/gems/actionpack-5.2.2/lib/action_dispatch/journey/path/pattern.rb")));
    assert_eq!(node_regexp.line, Some(80 as usize));
    assert_eq!(node_regexp.method, Some(String::from("accept")));
    assert_eq!(node_regexp.generation, Some(49 as usize));

    let flags = node_regexp.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);

    assert_eq!(node_regexp.frozen, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "class":"0x7f80912f5800", "frozen":true, "references":[], "flags":{}, "memsize":40}"#;
    let node_regexp_res = NodeRegexp::from_str(json_str);
    assert_eq!(node_regexp_res.is_ok(), true);

    let node_regexp = node_regexp_res.unwrap();
    assert_eq!(node_regexp.address, 140190151992440 as HeapAddress);
    assert_eq!(node_regexp.class, 140190168340480 as HeapAddress);
    assert_eq!(node_regexp.frozen, Some(true));
    assert_eq!(node_regexp.memsize, 40);

    assert_eq!(node_regexp.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_regexp.file, None);
    assert_eq!(node_regexp.line, None);
    assert_eq!(node_regexp.method, None);
    assert_eq!(node_regexp.generation, None);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_regexp_res = NodeRegexp::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_regexp_res.is_ok(), false);
  }
}

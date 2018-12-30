use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeMatch {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    frozen: Option<bool>,
    memsize: usize,
    pub flags: Flags,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeMatch {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9725fad00", "type":"MATCH", "class":"0x7fc9690cc788", "references":["0x7fc9725fad01"], "file":"/gems/arel-9.0.0/lib/arel/visitors/visitor.rb", "line":17, "method":"gsub", "generation":65, "memsize":280, "flags":{"uncollectible":true, "marked":true}}"#;
    let node_match_res = NodeMatch::from_str(json_str);
    assert_eq!(node_match_res.is_ok(), true);

    let node_match = node_match_res.unwrap();
    assert_eq!(node_match.address, 140503184026880 as HeapAddress);
    assert_eq!(node_match.class, 140503027599240 as HeapAddress);
    assert_eq!(node_match.references, &[140503184026881 as HeapAddress]);

    assert_eq!(node_match.file, Some(String::from("/gems/arel-9.0.0/lib/arel/visitors/visitor.rb")));
    assert_eq!(node_match.line, Some(17 as usize));
    assert_eq!(node_match.method, Some(String::from("gsub")));
    assert_eq!(node_match.generation, Some(65 as usize));
    assert_eq!(node_match.memsize, 280);
    assert_eq!(node_match.frozen, None);

    let flags = node_match.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9725fad00", "type":"MATCH", "class":"0x7fc9690cc788", "memsize":280, "frozen":true, "flags":{"marking":true}}"#;
    let node_match_res = NodeMatch::from_str(json_str);
    assert_eq!(node_match_res.is_ok(), true);

    let node_match = node_match_res.unwrap();
    assert_eq!(node_match.address, 140503184026880 as HeapAddress);
    assert_eq!(node_match.class, 140503027599240 as HeapAddress);
    assert_eq!(node_match.references, vec![] as Vec<HeapAddress>);

    assert_eq!(node_match.file, None);
    assert_eq!(node_match.line, None);
    assert_eq!(node_match.method, None);
    assert_eq!(node_match.generation, None);
    assert_eq!(node_match.memsize, 280);
    assert_eq!(node_match.frozen, Some(true));

    let flags = node_match.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, Some(true));
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_match_res = NodeMatch::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_match_res.is_ok(), false);
  }
}

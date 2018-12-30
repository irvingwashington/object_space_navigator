use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeStruct {
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

impl NodeStruct {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc974128ff0", "type":"STRUCT", "class":"0x7fc96af43b08", "references":["0x7fc96fe640e8"], "file":"/gems/activerecord-5.2.2/lib/active_record/reflection.rb", "line":290, "method":"new", "generation":65, "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_struct_res = NodeStruct::from_str(json_str);
    assert_eq!(node_struct_res.is_ok(), true);

    let node_struct = node_struct_res.unwrap();
    assert_eq!(node_struct.address, 140503212527600 as HeapAddress);
    assert_eq!(node_struct.class, 140503059544840 as HeapAddress);
    assert_eq!(node_struct.references, &[140503142514920 as HeapAddress]);
    assert_eq!(node_struct.file, Some(String::from("/gems/activerecord-5.2.2/lib/active_record/reflection.rb")));
    assert_eq!(node_struct.line, Some(290 as usize));
    assert_eq!(node_struct.method, Some(String::from("new")));
    assert_eq!(node_struct.generation, Some(65 as usize));
    assert_eq!(node_struct.memsize, 40);

    assert_eq!(node_struct.frozen, None);

    let flags = node_struct.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc974128ff0", "class":"0x7fc96af43b08", "frozen":true, "memsize":40, "flags":{"marking":true}}"#;
    let node_struct_res = NodeStruct::from_str(json_str);
    assert_eq!(node_struct_res.is_ok(), true);

    let node_struct = node_struct_res.unwrap();
    assert_eq!(node_struct.address, 140503212527600 as HeapAddress);
    assert_eq!(node_struct.class, 140503059544840 as HeapAddress);
    assert_eq!(node_struct.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_struct.file, None);
    assert_eq!(node_struct.line, None);
    assert_eq!(node_struct.method, None);
    assert_eq!(node_struct.generation, None);
    assert_eq!(node_struct.memsize, 40);

    assert_eq!(node_struct.frozen, Some(true));

    let flags = node_struct.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, Some(true));
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_struct_res = NodeStruct::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_struct_res.is_ok(), false);
  }
}

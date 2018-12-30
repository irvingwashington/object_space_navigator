use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
struct NodeFile {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    #[serde(default, deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
    memsize: usize,
    fd: usize,
    pub flags: Option<Flags>,
    frozen: Option<bool>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeFile {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7f809035e478", "type":"FILE", "class":"0x7f80912f5800", "fd":27, "file":"/ruby-2.6.0/lib/ruby/2.6.0/net/http.rb", "line":947, "method":"open", "generation":65, "memsize":232}"#;
    let node_file_res = NodeFile::from_str(json_str);
    assert_eq!(node_file_res.is_ok(), true);

    let node_file = node_file_res.unwrap();
    assert_eq!(node_file.address, 140190151992440 as HeapAddress);
    assert_eq!(node_file.class, 140190168340480 as HeapAddress);
    assert_eq!(node_file.references, vec![] as Vec<HeapAddress>);
    assert_eq!(node_file.memsize, 232);

    assert_eq!(node_file.file, Some(String::from("/ruby-2.6.0/lib/ruby/2.6.0/net/http.rb")));
    assert_eq!(node_file.line, Some(947 as usize));
    assert_eq!(node_file.method, Some(String::from("open")));
    assert_eq!(node_file.generation, Some(65 as usize));
    assert_eq!(node_file.fd, 27 as usize);
    assert_eq!(node_file.frozen, None);

    let flags_opt = node_file.flags;
    assert_eq!(flags_opt.is_none(), true);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7f809035e478", "class":"0x7f80912f5800", "references": ["0x7f809035e479"], "fd":27, "frozen": true, "memsize":232, "flags": {"marking":true}}"#;
    let node_file_res = NodeFile::from_str(json_str);
    assert_eq!(node_file_res.is_ok(), true);

    let node_file = node_file_res.unwrap();
    assert_eq!(node_file.address, 140190151992440 as HeapAddress);
    assert_eq!(node_file.class, 140190168340480 as HeapAddress);
    assert_eq!(node_file.references, &[140190151992441 as HeapAddress]);
    assert_eq!(node_file.memsize, 232);

    assert_eq!(node_file.file, None);
    assert_eq!(node_file.line, None);
    assert_eq!(node_file.method, None);
    assert_eq!(node_file.generation, None);
    assert_eq!(node_file.fd, 27 as usize);
    assert_eq!(node_file.frozen, Some(true));

    let flags_opt = node_file.flags;
    assert_eq!(flags_opt.is_some(), true);
    let flags = flags_opt.unwrap();
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, Some(true));
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_file_res = NodeFile::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_file_res.is_ok(), false);
  }
}
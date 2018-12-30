use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;
use super::flags::Flags;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeSymbol {
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    address: HeapAddress,
    #[serde(deserialize_with = "DeserializeUtils::from_hex")]
    class: HeapAddress,
    frozen: bool,
    bytesize: Option<usize>,
    value: String,
    memsize: usize,
    pub flags: Flags,
    capacity: Option<usize>,
    file: Option<String>,
    line: Option<usize>,
    method: Option<String>,
    generation: Option<usize>,
}

impl NodeSymbol {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes_with_some_fields_part_1() {
    let json_str = r#"{"address":"0x7fc9725fad00", "type":"SYMBOL", "class":"0x7fc9690cc788", "frozen":true, "bytesize":15, "value":"diagram_counter", "file":"/gems/actionview-5.2.2/lib/action_view/renderer/partial_renderer.rb", "line":531, "method":"retrieve_variable", "generation":65, "memsize":40, "flags":{"wb_protected":true, "old":true, "uncollectible":true, "marked":true}}"#;
    let node_symbol_res = NodeSymbol::from_str(json_str);
    assert_eq!(node_symbol_res.is_ok(), true);

    let node_symbol = node_symbol_res.unwrap();
    assert_eq!(node_symbol.address, 140503184026880 as HeapAddress);
    assert_eq!(node_symbol.class, 140503027599240 as HeapAddress);

    assert_eq!(node_symbol.file, Some(String::from("/gems/actionview-5.2.2/lib/action_view/renderer/partial_renderer.rb")));
    assert_eq!(node_symbol.line, Some(531 as usize));
    assert_eq!(node_symbol.method, Some(String::from("retrieve_variable")));
    assert_eq!(node_symbol.generation, Some(65 as usize));
    assert_eq!(node_symbol.memsize, 40);
    assert_eq!(node_symbol.frozen, true);
    assert_eq!(node_symbol.bytesize, Some(15));
    assert_eq!(node_symbol.value, String::from("diagram_counter"));
    assert_eq!(node_symbol.capacity, None);

    let flags = node_symbol.flags;
    assert_eq!(flags.wb_protected, Some(true));
    assert_eq!(flags.old, Some(true));
    assert_eq!(flags.uncollectible, Some(true));
    assert_eq!(flags.marked, Some(true));
    assert_eq!(flags.marking, None);
  }

  #[test]
  fn it_deserializes_with_some_fields_part_2() {
    let json_str = r#"{"address":"0x7fc9725fad00", "class":"0x7fc9690cc788", "frozen":true, "value":"diagram_counter", "capacity":40, "memsize":40, "flags":{"marking":true}}"#;
    let node_symbol_res = NodeSymbol::from_str(json_str);
    assert_eq!(node_symbol_res.is_ok(), true);

    let node_symbol = node_symbol_res.unwrap();
    assert_eq!(node_symbol.address, 140503184026880 as HeapAddress);
    assert_eq!(node_symbol.class, 140503027599240 as HeapAddress);

    assert_eq!(node_symbol.file, None);
    assert_eq!(node_symbol.line, None);
    assert_eq!(node_symbol.method, None);
    assert_eq!(node_symbol.generation, None);
    assert_eq!(node_symbol.memsize, 40);
    assert_eq!(node_symbol.frozen, true);
    assert_eq!(node_symbol.bytesize, None);
    assert_eq!(node_symbol.value, String::from("diagram_counter"));
    assert_eq!(node_symbol.capacity, Some(40 as usize));

    let flags = node_symbol.flags;
    assert_eq!(flags.wb_protected, None);
    assert_eq!(flags.old, None);
    assert_eq!(flags.uncollectible, None);
    assert_eq!(flags.marked, None);
    assert_eq!(flags.marking, Some(true));
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_symbol_res = NodeSymbol::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_symbol_res.is_ok(), false);
  }
}

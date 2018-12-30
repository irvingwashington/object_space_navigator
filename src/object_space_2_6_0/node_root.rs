use serde_json::Error;
use crate::heap_address::HeapAddress;
use crate::deserialize_utils::DeserializeUtils;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct NodeRoot {
    root: String,
    #[serde(deserialize_with = "DeserializeUtils::from_hex_array")]
    references: Vec<HeapAddress>,
}

impl NodeRoot {
    fn from_str(json_form: &str) -> Result<Self, Error> {
        serde_json::from_str(json_form)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_deserializes() {
    let node_root_res = NodeRoot::from_str(r#"{"type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_root_res.is_ok(), true);

    let node_root = node_root_res.unwrap();
    assert_eq!(node_root.root, "vm");
    assert_eq!(node_root.references, &[140503027253160 as HeapAddress]);
  }

  #[test]
  fn it_fails_to_deserialize() {
    let node_root_res = NodeRoot::from_str(r#""type":"ROOT", "root":"vm", "references":["0x7fc969077fa8"]}"#);
    assert_eq!(node_root_res.is_ok(), false);
  }
}
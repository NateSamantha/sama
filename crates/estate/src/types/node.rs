use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize, Hash)]
pub struct NodeName(String);

#[derive(Clone, Deserialize, Serialize)]
pub struct Region(String);

#[derive(Clone, Deserialize, Serialize)]
pub struct NodeInfo {
    pub name: NodeName,
    pub region: Region,
}

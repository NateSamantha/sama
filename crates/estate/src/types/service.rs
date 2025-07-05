use serde::{Deserialize, Serialize};

use super::{app::AppId, node::Region};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize, Debug)]
pub struct ExternalDestination(u16);

impl ExternalDestination {
    pub fn from_port(port: u16) -> Self {
        ExternalDestination(port)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DefinitionTarget(pub String);

#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Serialize, Debug)]
pub struct DefinitionKey {
    pub app_id: AppId,
    pub dest: ExternalDestination,
}

impl DefinitionKey {
    pub fn new(app_id: AppId, dest: ExternalDestination) -> DefinitionKey {
        DefinitionKey { app_id, dest }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Definition {
    #[serde(flatten)]
    pub def_key: DefinitionKey,
    pub region: Region,
    pub target: DefinitionTarget,
}

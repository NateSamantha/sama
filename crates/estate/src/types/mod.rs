use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    path::Path,
};

use app::{App, AppId};
use node::{NodeInfo, NodeName};
use serde::Deserialize;
use service::{Definition, DefinitionKey, ExternalDestination};

pub mod app;
pub mod node;
pub mod service;

pub struct Route(DefinitionKey);

impl Route {
    pub fn def_key(&self) -> &DefinitionKey {
        &self.0
    }
}

pub struct Catalog {
    pub node_by_name: HashMap<NodeName, NodeInfo>,
    pub app_by_id: HashMap<AppId, App>,
    pub app_by_ip: HashMap<IpAddr, AppId>,
    pub def_by_key: HashMap<DefinitionKey, Vec<Definition>>,
}

impl Catalog {
    pub async fn from_json(f: impl AsRef<Path>) -> eyre::Result<Catalog> {
        let s = tokio::fs::read_to_string(f).await?;
        let inner: SerializedCatalog = serde_json::from_str(&s)?;

        Ok(inner.into())
    }

    pub fn route(&self, local_addr: SocketAddr) -> Option<Route> {
        let app_id = self.app_by_ip.get(&local_addr.ip())?;
        let def_key =
            DefinitionKey::new(*app_id, ExternalDestination::from_port(local_addr.port()));
        if !self.def_by_key.contains_key(&def_key) {
            None
        } else {
            Some(Route(def_key))
        }
    }
}

#[derive(Deserialize)]
struct SerializedCatalog {
    nodes: Vec<NodeInfo>,
    apps: Vec<App>,
    services: Vec<Definition>,
}

impl From<SerializedCatalog> for Catalog {
    fn from(value: SerializedCatalog) -> Self {
        let node_by_name: HashMap<_, _> = value
            .nodes
            .into_iter()
            .map(|n| (n.name.clone(), n))
            .collect();
        let app_by_ip: HashMap<_, _> = value
            .apps
            .iter()
            .flat_map(|a| a.ips.iter().map(|ip| (ip.clone(), a.id)))
            .collect();
        let app_by_id: HashMap<_, _> = value.apps.into_iter().map(|a| (a.id, a)).collect();
        let mut def_by_key: HashMap<DefinitionKey, Vec<Definition>> = HashMap::new();

        for def in value.services.into_iter() {
            def_by_key.entry(def.def_key.clone()).or_default().push(def);
        }

        Catalog {
            node_by_name,
            app_by_id,
            app_by_ip,
            def_by_key,
        }
    }
}

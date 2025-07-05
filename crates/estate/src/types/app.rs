use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Deserialize, Serialize, Debug)]
pub struct AppId(u32);

#[derive(Clone, Deserialize, Serialize)]
pub struct App {
    pub id: AppId,
    pub name: String,
    pub ips: Vec<IpAddr>,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Nodes(Vec<String>);

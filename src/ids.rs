use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct CharacterId(pub String);

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Token(pub String);

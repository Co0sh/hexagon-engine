use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum GroundFeature {
    FOREST,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

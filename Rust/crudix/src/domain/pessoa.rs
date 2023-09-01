use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pessoa {
    nome: String,
    cpf: String,
}

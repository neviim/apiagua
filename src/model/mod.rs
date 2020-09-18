use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Consumo {
  pub idkey: String,
  pub host: String,
  pub relogio: i32,
  pub consumo:  i32,
  pub lavouroupa: bool,
  pub quantidade: u32,
  pub observacao: String,
  pub diffdias: i32,
  pub difftime: i32,
  pub data_cadastro: String,
  pub data_alteracao: String
}
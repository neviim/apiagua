use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Consumo {
  pub title: String,
  pub genre: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct _Consumo {
  pub _idkey: String,
  pub host: String,
  pub relogio: i32,
  pub consumo:  i32,
  pub lavouroupa: bool,
  pub quantidade: u32,
  pub observacao: String,
  pub diffdias: i16,
  pub difftime: i16,
  pub data_cadastro: String,
  pub data_alteracao: String
}
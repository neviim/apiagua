use std::fs;
use crate::model::Consumo;

static CONSUMO_DB: &str = "data/consumos.json";

fn _consumos() -> Result<Vec<Consumo>, serde_json::Error> {
    let data = fs::read_to_string(CONSUMO_DB).expect("Error reading from file");
    let consumos: Result<Vec<Consumo>, serde_json::Error> = serde_json::from_str(&data);
    consumos
}

fn _write_consumos(consumos: Vec<Consumo>) {
    let data = serde_json::to_string(&consumos).expect("Failed to turn consumos into serde string");
    fs::write(CONSUMO_DB, data).expect("Failed to write data.");
}

pub fn read_consumos() -> Option<Vec<Consumo>> {
    match _consumos() {
        Ok(consumos) => Some(consumos),
        Err(_) => None
    }
}

pub fn read_consumo(idkey: String) -> Option<Consumo> {
    match _consumos() {
        Ok(consumos) => {
            let index = consumos.iter().position(|m| m.idkey == idkey);

            match index {
                Some(x) => Some(consumos[x].clone()),
                None => None,
            }
        },
        Err(_) => None
    }
}

pub fn insert_consumo(consumo: Consumo) -> Option<Consumo> {
    match _consumos() {
        Ok(mut consumos) => {
            consumos.push(consumo.clone());
            _write_consumos(consumos);
            Some(consumo)
        },
        Err(_) => None
    }
}

pub fn delete_consumo(idkey: String) -> bool {
    match _consumos() {
        Ok(mut consumos) => {
            let index = consumos.iter().position(|m| m.idkey == idkey);

            match index {
                Some(x) => {
                    consumos.remove(x);
                    _write_consumos(consumos);
                    true
                },
                None => false,
            }
        },
        Err(_) => false
    }
}
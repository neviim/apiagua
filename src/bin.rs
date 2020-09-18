#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket::http::RawStr;
use lib::db;
use lib::model::Consumo;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get_consumos() -> Json<Option<Vec<Consumo>>> {
    Json(db::read_consumos())
}

#[get("/<idkey>")]
fn get_consumo(idkey: &RawStr) -> Json<Option<Consumo>> {
    Json(db::read_consumo(idkey.url_decode().expect("Failed to decode idkey.")))
}

#[post("/", data="<consumo>")]
fn create_consumo(consumo: Json<Consumo>) -> Json<Option<Consumo>> {
    Json(db::insert_consumo(consumo.0))
}

#[delete("/<idkey>")]
fn delete_consumo(idkey: &RawStr) -> Json<bool> {
    Json(db::delete_consumo(idkey.url_decode().expect("Failed to decode idkey.")))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/consumos",
        routes![get_consumos, get_consumo, create_consumo, delete_consumo],
    )
}
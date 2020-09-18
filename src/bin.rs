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

#[get("/<title>")]
fn get_consumo(title: &RawStr) -> Json<Option<Consumo>> {
    Json(db::read_consumo(title.url_decode().expect("Failed to decode title.")))
}

#[post("/", data="<consumo>")]
fn create_consumo(consumo: Json<Consumo>) -> Json<Option<Consumo>> {
    Json(db::insert_consumo(consumo.0))
}

#[delete("/<title>")]
fn delete_consumo(title: &RawStr) -> Json<bool> {
    Json(db::delete_consumo(title.url_decode().expect("Failed to decode title.")))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/consumos",
        routes![get_consumos, get_consumo, create_consumo, delete_consumo],
    )
}
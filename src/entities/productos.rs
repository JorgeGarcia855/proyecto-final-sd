use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Productos {
    codigo: i64,
    nit_proveedor: i64,
    iva_compra: f64,
    nombre_producto: String,
    precio_compra: f64,
    precio_venta: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateProducto {
    nit_proveedor: i64,
    iva_compra: f64,
    nombre_producto: String,
    precio_compra: f64,
    precio_venta: f64,
}

// #[post("/")]
// pub async fn create(state: Data<AppState>, producto: Json<Productos>) -> impl Responder {
//     todo!()
// }

// #[get("/")]
// pub async fn read_all(state: Data<AppState>) -> impl Responder {
//     todo!()
// }

// #[get("/{id}")]
// pub async fn read_by_id(state: Data<AppState>,  path: Path<i64>) -> impl Responder {
//     todo!()
// }

// #[patch("/{id}")]
// pub async fn update(state: Data<AppState>,  path: Path<i64>, producto: Json<UpdateUsuario>) -> impl Responder {
//     todo!()
// }

// #[delete("/{id}")]
// pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
//     todo!()
// }

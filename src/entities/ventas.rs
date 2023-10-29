use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Ventas {
    codigo: i64,
    cedula_cliente: i64,
    cedula_usuario: i64,
    iva_venta: f64,
    total_venta: f64,
    valor_venta: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateVenta {
    cedula_cliente: i64,
    cedula_usuario: i64,
    iva_venta: f64,
    total_venta: f64,
    valor_venta: f64,
}

// #[post("/")]
// pub async fn create(state: Data<AppState>, venta: Json<Ventas>) -> impl Responder {
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
// pub async fn update(state: Data<AppState>,  path: Path<i64>, venta: Json<UpdateUsuario>) -> impl Responder {
//     todo!()
// }

// #[delete("/{id}")]
// pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
//     todo!()
// }

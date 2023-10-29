use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct DetalleVentas {
    codigo: i64,
    codigo_producto: i64,
    codigo_venta: i64,
    cantidad_producto: i32,
    valor_total: f64,
    valor_venta: f64,
    valor_iva: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateDetalleVenta {
    codigo_producto: i64,
    codigo_venta: i64,
    cantidad_producto: i32,
    valor_total: f64,
    valor_venta: f64,
    valor_iva: f64,
}

// #[post("/")]
// pub async fn create(state: Data<AppState>, detalle_venta: Json<DetalleVentas>) -> impl Responder {
//     todo!()
// }

// #[get("/")]
// pub async fn read_all(state: Data<AppState>) -> impl Responder {
//     todo!()
// }

// #[get("/{id}")]
// pub async fn read_by_id(state: Data<AppState>,  path: Path<i64>, detalle_venta: Json<UpdateUsuario>) -> impl Responder {
//     todo!()
// }

// #[patch("/{id}")]
// pub async fn update(state: Data<AppState>,  path: Path<i64>) -> impl Responder {
//     todo!()
// }

// #[delete("/{id}")]
// pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
//     todo!()
// }
//!Este archivo representa el servicio REST de la tabla 'detalle_ventas'.
//!Contiene todas las operaciones CRUD relacionadas.

use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// La tabla 'detalle_ventas' representada como un struct.
/// La llave primaria y las foraneas son opcionales debido a la operacion de `update`
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct DetalleVentas {
    codigo: Option<i64>,
    codigo_producto: Option<i64>,
    codigo_venta: Option<i64>,
    cantidad_producto: i32,
    valor_total: f64,
    valor_venta: f64,
    valor_iva: f64,
}

/// Crea un nuevo detalle de la venta y lo envia a la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `venta` - Un json en el body del request representando el detalle de la venta
#[post("/")]
pub async fn create(state: Data<AppState>, detalle_venta: Json<DetalleVentas>) -> impl Responder {
    match sqlx::query_as::<_, DetalleVentas>(
        "insert into detalle_ventas values ($1,$2,$3,$4,$5,$6,$7);",
    )
    .bind(detalle_venta.codigo)
    .bind(detalle_venta.codigo_producto)
    .bind(detalle_venta.codigo_venta)
    .bind(detalle_venta.cantidad_producto)
    .bind(detalle_venta.valor_total)
    .bind(detalle_venta.valor_venta)
    .bind(detalle_venta.valor_iva)
    .fetch_optional(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Created().json("detalle venta creada"),
        Err(_) => HttpResponse::InternalServerError().json("could not create detalle venta"),
    }
}

/// Obtiene todos los detalles de la venta de la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, DetalleVentas>("select * from detalle_ventas;")
        .fetch_all(&state.db)
        .await
    {
        Ok(ventas) => HttpResponse::Ok().json(ventas),
        Err(_) => HttpResponse::NotFound().json("detalle ventas not found"),
    }
}

/// Obtiene un detalle de la venta de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, DetalleVentas>("select * from detalle_ventas where codigo = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(venta) => HttpResponse::Ok().json(venta),
        Err(_) => HttpResponse::NotFound().json("detalle venta not found"),
    }
}

/// Actualiza un detalle de la venta de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
/// * `detalle_venta` - Un json en el body del request representando el detalle de la venta a actualizar
#[patch("/{id}")]
pub async fn update(state: Data<AppState>, path: Path<i64>, detalle_venta: Json<DetalleVentas>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, DetalleVentas>("update detalle_ventas set cantidad_producto = $1, valor_total = $2, valor_venta = $3, valor_iva = $4 where codigo = $5;")
        .bind(detalle_venta.cantidad_producto)
        .bind(detalle_venta.valor_total)
        .bind(detalle_venta.valor_venta)
        .bind(detalle_venta.valor_iva)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("detalle venta updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update detalle venta")
    }
}

/// Borra un detalle de la venta de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, DetalleVentas>("delete from detalle_ventas where codigo = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("detalle venta deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete detalle venta"),
    }
}

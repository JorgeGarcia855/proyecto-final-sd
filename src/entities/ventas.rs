//!Este archivo representa el servicio REST de la tabla 'ventas'.
//!Contiene todas las operaciones CRUD relacionadas.

use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// La tabla 'ventas' representada como un struct.
/// La llave primaria y las foraneas son opcionales debido a la operacion de `update`
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Ventas {
    codigo: Option<i64>,
    cedula_cliente: Option<i64>,
    cedula_usuario: Option<i64>,
    iva_venta: f64,
    total_venta: f64,
    valor_venta: f64,
}

/// Crea una nueva venta y la envia a la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `venta` - Un json en el body del request representando la venta
#[post("/")]
pub async fn create(state: Data<AppState>, venta: Json<Ventas>) -> impl Responder {
    match sqlx::query_as::<_, Ventas>("insert into ventas values ($1,$2,$3,$4,$5,$6);")
        .bind(venta.codigo)
        .bind(venta.cedula_cliente)
        .bind(venta.cedula_usuario)
        .bind(venta.iva_venta)
        .bind(venta.total_venta)
        .bind(venta.valor_venta)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("venta creada"),
        Err(e) => {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().json("could not create venta")},
    }
}

/// Obtiene todos las ventas de la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Ventas>("select * from ventas;")
        .fetch_all(&state.db)
        .await
    {
        Ok(ventas) => HttpResponse::Ok().json(ventas),
        Err(_) => HttpResponse::NotFound().json("ventas not found"),
    }
}

/// Obtiene una venta de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Ventas>("select * from ventas where codigo = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(venta) => HttpResponse::Ok().json(venta),
        Err(_) => HttpResponse::NotFound().json("venta not found"),
    }
}

/// Actualiza una venta de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
/// * `venta` - Un json en el body del request representando la venta a actualizar
#[patch("/{id}")]
pub async fn update(state: Data<AppState>, path: Path<i64>, venta: Json<Ventas>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Ventas>(
        "update ventas set iva_venta = $1, total_venta = $2, valor_venta = $3 where codigo = $4;",
    )
    .bind(venta.iva_venta)
    .bind(venta.total_venta)
    .bind(venta.valor_venta)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json("venta updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update venta"),
    }
}

/// Borra una venta de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Ventas>("delete from ventas where codigo = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Venta deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete venta"),
    }
}

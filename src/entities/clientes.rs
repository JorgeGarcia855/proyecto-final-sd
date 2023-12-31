//!Este archivo representa el servicio REST de la tabla 'clientes'.
//!Contiene todas las operaciones CRUD relacionadas.

use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// La tabla 'cliente' representada como un struct.
/// Cedula es opcional debido a la operacion de `update`
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Clientes {
    cedula: Option<i64>,
    direccion: String,
    email: String,
    nombre: String,
    telefono: String,
}

/// Crea un nuevo cliente y lo envia a la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `cliente` - Un json en el body del request representando el cliente
#[post("/")]
pub async fn create(state: Data<AppState>, cliente: Json<Clientes>) -> impl Responder {
    match sqlx::query_as!(
        Clientes,
        "insert into clientes values ($1,$2,$3,$4,$5);",
        cliente.cedula,
        cliente.direccion.as_str(),
        cliente.email.as_str(),
        cliente.nombre.as_str(),
        cliente.telefono.as_str()
    )
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Cliente creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create client"),
    }
}

/// Obtiene todos los clientes de la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(Clientes,"select * from clientes;")
        .fetch_all(&state.db)
        .await
    {
        Ok(clientes) => HttpResponse::Ok().json(clientes),
        Err(_) => HttpResponse::NotFound().json("not clientes found"),
    }
}

/// Obtiene un cliente de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Clientes, "select * from clientes where cedula = $1;", id)
        .fetch_one(&state.db)
        .await
    {
        Ok(cliente) => HttpResponse::Ok().json(cliente),
        Err(_) => HttpResponse::NotFound().json("cliente not found"),
    }
}

/// Actualiza un cliente de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
/// * `cliente` - Un json en el body del request representando el cliente a actualizar
#[patch("/{id}")]
pub async fn update(state: Data<AppState>, path: Path<i64>, cliente: Json<Clientes>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(
        Clientes,
        "update clientes set direccion = $1, email = $2, nombre = $3, telefono = $4 where cedula = $5;",
        cliente.direccion.as_str(),
        cliente.email.as_str(),
        cliente.nombre.as_str(),
        cliente.telefono.as_str(),
        id
    )
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Client updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update client")
    }
}

/// Borra un cliente de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Clientes, "delete from clientes where cedula = $1;", id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Client deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Could not delete, reason: {e}"))
    }
}

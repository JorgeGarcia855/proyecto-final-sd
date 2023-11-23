//!Este archivo representa el servicio REST de la tabla 'proveedores'.
//!Contiene todas las operaciones CRUD relacionadas.

use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// La tabla 'proveedores' representada como un struct.
/// NIT es opcional debido a la operacion de `update`
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Proveedores {
    nit: Option<i64>,
    ciudad: String,
    direccion: String,
    nombre: String,
    telefono: String,
}

/// Crea un nuevo proveedor y lo envia a la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `proveedor` - Un json en el body del request representando el proveedor
#[post("/")]
pub async fn create(state: Data<AppState>, proveedor: Json<Proveedores>) -> impl Responder {
    match sqlx::query_as!(Proveedores, 
        "insert into proveedores values ($1,$2,$3,$4,$5);",
        proveedor.nit,
        proveedor.ciudad.as_str(),
        proveedor.direccion.as_str(),
        proveedor.nombre.as_str(),
        proveedor.telefono.as_str()
    )
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Proveedor creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create proveedor"),
    }
}

/// Obtiene todos los proveedores de la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(Proveedores, "select * from proveedores;")
        .fetch_all(&state.db)
        .await
    {
        Ok(proveedores) => HttpResponse::Ok().json(proveedores),
        Err(_) => HttpResponse::NotFound().json("proveedores not found"),
    }
}

/// Obtiene un proveedor de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Proveedores, "select * from proveedores where nit = $1;", id)
        .fetch_one(&state.db)
        .await
    {
        Ok(proveedor) => HttpResponse::Ok().json(proveedor),
        Err(_) => HttpResponse::NotFound().json("proveedor not found"),
    }
}

/// Actualiza un proveedor de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
/// * `proveedor` - Un json en el body del request representando el proveedor a actualizar
#[patch("/{id}")]
pub async fn update(state: Data<AppState>, path: Path<i64>, proveedor: Json<Proveedores>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Proveedores, 
        "update proveedores set ciudad = $1, direccion = $2, nombre = $3, telefono = $4 where nit = $5;",
        proveedor.ciudad.as_str(),
        proveedor.direccion.as_str(),
        proveedor.nombre.as_str(),
        proveedor.telefono.as_str(),
        id
    )
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Proveedor updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update proveedor")
    }
}

/// Borra un proveedor de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Proveedores, "delete from proveedores where nit = $1;", id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Proveedor deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete proveedor"),
    }
}

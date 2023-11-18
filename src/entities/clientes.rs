use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Clientes {
    cedula: Option<i64>,
    direccion: String,
    email: String,
    nombre: String,
    telefono: String,
}

#[post("/")]
pub async fn create(state: Data<AppState>, cliente: Json<Clientes>) -> impl Responder {
    match sqlx::query_as::<_, Clientes>("insert into clientes values ($1,$2,$3,$4,$5);")
        .bind(cliente.cedula)
        .bind(cliente.direccion.as_str())
        .bind(cliente.email.as_str())
        .bind(cliente.nombre.as_str())
        .bind(cliente.telefono.as_str())
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Cliente creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create client"),
    }
}

#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Clientes>("select * from clientes;")
        .fetch_all(&state.db)
        .await
    {
        Ok(clientes) => HttpResponse::Ok().json(clientes),
        Err(_) => HttpResponse::NotFound().json("not clientes found"),
    }
}

#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Clientes>("select * from clientes where cedula = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(cliente) => HttpResponse::Ok().json(cliente),
        Err(_) => HttpResponse::NotFound().json("cliente not found"),
    }
}

#[patch("/{id}")]
pub async fn update(
    state: Data<AppState>,
    path: Path<i64>,
    cliente: Json<Clientes>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Clientes>("update clientes set direccion = $1, email = $2, nombre = $3, telefono = $4 where cedula = $5;")
        .bind(cliente.direccion.as_str())
        .bind(cliente.email.as_str())
        .bind(cliente.nombre.as_str())
        .bind(cliente.telefono.as_str())
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Client updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update client")
    }
}

#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Clientes>("delete from clientes where cedula = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Client deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete client"),
    }
}

use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Usuarios {
    cedula: i64,
    email: String,
    nombre: String,
    password: String,
    usuario: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateUsuario {
    email: String,
    nombre: String,
    password: String,
    usuario: String,
}

#[post("/")]
pub async fn create(state: Data<AppState>, usuario: Json<Usuarios>) -> impl Responder {
    match sqlx::query_as::<_, Usuarios>("insert into usuarios values ($1,$2,$3,$4,$5);")
        .bind(usuario.cedula)
        .bind(usuario.email.as_str())
        .bind(usuario.nombre.as_str())
        .bind(usuario.password.as_str())
        .bind(usuario.usuario.as_str())
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Usuario creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create user")
    }
}

#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Usuarios>("select * from usuarios;")
        .fetch_all(&state.db)
        .await
    {
        Ok(usuarios) => HttpResponse::Ok().json(usuarios),
        Err(_) => HttpResponse::NotFound().json("users not found")
    }
}

#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>,  path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Usuarios>("select * from usuarios where cedula = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(usuario) => HttpResponse::Ok().json(usuario),
        Err(_) => HttpResponse::NotFound().json("user not found")
    } 
}

#[patch("/{id}")]
pub async fn update(state: Data<AppState>,  path: Path<i64>, usuario: Json<UpdateUsuario>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Usuarios>("update usuarios set email = $1, nombre = $2, password = $3, usuario = $4 where cedula = $5;")
        .bind(usuario.email.as_str())
        .bind(usuario.nombre.as_str())
        .bind(usuario.password.as_str())
        .bind(usuario.usuario.as_str())
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Usuario updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update user")
    }
}

#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Usuarios>("delete from usuarios where cedula = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Usuario deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete user")
    }
}

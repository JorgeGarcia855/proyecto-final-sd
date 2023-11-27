//!Este archivo representa el servicio REST de la tabla 'usuarios'.
//!Contiene todas las operaciones CRUD relacionadas.

use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// La tabla 'usuarios' representada como un struct.
/// Cedula es opcional debido a la operacion de `update`
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Usuarios {
    cedula: Option<i64>,
    email: Option<String>,
    nombre: Option<String>,
    password: String,
    usuario: String,
}

/// Crea un nuevo usuario y lo envia a la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `usuario` - Un json en el body del request representando el usuario
#[post("/")]
pub async fn create(state: Data<AppState>, usuario: Json<Usuarios>) -> impl Responder {
    match sqlx::query_as!(
        Usuarios,
        "insert into usuarios values ($1,$2,$3,$4,$5);",
        usuario.cedula,
        usuario.email,
        usuario.nombre,
        usuario.password.as_str(),
        usuario.usuario.as_str()
    )
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Usuario creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create user"),
    }
}

/// Obtiene todos los usuarios de la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(Usuarios, "select * from usuarios;")
        .fetch_all(&state.db)
        .await
    {
        Ok(usuarios) => HttpResponse::Ok().json(usuarios),
        Err(_) => HttpResponse::NotFound().json("users not found"),
    }
}

/// Obtiene un usuario de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Usuarios,"select * from usuarios where cedula = $1;", id)
        .fetch_one(&state.db)
        .await
    {
        Ok(usuario) => HttpResponse::Ok().json(usuario),
        Err(_) => HttpResponse::NotFound().json("user not found"),
    }
}


/// Autenticacion basica de un usuario
/// * `state` - La coneccion a la base de datos
/// * `form` - Un json en el body del request representando el usuario que se ingreso en el formulario de login
#[post("/login")]
pub async fn auth(state: Data<AppState>, form: Json<Usuarios>) -> impl Responder {
    match sqlx::query_as!(Usuarios, "select * from usuarios where usuario = $1;", form.usuario).fetch_optional(&state.db).await {
        Ok(Some(user)) => {
            if user.password == form.password {
                HttpResponse::Ok().body("success")
            } else {
                HttpResponse::Unauthorized().body("bad password")
            }
        },
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// Actualiza un usuario de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
/// * `usuario` - Un json en el body del request representando el usuario a actualizar
#[patch("/{id}")]
pub async fn update(state: Data<AppState>, path: Path<i64>, usuario: Json<Usuarios>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(
        Usuarios,
        "update usuarios set email = $1, nombre = $2, password = $3, usuario = $4 where cedula = $5;",
        usuario.email,
        usuario.nombre,
        usuario.password.as_str(),
        usuario.usuario.as_str(),
        id
    )
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Usuario updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update user")
    }
}

/// Borra un usuario de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(Usuarios, "delete from usuarios where cedula = $1;", id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Usuario deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete user"),
    }
}

use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Proveedores {
    nit: Option<i64>,
    ciudad: String,
    direccion: String,
    nombre: String,
    telefono: String,
}

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

#[patch("/{id}")]
pub async fn update(
    state: Data<AppState>,
    path: Path<i64>,
    proveedor: Json<Proveedores>,
) -> impl Responder {
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

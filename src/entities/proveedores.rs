use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Proveedores {
    nit: i64,
    ciudad: String,
    direccion: String,
    nombre: String,
    telefono: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateProveedor {
    ciudad: String,
    direccion: String,
    nombre: String,
    telefono: String,
}

#[post("/")]
pub async fn create(state: Data<AppState>, proveedor: Json<Proveedores>) -> impl Responder {
    match sqlx::query_as::<_, Proveedores>("insert into proveedores values ($1,$2,$3,$4,$5);")
        .bind(proveedor.nit)
        .bind(proveedor.ciudad.as_str())
        .bind(proveedor.direccion.as_str())
        .bind(proveedor.nombre.as_str())
        .bind(proveedor.telefono.as_str())
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Proveedor creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create proveedor")
    }
}

#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Proveedores>("select * from proveedores;")
        .fetch_all(&state.db)
        .await
    {
        Ok(proveedores) => HttpResponse::Ok().json(proveedores),
        Err(_) => HttpResponse::NotFound().json("proveedores not found")
    }
}

#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>,  path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Proveedores>("select * from proveedores where nit = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(proveedor) => HttpResponse::Ok().json(proveedor),
        Err(_) => HttpResponse::NotFound().json("proveedor not found")
    } 
}

#[patch("/{id}")]
pub async fn update(state: Data<AppState>,  path: Path<i64>, proveedor: Json<UpdateProveedor>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Proveedores>("update proveedores set ciudad = $1, direccion = $2, nombre = $3, telefono = $4 where nit = $5;")
        .bind(proveedor.ciudad.as_str())
        .bind(proveedor.direccion.as_str())
        .bind(proveedor.nombre.as_str())
        .bind(proveedor.telefono.as_str())
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Proveedor updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update proveedor")
    }
}

#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Proveedores>("delete from proveedores where nit = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Proveedor deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete proveedor")
    }
}
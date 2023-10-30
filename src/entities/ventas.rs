use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Ventas {
    codigo: i64,
    cedula_cliente: i64,
    cedula_usuario: i64,
    iva_venta: f64,
    total_venta: f64,
    valor_venta: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateVenta {
    iva_venta: f64,
    total_venta: f64,
    valor_venta: f64,
}

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
        Err(_) => HttpResponse::InternalServerError().json("could not create venta")
    }
}

#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Ventas>("select * from ventas;")
        .fetch_all(&state.db)
        .await
    {
        Ok(ventas) => HttpResponse::Ok().json(ventas),
        Err(_) => HttpResponse::NotFound().json("ventas not found")
    }
}

#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>,  path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Ventas>("select * from ventas where codigo = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(venta) => HttpResponse::Ok().json(venta),
        Err(_) => HttpResponse::NotFound().json("venta not found")
    }
}

#[patch("/{id}")]
pub async fn update(state: Data<AppState>,  path: Path<i64>, venta: Json<UpdateVenta>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Ventas>("update ventas set iva_venta = $1, total_venta = $2, valor_venta = $3 where codigo = $4;")
        .bind(venta.iva_venta)
        .bind(venta.total_venta)
        .bind(venta.valor_venta)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("venta updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update venta")
    }
}

#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Ventas>("delete from ventas where codigo = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Venta deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete venta")
    }
}

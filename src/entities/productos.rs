use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use actix_web::{get, post, patch, delete, Responder, HttpResponse, web::{Data, Path, Json} };
use crate::AppState;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Productos {
    codigo: i64,
    nit_proveedor: i64,
    iva_compra: f64,
    nombre_producto: String,
    precio_compra: f64,
    precio_venta: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateProducto {
    iva_compra: f64,
    nombre_producto: String,
    precio_compra: f64,
    precio_venta: f64,
}

#[post("/")]
pub async fn create(state: Data<AppState>, producto: Json<Productos>) -> impl Responder {
    match sqlx::query_as::<_, Productos>("insert into productos values ($1,$2,$3,$4,$5,$6);")
        .bind(producto.codigo)
        .bind(producto.nit_proveedor)
        .bind(producto.iva_compra)
        .bind(producto.nombre_producto.as_str())
        .bind(producto.precio_compra)
        .bind(producto.precio_venta)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Producto creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create producto")
    }
}

#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Productos>("select * from productos;")
        .fetch_all(&state.db)
        .await
    {
        Ok(productos) => HttpResponse::Ok().json(productos),
        Err(_) => HttpResponse::NotFound().json("productos not found")
    }
}

#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>,  path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("select * from productos where codigo = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(producto) => HttpResponse::Ok().json(producto),
        Err(_) => HttpResponse::NotFound().json("producto not found")
    }
}

#[patch("/{id}")]
pub async fn update(state: Data<AppState>,  path: Path<i64>, producto: Json<UpdateProducto>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("update productos set iva_compra = $1, nombre_producto = $2, precio_compra = $3, precio venta = $4 where codigo = $5;")
        .bind(producto.iva_compra)
        .bind(producto.nombre_producto.as_str())
        .bind(producto.precio_compra)
        .bind(producto.precio_venta)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Created().json("Producto creado"),
        Err(_) => HttpResponse::InternalServerError().json("could not create producto")
    }
}

#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("delete from productos where codigo = ?;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Producto deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete producto")
    }
}

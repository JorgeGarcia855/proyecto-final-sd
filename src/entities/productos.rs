use crate::AppState;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, self},
    HttpResponse, Responder
};

use futures_util::stream::StreamExt;
use std::io::Write;
use tempfile::NamedTempFile;
use csv::ReaderBuilder;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Productos {
    codigo: Option<i64>,
    nit_proveedor: Option<i64>,
    iva_compra: f64,
    nombre_producto: String, 
    precio_compra: f64,
    precio_venta: f64,
}

#[post("/")]
pub async fn create(state: Data<AppState>, mut payload: web::Payload) -> impl Responder {
    let mut temp_file = NamedTempFile::new().unwrap();
    let mut bytes = Vec::new();

    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        temp_file.write_all(&data).unwrap();
        bytes.extend_from_slice(&data);
    }

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true) // Assumes the first row contains headers
        .from_reader(bytes.as_slice());

    let transaction = state.db.begin().await.unwrap();
    for (index, result) in csv_reader.records().enumerate() {
        match result {
            Ok(record) => {
                if let Ok(producto) = record.deserialize::<Productos>(None) {
                    if let Err(err) = sqlx::query_as::<_, Productos>("INSERT INTO productos VALUES ($1, $2, $3, $4, $5, $6);")
                        .bind(producto.codigo)
                        .bind(producto.nit_proveedor)
                        .bind(producto.iva_compra)
                        .bind(producto.nombre_producto.as_str())
                        .bind(producto.precio_compra)
                        .bind(producto.precio_venta)
                        .fetch_optional(&state.db)
                        .await
                    {
                        transaction.rollback().await.unwrap();
                        eprintln!("Error inserting product at index {}: {:?}", index, err);
                        return HttpResponse::InternalServerError().json("Could not create product");
                    }
                }
            }
            Err(err) => {
                eprintln!("Error processing CSV record at index {}: {:?}", index, err);
                return HttpResponse::InternalServerError().json("Error processing CSV record");
            }
        }
    }
    transaction.commit().await.unwrap();
    HttpResponse::Ok().body("CSV file processed and data inserted into SQLite table")
}

#[get("/")]
pub async fn read_all(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Productos>("select * from productos;")
        .fetch_all(&state.db)
        .await
    {
        Ok(productos) => HttpResponse::Ok().json(productos),
        Err(_) => HttpResponse::NotFound().json("productos not found"),
    }
}

#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("select * from productos where codigo = ?;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(producto) => HttpResponse::Ok().json(producto),
        Err(_) => HttpResponse::NotFound().json("producto not found"),
    }
}

#[patch("/{id}")]
pub async fn update(
    state: Data<AppState>,
    path: Path<i64>,
    producto: Json<Productos>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("update productos set iva_compra = $1, nombre_producto = $2, precio_compra = $3, precio_venta = $4 where codigo = $5;")
        .bind(producto.iva_compra)
        .bind(producto.nombre_producto.as_str())
        .bind(producto.precio_compra)
        .bind(producto.precio_venta)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Producto updated"),
        Err(_) => HttpResponse::InternalServerError().json("could not update producto")
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
        Err(_) => HttpResponse::InternalServerError().json("could not delete producto"),
    }
}

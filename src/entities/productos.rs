//!Este archivo representa el servicio REST de la tabla 'productos'.
//!Contiene todas las operaciones CRUD relacionadas.

use crate::AppState;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use csv::ReaderBuilder;
use futures_util::stream::StreamExt;
use std::io::{Seek, Write};
use tempfile::NamedTempFile;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// La tabla 'productos' representada como un struct.
/// El codigo y el nit son opcionales debido a la operacion de `update`
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Productos {
    codigo: Option<i64>,
    nit_proveedor: Option<i64>,
    iva_compra: f64,
    nombre_producto: String,
    precio_compra: f64,
    precio_venta: f64,
}

/// Crea un nuevo producto o nuevos productos y lo envia a la base de datos.
/// A diferencia de los otros servicios REST, este recibe un archivo CSV en binario,
/// lee su contenido y genera una nueva transaccion hacia la tabla 'productos'
/// (El servicio falla si el proveedor de tal producto no existe en la base de datos)
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `producto` - Un json en el body del request representando el producto
#[post("/")]
pub async fn create(state: Data<AppState>, mut payload: Multipart) -> impl Responder {
    // Create a temporary file to store the uploaded CSV
    let mut temp_file = NamedTempFile::new().unwrap();

    // Read the payload stream into the temporary file
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            temp_file.write_all(&data).unwrap();
        }
    }

    // Reset the file cursor to the beginning
    temp_file
        .as_file_mut()
        .seek(std::io::SeekFrom::Start(0))
        .unwrap();

    // Create a CSV reader with a flexible configuration
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true) // Assumes the first row contains headers
        .from_reader(temp_file);

    let transaction = state.db.begin().await.unwrap();
    for (index, result) in csv_reader.records().enumerate() {
        match result {
            Ok(record) => {
                if let Ok(producto) = record.deserialize::<Productos>(None) {
                    if let Err(err) = sqlx::query_as::<_, Productos>(
                        "INSERT INTO productos VALUES ($1, $2, $3, $4, $5, $6);",
                    )
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
                        return HttpResponse::InternalServerError()
                            .body("Este csv contiene proveedores que no existen");
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
    HttpResponse::Ok().body("CSV file processed and data inserted into the table")
}

/// Crea un nuevo producto y lo envia a la base de datos.
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `producto` - Un json en el body del request representando el producto
#[post("/")]
pub async fn create_json(state: Data<AppState>, producto: Json<Productos>) -> impl Responder {
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
        Err(_) => HttpResponse::InternalServerError().json("could not create producto"),
    }
}

/// Obtiene todos los productos de la base de datos
/// ### Parametros
/// * `state` - La coneccion a la base de datos
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

/// Obtiene un producto de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[get("/{id}")]
pub async fn read_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("select * from productos where codigo = $1;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(producto) => HttpResponse::Ok().json(producto),
        Err(_) => HttpResponse::NotFound().json("producto not found"),
    }
}

/// Actualiza un producto de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
/// * `producto` - Un json en el body del request representando el producto a actualizar
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

/// Borra un producto de la base de datos, por medio de la id en la uri
/// ### Parametros
/// * `state` - La coneccion a la base de datos
/// * `path` - la uri relativa a la api, esto es la id
#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Productos>("delete from productos where codigo = $1;")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Producto deleted"),
        Err(_) => HttpResponse::InternalServerError().json("could not delete producto"),
    }
}

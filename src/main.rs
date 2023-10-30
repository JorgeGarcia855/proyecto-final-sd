mod entities;

use actix_cors::Cors;
use actix_web::{
    http::header,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::entities::*;

pub struct AppState {
    db: Pool<Sqlite>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://tienda-generica.db")
        .await
        .expect("could not connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8081")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/clientes")
                            .service(clientes::read_all)
                            .service(clientes::read_by_id)
                            .service(clientes::create)
                            .service(clientes::delete)
                            .service(clientes::update),
                    )
                    .service(
                        web::scope("/usuarios")
                            .service(usuarios::read_all)
                            .service(usuarios::read_by_id)
                            .service(usuarios::create)
                            .service(usuarios::delete)
                            .service(usuarios::update),
                    )
                    .service(
                        web::scope("/proveedores")
                            .service(proveedores::read_all)
                            .service(proveedores::read_by_id)
                            .service(proveedores::create)
                            .service(proveedores::delete)
                            .service(proveedores::update),
                    )
                    .service(
                        web::scope("/productos")
                            .service(productos::read_all)
                            .service(productos::read_by_id)
                            .service(productos::create)
                            .service(productos::delete)
                            .service(productos::update),
                    )
                    .service(
                        web::scope("/ventas")
                            .service(ventas::read_all)
                            .service(ventas::read_by_id)
                            .service(ventas::create)
                            .service(ventas::delete)
                            .service(ventas::update),
                    )
                    .service(
                        web::scope("/detalle_ventas")
                            .service(detalle_ventas::read_all)
                            .service(detalle_ventas::read_by_id)
                            .service(detalle_ventas::create)
                            .service(detalle_ventas::delete)
                            .service(detalle_ventas::update),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

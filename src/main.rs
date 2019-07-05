//! Actix web r2d2 example
extern crate postgres;
use std::io;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::Future;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager,TlsMode};

use uuid;
use dotenv;


/// Async request handler. Ddb pool is stored in application state.
fn index(
    path: web::Path<String>,
    db: web::Data<Pool<PostgresConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // execute sync code in threadpool
    web::block(move || {
        let conn = db.get().unwrap();

        let uuid = format!("{}", uuid::Uuid::new_v4());
        conn.execute(
            "INSERT INTO users (id, name) VALUES ($1, $2)",
            &[&uuid, &path.into_inner()],
        )
        .unwrap();

        conn.execute("SELECT name FROM users WHERE id=$1", &[&uuid])
    })
    .then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let sys = actix_rt::System::new("r2d2-example");

    // r2d2 pool
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let target = std::env::var("TARGET").expect("TARGET not defined");
    let manager = PostgresConnectionManager::new(url,TlsMode::None)
        .expect("Unable to connect to database");
    let pool = r2d2::Pool::new(manager).unwrap();

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            .route("/{name}", web::get().to_async(index))
    })
    .bind(target)?
    .start();

    sys.run()
}

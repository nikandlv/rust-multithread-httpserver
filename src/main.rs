extern crate postgres;
use std::io;

use actix_web::{middleware, App, HttpServer};
use r2d2_postgres::{PostgresConnectionManager,TlsMode};

use dotenv;

mod service;
mod router;
/// Async request handler. Ddb pool is stored in application state.


fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let sys = actix_rt::System::new("nifty-core-authentication");

    // r2d2 pool
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let target = std::env::var("TARGET").expect("TARGET not defined");
    let manager = PostgresConnectionManager::new(url,TlsMode::None)
        .expect("Unable to connect to database");
    let pool = r2d2::Pool::new(manager).unwrap();

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(router::get)
            .wrap(middleware::Logger::default())
    })
    .bind(target)?
    .start();

    sys.run()
}

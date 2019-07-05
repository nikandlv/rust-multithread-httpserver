use uuid;
use futures::Future;
use actix_web::{web, Error, HttpResponse};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

pub fn handle(
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

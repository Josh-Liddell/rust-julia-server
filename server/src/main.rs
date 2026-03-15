mod routes;
mod tasks;

use actix_web::{App, HttpServer, web};
use jlrs::prelude::*;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let (async_handle, _thread_handle) = Builder::new()
        .n_threads(4)
        .async_runtime(Tokio::<3>::new(false))
        .spawn()
        .expect("cannot init Julia");
    info!("julia runtime startup initiated");

    let handle = web::Data::new(async_handle);
    HttpServer::new(move || {
        App::new()
            .app_data(handle.clone())
            .service(routes::test)
            .service(routes::test2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod routes;
mod stop_handle;
mod tasks;

use actix_web::{App, HttpServer, middleware, web};
use anyhow::Result;
use jlrs::prelude::*;
use log::info;
use stop_handle::StopHandle;

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    info!("starting julia async runtime on a new thread");
    let (julia, thread_handle) = Builder::new()
        .n_threads(4)
        .async_runtime(Tokio::<3>::new(false))
        .spawn()
        .expect("cannot init Julia");

    // dispatch the including task to the julia runtime
    unsafe {
        let recv = julia
            // possible use env variable instead
            .include("/Users/joshua/Motoro.jl/Motoro/src/Motoro.jl")?
            .try_dispatch()
            .expect("runtime has shut down");

        let res = recv.await?;
        res.expect("include failed");
    }

    let handle = web::Data::new(julia.clone());
    let stop_handle = web::Data::new(StopHandle::default());

    info!("starting HTTP server at http://localhost:8080");
    let srv = HttpServer::new({
        let stop_handle = stop_handle.clone();
        move || {
            App::new()
                .wrap(middleware::Logger::default())
                .app_data(stop_handle.clone())
                .app_data(handle.clone())
                .service(routes::test)
                .service(routes::test2)
                .service(routes::number)
                .service(routes::stop)
        }
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    // register the server handle with the stop handle
    stop_handle.register(srv.handle());
    srv.await?;

    std::mem::drop(julia);
    thread_handle.join().expect("runtime thread panicked");

    Ok(())
}

// julia runtime is just not able to be interupted by a singal such as that from ctrl c

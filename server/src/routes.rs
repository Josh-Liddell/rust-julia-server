use crate::stop_handle::StopHandle;
use crate::tasks::{AdditionTask, GetNumber};
use actix_web::{HttpResponse, Responder, get, post, web};
use actix_web_lab::extract::Path;
use jlrs::runtime::handle::async_handle::AsyncHandle;

// route may receieve data such as with a post request, then execute a task on julia runtime

#[get("/get_test")]
async fn test(handle: web::Data<AsyncHandle>) -> impl Responder {
    // what if the task takes a long time??
    // right now it waits but it may be better to have it send an email with results when done or something
    let recv = handle
        .task(AdditionTask { a: 1.0, b: 2.0 })
        .try_dispatch()
        .expect("runtime has shut down");

    let res = recv.await.expect("cannot receive result");

    HttpResponse::Ok().body(format!("{res}"))
}

#[post("/post_test")]
async fn test2(
    req_body: web::Json<AdditionTask>,
    handle: web::Data<AsyncHandle>,
) -> impl Responder {
    let recv = handle
        .task(req_body.into_inner()) // { a: 1.0, b: 2.0 } for example
        .try_dispatch()
        .expect("runtime has shut down");

    let res = recv.await.expect("cannot receive result");

    HttpResponse::Ok().body(format!("{res}"))
}

#[post("/stop/{graceful}")]
async fn stop(Path(graceful): Path<bool>, stop_handle: web::Data<StopHandle>) -> HttpResponse {
    stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

#[get("/number")]
async fn number(handle: web::Data<AsyncHandle>) -> impl Responder {
    let recv = handle
        .task(GetNumber)
        .try_dispatch()
        .expect("runtime has shut down");

    let res = recv.await.expect("cannot receive result");

    HttpResponse::Ok().body(format!("{res}"))
}

// EXAMPLE
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

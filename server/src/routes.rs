use crate::tasks::AdditionTask;
use actix_web::{HttpResponse, Responder, get, post, web};
use jlrs::runtime::handle::async_handle::AsyncHandle;
use log::info;

// route may receieve data such as with a post request, then execute a task on julia runtime

#[get("/get_test")]
async fn test(handle: web::Data<AsyncHandle>) -> impl Responder {
    // what if the task takes a long time??
    // right now it waits but it may be better to have it send an email with results when done or something
    info!("Handling addition task request");
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
    info!("Handling addition task request");
    let recv = handle
        .task(req_body.into_inner()) // { a: 1.0, b: 2.0 } for example
        .try_dispatch()
        .expect("runtime has shut down");

    let res = recv.await.expect("cannot receive result");

    // println!("A request was hanldled");
    HttpResponse::Ok().body(format!("{res}"))
}

// EXAMPLE
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

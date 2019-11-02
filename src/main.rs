use actix_web::{http::header, web, App, HttpRequest, HttpServer, Responder, Result};
use rug::Integer;

fn index() -> impl Responder {
    "Hello, World".to_string()
}

fn factorial_iterative(n: u32) -> Integer {
    let mut result = Integer::from(n);
    for x in 2..n {
        result = result * x;
    }
    return result;
}

fn factorial_iter_handler(path: web::Path<(u32,)>) -> Result<String> {
    Ok(format!("{}", factorial_iterative(path.0)))
}

fn ip_address_handler(req: HttpRequest) -> impl Responder {
    if let Some(peer) = req.peer_addr() {
        peer.ip().to_string()
    } else {
        "Peer info is unavailable.".to_string()
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/factorial/{number}", web::get().to(factorial_iter_handler))
            .route("/peer", web::get().to(ip_address_handler))
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .unwrap();
}

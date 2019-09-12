use actix_web::{web, App, HttpServer, Responder, Result };
use rug::Integer;

fn index() -> impl Responder {
    "Hello, World".to_string()
}
fn factorial_iterative(n: u32) -> Integer {
    let mut result = Integer::from(n);
    for x in 2..n {
        result = result * x;
    }
    return result
}


fn factorial_iter_handler(path: web::Path<(u32,)>) -> Result<String> {
    Ok(format!("{}", factorial_iterative(path.0)))
}


fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/factorial/{number}", web::get().to(factorial_iter_handler))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}

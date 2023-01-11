mod api;

use api::task::{
    get_task,
}

use actix_web::{App, HttpServer, web::Data, middleware::Logger};

#[actix_web::main] 

async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(Data::new(get_task()))
    })
    .bind(("127.0.0.1", 80))? 
    .run()
    .await
}

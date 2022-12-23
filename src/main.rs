use actix_web::{HttpServer, App, web};

use log::info;
use std::env;

mod utils;
mod db;
mod handlers;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    env_logger::init();
    //let mut handlebars = Handlebars::new();

    let (addr, port) = (
        &env::var("SERVER_ADDR").expect("Unable to find server address env var."),
        &env::var("SERVER_PORT").expect("Unable to find server port env var.")
        );
    info!("Registered host at {}:{}.",addr,port);

    let host_sock = utils::parse_env_sockargs(addr, port);

    // Get the Kluge Render 
    let index_render = utils::KlugeRender::new(
        "src/templates/index_header.html", "src/templates/index_footer.html"
        )
        .unwrap();
    let index_render = web::Data::new(index_render);

    // Get a "DB" ready
    let data = web::Data::new(db::db_init());
    

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(index_render.clone())
            .service(handlers::index)
            .service(handlers::submit_order)
            .service(handlers::create_order)
            .service(handlers::finish)
            .service(handlers::finish_order)
    })
    .bind(host_sock)?
    .run()
    .await
}

use actix_web::{get, post, web, Responder, HttpResponse}; 

use crate::db::{Db, Order, SubmitOrder};
use crate::utils::Kr;
use uuid::Uuid;

#[get("/")]
pub async fn index(data: web::Data<Db>, kr: web::Data<Kr>) -> impl Responder {
    //let orders = data.read().unwrap();
    //let html = kr.index_html(orders).unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().body(list_orders(data, kr))
}

fn list_orders(data: web::Data<Db>, kr: web::Data<Kr>) -> String {
    let orders = data.read().unwrap();
    let html = kr.index_html(orders).unwrap_or_else(|e| e.to_string());
    html
}

#[get("/submit/order")]
pub async fn submit_order() -> impl Responder {
 
    let contents = std::fs::read_to_string("./src/templates/order_header.html").unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().body(contents)
}

// form: web::Form<SubmitOrder>

#[post("/create/order")]
pub async fn create_order(data: web::Data<Db>, form: web::Form<SubmitOrder>) -> impl Responder {
  
    let mut orders = data.write().unwrap();
    let order = Order::new(&form, "ethan");
    orders.insert(order.id(), order);
    let contents = std::fs::read_to_string("./src/templates/order_submitted.html").unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().body(format!("{}",contents))
}

#[get("/finish")]
pub async fn finish(data: web::Data<Db>, kr: web::Data<Kr>) -> impl Responder {
    //let orders = data.read().unwrap();
    //let html = kr.finish_html(orders).unwrap_or_else(|e| e.to_string());
    HttpResponse::Ok().body(list_orders_finish(data, kr))
}

fn list_orders_finish(data: web::Data<Db>, kr: web::Data<Kr>) -> String {
    let orders = data.read().unwrap();
    let html = kr.finish_html(orders).unwrap_or_else(|e| e.to_string());
    html
}
#[get("/finish/{shard0}/{shard1}")]
pub async fn finish_order(data: web::Data<Db>, kr: web::Data::<Kr>, path: web::Path<(u64,u64)>) -> impl Responder {
    {
        let mut orders = data.write().unwrap();
        let (hi,lo) = path.into_inner();
        let id = Uuid::from_u64_pair(hi,lo);
     
        if let Some(order) = orders.get_mut(&id.as_u128()) {
            order.ready();
        }
    }
   
    HttpResponse::Ok().body(list_orders_finish(data, kr))
}

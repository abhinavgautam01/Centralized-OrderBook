use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web::{Data, Json}, HttpResponse, Responder};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::Orderbook};



#[post("/order")]
pub async fn create_order(order: Json<CreateOrderInput>, orderbook:Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    // let price = order.0.price;
    // let quantity = order.0.quantity;
    // let user_id = order.0.user_id;
    // let side = order.0.side;

    // let mut orderbook = orderbook.lock().unwrap();
    // orderbook.create_order(price, quantity, user_id, side);

    // return HttpResponse::Ok().json(CreateOrderResponse {
    //     order_id: String::from("order1")
    // });
    println!("{:?}", order);
    let mut orderbook = orderbook.lock().unwrap();
    let orderbook = orderbook.create_order(order.0);
    HttpResponse::Ok().json(orderbook)
}


#[delete("/cancel")]
pub async fn cancel_order(Json(order): Json<DeleteOrder>, orderbook:Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    // let _order_id = order.order_id;

    // return HttpResponse::Ok().json(DeleteOrderResponse {
    //     filled_qty: 0,
    //     average_price: 0
    // });

    let mut orderbook = orderbook.lock().unwrap();
    let orderbook = orderbook.delete_order(order);
    HttpResponse::Ok().json(orderbook)
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    // HttpResponse::Ok().json(Depth {
    //     bids: vec![],
    //     asks: vec![],
    //     last_update_id: String::from("")
    // })
    let orderbook = orderbook.lock().unwrap();
    let depth = orderbook.get_depth();
    HttpResponse::Ok().json(depth)
}
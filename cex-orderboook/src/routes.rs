use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web::{Data, Json}, HttpResponse, Responder};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::Orderbook, output::{CreateOrderResponse, DeleteOrderResponse, Depth}};



#[post("/order")]
pub async fn create_order(order: Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    println!("Creating order: {:?}", order);
    
    let mut orderbook_guard = orderbook.lock().unwrap();
    let order_id = orderbook_guard.order_id_index.to_string();
    orderbook_guard.create_order(order.0);
    
    HttpResponse::Ok().json(CreateOrderResponse {
        order_id
    })
}


#[delete("/cancel")]
pub async fn cancel_order(Json(order): Json<DeleteOrder>, orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    println!("Cancelling order: {:?}", order);
    
    let mut orderbook_guard = orderbook.lock().unwrap();
    orderbook_guard.delete_order(order);
    
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 0
    })
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    let orderbook_guard = orderbook.lock().unwrap();
    let depth_response = orderbook_guard.get_depth();
    
    // Convert DepthResponse to Depth format expected by output module
    let depth = Depth {
        bids: depth_response.bids.iter().map(|d| [d.price, d.quantity]).collect(),
        asks: depth_response.asks.iter().map(|d| [d.price, d.quantity]).collect(),
        last_update_id: "1".to_string()
    };
    
    HttpResponse::Ok().json(depth)
}

use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};

mod models;
use crate::models::BuyPizzaRequest;

#[get("/pizza")]
 async fn get_pizza()->impl Responder{
    HttpResponse::Ok().body("pizza available ")
 }

 #[post("/buypizza")]
 async fn buy_pizza(body :Json<BuyPizzaRequest>)-> impl Responder{
    let is_valid= body.validate();
    match is_valid{
        Ok(_)=>{
            let pizza_name =body.pizza_name.clone();
            HttpResponse::Ok().body(format!(" pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("pizza name required"),
    }
  

 }
 #[patch("updatePIzza/{uuid}")]
 async fn update_pizza()-> impl Responder{
    HttpResponse::Ok().body("updating pizza")

 }

 #[actix_web::main]

async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(get_pizza)
        .service(buy_pizza)
        .service(update_pizza)
    } )
    .bind("127.1.1.0:8080")?
    .run()
    .await
    
}

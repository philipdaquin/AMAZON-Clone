use actix_web::{HttpResponse, web};
use stripe::{*, Currency, Client, Charge, RequestError, StripeError};

#[derive(Debug, Deserialize)]
pub struct Total { 
    subtotal: i64
}   

pub async fn handle_stripe(web::Query(info): web::Query<Total>) -> Result<HttpResponse, HttpResponse> { 
    let stripe_secret = 
        std::env::var("STRIPE_SECRET_KEY")
        .expect("No Stripe Payment Provided"); 
    format!("Payment Request Received {}", info.subtotal);
    let client = Client::new(stripe_secret);
    let payment_intent = stripe::CreateCharge::new();
    let CreateCharge { mut amount, mut currency, ..} = payment_intent;
    
    amount = Some(info.subtotal);
    currency = Some(Currency::AUD);

    let charge = Charge::create(&client, payment_intent).await;
    match charge { 
        Ok(x) => Ok(HttpResponse::Ok().json(x)),
        Err(e) => Ok(HttpResponse::Ok().body(format!("{:?}", e)))
    }
    
    
}
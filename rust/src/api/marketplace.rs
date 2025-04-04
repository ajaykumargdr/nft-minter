use std::sync::Mutex;

use super::types::*;
use super::{GTKContract, HttpResponse, Responder, web};

// Todo - move to db
static LISTINGS: Mutex<Vec<ListingInfo>> = Mutex::new(Vec::new());

#[actix_web::post("/list")]
pub async fn list(listing_info: web::Json<ListingInfo>) -> impl Responder {
    // Todo - make sure the sender and owner of the token_id are same ***

    LISTINGS.lock().unwrap().push(listing_info.0);
    HttpResponse::Ok()
}

#[actix_web::get("/listings")]
pub async fn get_listings(_contract: web::Data<GTKContract>) -> impl Responder {
    let listing: Vec<ListingInfo> = LISTINGS.lock().unwrap().iter().map(|v| v.clone()).collect();

    HttpResponse::Ok().json(listing)
}

#[actix_web::post("/buy/{listing_id}")]
pub async fn buy(
    _contract: web::Data<GTKContract>,
    _listing_id: web::Path<String>,
) -> impl Responder {
    // Todo - Transfer ownership of the token
    // Todo - Transfer fee from the buyer to owner.  
    HttpResponse::NotImplemented().finish()
}

#[actix_web::put("/updateListing/{listing_id}")]
pub async fn update_listing(
    _contract: web::Data<GTKContract>,
    _listing_id: web::Path<String>,
    _listing_info: web::Json<ListingInfo>,
) -> impl Responder {
    // Todo - check the ownership of the token and update the listing
    HttpResponse::NotImplemented().finish()
}

#[actix_web::delete("/cancelListing/{listing_id}")]
pub async fn cancel_listing(
    _contract: web::Data<GTKContract>,
    _listing_id: web::Path<String>,
) -> impl Responder {
    // Todo - check the ownership of the token and remove token from the listing
    HttpResponse::NotImplemented().finish()
}

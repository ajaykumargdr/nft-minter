use super::{
    authentication::AuthenticationGuard,
    types::{ActixContext, BidInfo, ListingInfo},
};
use actix_web::{HttpResponse, Responder, web};
use std::sync::Mutex;

// Todo - move to db
static LISTINGS: Mutex<Vec<ListingInfo>> = Mutex::new(Vec::new());

#[actix_web::post("/list")]
pub async fn list(
    auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    listing_info: web::Json<ListingInfo>,
) -> impl Responder {
    match context.contract.owner_of_token(listing_info.token_id).await {
        Ok(token_owner) => {
            if token_owner != auth_guard.user.wallet_address {
                return HttpResponse::Unauthorized().finish();
            }
        }
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    };

    let mut listings = LISTINGS.lock().unwrap();

    if listings
        .iter_mut()
        .find(|l| l.token_id == listing_info.token_id)
        .is_some()
    {
        return HttpResponse::Conflict().finish();
    }

    listings.push(listing_info.0);
    HttpResponse::Ok().finish()
}

#[actix_web::get("/listings")]
pub async fn get_listings(
    _auth_guard: AuthenticationGuard,
    _context: web::Data<ActixContext>,
) -> impl Responder {
    let listing: Vec<ListingInfo> = LISTINGS.lock().unwrap().iter().map(|v| v.clone()).collect();

    HttpResponse::Ok().json(listing)
}

#[actix_web::post("/bid/{token_id}")]
pub async fn bid(
    auth_guard: AuthenticationGuard,
    _context: web::Data<ActixContext>,
    mut input: web::Json<BidInfo>,
    token_id: web::Path<usize>,
) -> impl Responder {
    let token_id = token_id.into_inner();

    let mut listings = LISTINGS.lock().unwrap();
    let listing = listings.iter_mut().find(|l| l.token_id == token_id);

    match listing {
        Some(listing_info) => {
            input.bidder = auth_guard.user.wallet_address;
            listing_info.bids.push(input.into_inner());
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::put("/updateListing")]
pub async fn update_listing(
    auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    listing_info: web::Json<ListingInfo>,
) -> impl Responder {
    match context.contract.owner_of_token(listing_info.token_id).await {
        Ok(token_owner) => {
            if token_owner != auth_guard.user.wallet_address {
                return HttpResponse::Unauthorized().finish();
            }
        }
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    };

    let mut listings = LISTINGS.lock().unwrap();
    let listing = listings
        .iter_mut()
        .find(|l| l.token_id == listing_info.token_id);

    match listing {
        Some(listing) => {
            listing.price = listing_info.price;
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::delete("/cancelListing/{token_id}")]
pub async fn cancel_listing(
    auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    token_id: web::Path<usize>,
) -> impl Responder {
    let token_id = token_id.into_inner();

    match context.contract.owner_of_token(token_id).await {
        Ok(token_owner) => {
            if token_owner != auth_guard.user.wallet_address {
                return HttpResponse::Unauthorized().finish();
            }
        }
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    };

    let mut listings = LISTINGS.lock().unwrap();
    let removed = listings.iter().position(|l| l.token_id == token_id);

    match removed {
        Some(index) => {
            listings.remove(index);
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

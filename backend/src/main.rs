use axum::{
    extract::Path,
    http::Method,
    response::{IntoResponse, Response},
    Json,
    routing::get,
    Router,
};
use serde::Serialize;
use tower_http::cors::{CorsLayer, Any};
use db::{get_products_in_stock, establish_connection, models::Product};
use axum::debug_handler;


#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[debug_handler]
async fn get_all_products() -> ApiResponse<Vec<Product>> {
    let conn = &mut establish_connection();
    match get_products_in_stock(conn) {
        Ok(products) => ApiResponse {
            success: true,
            data: products,
        },
        Err(_) => ApiResponse {
            success: false,
            data: vec![],
        },
    }
}

async fn root() -> &'static str {
    "Hello from Rust backend!"
}
#[tokio::main]
async fn main() {

        let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);


    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_all_products))
        .layer(cors);
        
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server on localhost:3000");
    axum::serve(listener, app).await.unwrap();
    
}
mod controllers;
mod dto;

use controllers::user_controller;
use dto::{CustomAppData, User};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let custom_app_data = web::Data::new(get_app_data().await);

    HttpServer::new(move || {
        App::new()
            .app_data(custom_app_data.clone())
            .service(user_controller::get_user)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn get_app_data() -> CustomAppData {
    let database: Vec<User> = vec![
        User::new(0, "name1", 1),
        User::new(1, "name1", 2),
        User::new(2, "name1", 3),
        User::new(3, "name1", 4),
    ];

    CustomAppData::new(database)
}

use actix_web::{get, web, HttpResponse};

use crate::dto::{CustomAppData, User};

#[get("/users/{id}")]
pub async fn get_user(
    web::Path(id): web::Path<u32>,
    data: web::Data<CustomAppData>,
) -> HttpResponse {
    let user: &User = match &data.database.iter().find(|user| user.id == id) {
        Some(user) => user,
        None => {
            return HttpResponse::NotFound().body("User not found");
        }
    };

    HttpResponse::Ok().json(user)
}

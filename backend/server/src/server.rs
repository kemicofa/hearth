use actix_web::{ App, HttpServer, web };

use crate::{ bootstrap::build_dependencies, routes::create_user_handler };

pub async fn build_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let dependencies = web::Data::new(build_dependencies());
        App::new().app_data(dependencies).service(create_user_handler)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}

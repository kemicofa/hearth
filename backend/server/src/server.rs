use actix_web::{App, HttpServer, web};

use crate::{bootstrap::Dependencies, routes::signup_email_handler};

pub async fn build_server(dependencies: Dependencies, port: u16) -> std::io::Result<()> {
    // Data is a wrapper around Arc, so we can clone it.
    let data = web::Data::new(dependencies);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(signup_email_handler)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

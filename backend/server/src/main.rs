use crate::{ server::build_server };

mod bootstrap;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    build_server().await
}

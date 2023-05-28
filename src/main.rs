mod server;
mod http;
use http::Methods;
use http::Request;
use server::index::Server;


fn main() {
    let method = Methods::DELETE;
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.connect();
}
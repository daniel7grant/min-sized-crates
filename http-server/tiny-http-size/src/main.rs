use std::error::Error;

use tiny_http::{Response, Server};

fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::http("0.0.0.0:3000").unwrap();
    println!("Now listening on port 3000");

    for req in server.incoming_requests() {
        req.respond(Response::from_string("Hello, World!"))?;
    }

    Ok(())
}

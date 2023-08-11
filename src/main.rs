mod config;
mod http;
mod path;
mod serve;

use config::ServerConfig;
use http::{response::{Response, ResponseBuilder}, Status};
use path::PathMatch;
use serve::ServeDir;
use httparse::Request;
use std::{rc::Rc, collections::HashMap};


struct State {
    serve_dir: ServeDir,
    redirects: HashMap<String, String>,
    ignored: PathMatch<()>
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::load()?;

    println!("Hosting {:?} at \x1b[94mhttp://{:?}\x1b[0m", config.dir, config.address);

    let state = State {
        serve_dir: ServeDir::new(config.dir, config.routes),
        redirects: config.redirects,
        ignored: config.ignored
    };

    http::Server::bind(config.address)?
        .serve_with_state(Box::new(handler), Rc::new(state))?;

    Ok(())
}


fn handler(state: Rc<State>, req: Request) -> Option<Response> {
    match req.method? {
        "GET" => {
            let path = req.path.unwrap();

            if let Some(redir) = state.redirects.get(path) {
                Some(ResponseBuilder::new()
                    .status(Status::TemporaryRedirect)
                    .header("Location", redir)
                    .into_response())
            }
            else if state.ignored.contains(path) {
                None
            }
            else {
                Some(state.serve_dir.serve(path))
            }
        },
        _ => {
            Some(ResponseBuilder::new()
                .status(Status::MethodNotAllowed)
                .header("Allow", "GET")
                .into_response())
        }
    }
}

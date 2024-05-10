use bytes::Bytes;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use route_recognizer::Params;
use router::Router;
use std::sync::Arc;

use std::env;
use std::process::exit;

mod handler;
mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

#[tokio::main]
async fn main() {
    let mut port = 8080; // Default port
    for arg in env::args().skip(1) {
        if arg.starts_with("--port=") || arg.starts_with("-p=") {
            let port_str = arg.splitn(2, '=').nth(1).unwrap();
            let parsed_port = port_str.parse::<u16>();
            if let Err(err) = parsed_port {
                eprintln!("Error parsing port: {}", err);
                exit(1);
            }
            port = parsed_port.unwrap();
            break; // Exit after finding the port argument
        }
    }

    println!("Using port: {}", port);
    let str_port = port.to_string();
    println!("The string value is: {}", str_port);

    println!("\ncurl http://localhost:{}/test\n", &str_port);
    println!("curl http://localhost:{}/params/1234\n", &str_port);
    println!(
        "curl -X POST http://localhost:{}/send -d '{{\"name\": \"chip\", \"active\": true}}'\n\n",
        &str_port
    );

    let some_state = "state".to_string();

    let mut router: Router = Router::new();
    router.get("/test", Box::new(handler::test_handler));
    router.post("/send", Box::new(handler::send_handler));
    router.get("/params/:some_param", Box::new(handler::param_handler));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    let _ = server.await;
}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::process::Command;

    #[test]
    fn curl_test() {
        let url = "http://localhost:8080/test";
        let mut command = Command::new("curl");
        command.arg(url);

        // Capture output (optional)
        let output = command.output().unwrap();
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    }
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: -1\n right: 3")]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}

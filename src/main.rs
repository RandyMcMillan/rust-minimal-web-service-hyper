use crate::context::*;
use crate::route::route;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use router::Router;
use std::env;
use std::process::exit;
use std::sync::Arc;

mod context;
mod handler;
mod router;
mod route;

use sysinfo::{get_current_pid, Pid, System};

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() {
    let mut port = 8080; // Default port
    let mut help = false;
    let mut verbose = false;
    #[allow(unused_assignments)]
    let mut port_str: &str = "8080";
    let mut assign_next = false;
    for arg in env::args().skip(1) {
        if assign_next {
            port = arg.parse::<u16>().unwrap();
            break;
        }
        if arg.starts_with("--help") || arg.starts_with("-h") {
            help = true;
        }
        if arg.starts_with("--verbose") || arg.starts_with("-vv") {
            verbose = true;
        }
        if arg.starts_with("--port=") || arg.starts_with("-p=") {
            port_str = arg.splitn(2, '=').nth(1).unwrap();
            let parsed_port = port_str.parse::<u16>();
            if let Err(err) = parsed_port {
                eprintln!("Error parsing port: {}", err);
                exit(1);
            }
            port = parsed_port.unwrap();
            break; // Exit after finding the port argument
        }
        if arg.starts_with("--port") || arg.starts_with("-p") {
            assign_next = true;
        }
    }

    let str_port = port.to_string();
    let mut some_state = "state".to_string();
    if help {
        some_state = "help".to_string();
        println!("curl http://localhost:{}/help", &str_port);
    }
    if verbose {
        some_state = "verbose".to_string();
        if !help {
            println!("curl http://localhost:{}/help", &str_port);
        }
        println!("curl http://localhost:{}/test", &str_port);
        println!("curl http://localhost:{}/params/1234", &str_port);
        println!(
            "curl -X POST http://localhost:{}/send -d '{{\"name\": \"chip\", \"active\": true}}'",
            &str_port
        );
    }

    let mut router: Router = Router::new();
    router.get("/help", Box::new(handler::help));
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
    match get_current_pid() {
        Ok(pid) => {
            let s = System::new_all();
            if let Some(process) = s.process(Pid::from(pid)) {
                println!(
                    "{{\"{}\",\"{}\",\"{}\",\"{}\"}}",
                    process.name(),
                    pid,
                    addr,
                    port
                );
            }
        }
        Err(e) => {
            println!("failed to get current pid: {}", e);
        }
    }
    let _ = server.await;
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
    //use super::*;

    use std::process::Command;

    #[test]
    fn curl_test() {
        let mut url = "http://localhost:8080/help";
        let mut command = Command::new("curl");
        let s_flags = "-sS";
        let d_flag = "-d";
        let data = r#"{"name":"John Doe","active":true,"phones":["+44 1234567","+44 2345678"]}"#;

        println!("{:?}", url);
        command.args([url, s_flags]);
        let mut output = command.output().unwrap();
        println!("{:?}", output);
        url = "http://localhost:8080/test";
        println!("{:?}", url);
        command = Command::new("curl");
        command.args([url, s_flags]);
        output = command.output().unwrap();
        println!("{:?}", output);

        url = "http://localhost:8080/params/1234";
        println!("{:?}", url);
        command = Command::new("curl");
        command.args([url, s_flags]);
        output = command.output().unwrap();
        println!("{:?}", output);

        url = "http://localhost:8080/send";
        println!("{:?}", url);
        // data include extraneous data for testing
        println!("{:?}", data);
        command = Command::new("curl");
        command.args([url, s_flags, d_flag, data]);
        output = command.output().unwrap();
        print!("{:?}\n", output);
    }
}

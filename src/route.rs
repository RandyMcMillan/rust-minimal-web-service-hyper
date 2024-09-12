use crate::context::{AppState, Context};
use crate::router::Router;
use hyper::Request;
use std::sync::Arc;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn route(
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

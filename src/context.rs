use bytes::Bytes;
use hyper::{
    body::to_bytes,
    Body, Request,
};
use route_recognizer::Params;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
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

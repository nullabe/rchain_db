use crate::http_api::node::Node;
use tide::{Request, Response};

impl Node {
    pub fn routing(&mut self) {
        self.root_action();
    }

    fn root_action(&mut self) {
        self.get_server()
            .at("/")
            .get(|mut _request: Request<()>| async move {
                Response::new(200).body_string(String::from("test"))
            });
    }
}

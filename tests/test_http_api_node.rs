// #[cfg(test)]
// pub mod test_http_api_node {
//     use futures_await_test::async_test;
//
//     use rchain_db::http_api::node::Node;
//     use std::error::Error;
//     use std::thread::spawn;
//
//     #[async_test]
//     pub async fn test_node() -> Result<(), Box<dyn Error>> {
//         spawn(|| async move {
//             Node::run("127.0.0.1", "7878").await.unwrap()
//         });
//
//         let resp = reqwest::get("http://127.0.0.1:7878")
//             .await?
//             .text()
//             .await?
//             ;
//
//         assert_eq!("1", resp);
//
//         Ok(())
//     }
// }

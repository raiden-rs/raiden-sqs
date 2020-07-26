use raiden_sqs::*;
use serde::Serialize;

#[derive(RaidenSqs, Serialize, Debug)]
#[sqs(queue_name = "MyFirstQueue")]
#[sqs(endpoint = "http://127.0.0.1:9324")]
#[sqs(region = "ap-northeast-1")]
pub struct MyMessage {
    name: String,
    greetings: String,
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    async fn run() {
        let msg = MyMessage {
            name: "kuy".into(),
            greetings: "Hello, SQS!".into(),
        };
        let res = msg.send().await.unwrap();
        println!("Sent: message_id={}", res);
    }
    rt.block_on(run());
}

use raiden_sqs::*;
use serde::Serialize;

#[derive(RaidenCloudPubSub, Serialize)]
pub struct GreetingMessage {
    name: String,
}

fn main() {}

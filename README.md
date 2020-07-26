<p align="center"><img src ="https://github.com/raiden-rs/raiden-sqs/blob/master/assets/raiden-sqs.png?raw=true" /></p>

<p align="center">
    Ergonomic SQS library for Rust.
</p>

---

![CI](https://github.com/raiden-rs/raiden-sqs/workflows/CI/badge.svg)

## Status

Underdevelopment. No working examples.

## Development

### Test

```
cd test-suite
cargo test
```

## Supported APIs

- [ ] AddPermission
- [ ] ChangeMessageVisibility
- [ ] ChangeMessageVisibilityBatch
- [ ] CreateQueue
- [ ] DeleteMessage
- [ ] DeleteMessageBatch
- [ ] DeleteQueue
- [ ] GetQueueAttributes
- [ ] GetQueueUrl
- [ ] ListDeadLetterSourceQueues
- [ ] ListQueues
- [ ] ListQueueTags
- [ ] PurgeQueue
- [ ] ReceiveMessage
- [ ] RemovePermission
- [ ] SendMessage
- [ ] SendMessageBatch
- [ ] SetQueueAttributes
- [ ] TagQueue
- [ ] UntagQueue

## Code (not working example)

```rust
use raiden_core::*;
use raiden_sqs::*;

#[derive(Sqs, Serialize, Debug)]
#[sqs(queue_name = "MyFirstQueue", region = "us-east-1")]
pub struct MyMessage {
    name: String,
    greetings: String,
}

#[derive(Sqs, Serialize, Debug)]
#[sqs(queue_name = "MoreQueue", ops_prefix = "sqs_", ops_suffix = "_message", region = "us-east-1")]
pub struct MoreMessage {
    name: String,
    more: String,
}

#[derive(Sqs, Serialize, Debug)]
#[sqs(queue_name = "MinimumQueue", ops = ("receive", "delete"))]
pub struct MinimumMessage {
    name: String,
    ops: String,
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    async fn run() {
        // Send a message
        let res = MyMessage {
            name: "kuy".into(),
            greetings: "Hello, SQS!".into(),
        }.send().await.unwrap();
        println!("Sent: message_id={}", res.message_id);

        // Receive a message
        let res = MyMessage::receive().await.unwrap();
        println!("Received: {:?}", res.message);

        // Delete a message
        res.delete().await;

        // Send messages in batch
        let messages = vec![...];
        let ret = MyMessage::batch_send(&messages).await.unwrap();

        // Delete messages in batch
        let handles = vec![...];
        let ret = MyMessage::batch_delete(&handles).await.unwrap();

        // Send a message with delay
        MyMessage {
            name: "kuy".into(),
            greetings: "Goodbye, SQS!".into(),
        }.send_with().delay(30).await.unwrap();

        // Send a message with attribute/meta data
        MyMessage {
            name: "kuy".into(),
            greetings: "Goodbye, SQS!".into(),
        }.send_with().attr("System", "Solar").attr("Universe", 42).await.unwrap();

        // Send a plain text message
        MyMessage::create("I'm not JSON".into()).send().await.unwrap();
        MyMessage::create("Delayed message".into()).send_with().delay(30).await.unwrap();

        // Send a message with overriding config
        MyMessage {
            name: "kuy".into(),
            greetings: "Hello, Tokyo!".into(),
        }.send_with().config(&opt).await.unwrap();

        // Rename operation methods to avoid conflict
        let res = MoreMessage {
            name: "kuy".into(),
            more: "YES IS MORE".into(),
        }.sqs_send_message().await.unwrap();

        let res = MoreMessage::sqs_receive_message().await.unwrap();
    }
    rt.block_on(run());
}
```

## Memo

- Strategy for error handling and retry policy
- Derive `Into` trait to convert struct to String
- Delay
- Batch ops
- Support simple client to send plain text
- Type checking for FIFO
- Message attributes
- Automatic deletion (waiting for async Drop trait)
- Feature flag to disable serialize/deserialize

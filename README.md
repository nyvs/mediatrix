# mediatrix
[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]
[![Dependencies][deps]][github]

Strongly typed, extensible event mediator.
For more info and explanation, please see the [docs][docs-rs-url].

## Usage
### Sync
```rust
use mediatrix::synchronous::basic::*;

struct UserMessageRequest {
    msg: String,
    priority: u8,
}

#[derive(Debug)]
enum NotifyEvent {
    Ignore,
    SendEmail(String),
    SendTextMessage(String),
}

impl RequestHandler<UserMessageRequest, NotifyEvent> for BasicMediator<NotifyEvent> {
    fn handle(&self, req: UserMessageRequest) {
        match req.priority {
            0 => self.publish(NotifyEvent::Ignore),
            1..=5 => self.publish(NotifyEvent::SendEmail(req.msg)),
            _ => self.publish(NotifyEvent::SendTextMessage(req.msg)),
        };
    }
}

let mediator = BasicMediator::<NotifyEvent>::builder()
    .add_listener(move |ev: &NotifyEvent| {
        if let NotifyEvent::Ignore = ev {
            println!("Ignored some Message")
        }
    })
    .add_listener(move |ev: &NotifyEvent| {
        if let NotifyEvent::SendEmail(msg) = ev {
            println!("Send Email with Message: {}", msg)
        }
    })
    .add_listener(move |ev: &NotifyEvent| {
        if let NotifyEvent::SendTextMessage(msg) = ev {
            println!("Send SMS with Message: {}", msg)
        }
    })
    .build();

mediator.send(UserMessageRequest {
    msg: String::from("Hello World"),
    priority: 0,
});

mediator.send(UserMessageRequest {
    msg: String::from("Is Rust Memory Safe?"),
    priority: 2,
});

mediator.send(UserMessageRequest {
    msg: String::from("New Rust Version"),
    priority: 8,
});

// Prints: Ignored some Message
mediator.next().ok();
// Prints: Send Email with Message: Is Rust Memory Safe?
mediator.next().ok();
// Prints: Send SMS with Message: New Rust Version
mediator.next().ok();
```

### Async
<details>
<summary>Click to open the asynchronous version</summary>

```rust
use mediatrix::asynchronous::basic::*;
use async_trait::async_trait;

struct UserMessageRequest {
    msg: String,
    priority: u8,
}

#[derive(Debug)]
enum NotifyEvent {
    Ignore,
    SendEmail(String),
    SendTextMessage(String),
}

#[async_trait]
impl AsyncRequestHandler<UserMessageRequest, NotifyEvent> for BasicAsyncMediator<NotifyEvent> {
    async fn handle(&self, req: UserMessageRequest) {
        match req.priority {
            0 => self.publish(NotifyEvent::Ignore).await,
            1..=5 => self.publish(NotifyEvent::SendEmail(req.msg)).await,
            _ => self.publish(NotifyEvent::SendTextMessage(req.msg)).await,
        };
    }
}

let async_mediator = BasicAsyncMediator::<NotifyEvent>::builder()
    .add_listener(move |ev: &NotifyEvent| {
        if let NotifyEvent::Ignore = ev {
            println!("Ignored some Message")
        }
    })
    .add_listener(move |ev: &NotifyEvent| {
        if let NotifyEvent::SendEmail(msg) = ev {
            println!("Send Email with Message: {}", msg)
        }
    })
    .add_listener(move |ev: &NotifyEvent| {
        if let NotifyEvent::SendTextMessage(msg) = ev {
            println!("Send SMS with Message: {}", msg)
        }
    })
    .build();

async_std::task::block_on(async {
    async_mediator.send(UserMessageRequest {
        msg: String::from("Hello World"),
        priority: 0,
    }).await;

    async_mediator.send(UserMessageRequest {
        msg: String::from("Is Rust Memory Safe?"),
        priority: 2,
    }).await;

    async_mediator.send(UserMessageRequest {
        msg: String::from("New Rust Version"),
        priority: 8,
    }).await;

    async_mediator.next().await.ok();
    async_mediator.next().await.ok();
    async_mediator.next().await.ok();
});
```

</details>

## Features
- sync and async (use `async` feature) mediators 
- `CxAwareMediator` (use `async` feature, carries a struct of your choice)
- compiler-baked typing
- extensible architecture

## Todo
- internally, make builders reuse other builders whose target is a comp. of this builder.

## Contributions
Feel free to open an issue/PR explaining possible improvements or changes.

## Help
Also, please do not hesitate and open an issue when needed. I am happy to help!

[deps]: https://img.shields.io/librariesio/github/nyvs/mediatrix
[github]: https://github.com/nyvs/mediatrix
[crates-io-badge]: https://img.shields.io/crates/v/mediatrix.svg
[crates-io-url]: https://crates.io/crates/mediatrix
[docs-rs-img]: https://docs.rs/mediatrix/badge.svg
[docs-rs-url]: https://docs.rs/mediatrix

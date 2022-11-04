use async_trait::async_trait;
use std::{fmt::Debug, sync::mpsc::TryRecvError};

/// Publish an event `Ev` asynchronously from within a handler.
#[async_trait]
pub trait AsyncMediatorInternal<Ev: Debug> {
    #[allow(missing_docs)]
    async fn publish(&self, event: Ev);
}

/// Send a request `Req` asynchronously for processing to the mediator.
/// This will call the handler.
#[async_trait]
pub trait AsyncMediatorInternalHandle<Ev: Debug> {
    #[allow(missing_docs)]
    async fn send<Req>(&self, req: Req)
    where
        Req: Send,
        Self: AsyncRequestHandler<Req, Ev>;
}

/// Process the next event `Ev` from the channel asynchronously.
/// This will call all listeners with a `&Ev`.
#[async_trait]
pub trait AsyncMediatorInternalNext {
    #[allow(missing_docs)]
    async fn next(&self) -> Result<(), TryRecvError>;
}

/// Handles the request `Req` asynchronously.
/// Implemented by the user.
#[async_trait]
pub trait AsyncRequestHandler<Req, Res>
where
    Self: Sync,
{
    #[allow(missing_docs)]
    async fn handle(&self, req: Req);
}

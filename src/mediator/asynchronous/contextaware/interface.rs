use async_trait::async_trait;
use std::fmt::Debug;

/// Send a request `Req` asynchronously for processing to the mediator.
/// This will call the handler.
/// The handler here is context-dependent.
#[async_trait]
pub trait CxAwareAsyncMediatorInternalHandle<Cx, Ev: Debug> {
    #[allow(missing_docs)]
    async fn send<Req>(&self, req: Req)
    where
        Req: Send,
        Self: CxAwareAsyncRequestHandler<Cx, Req, Ev>;
}

/// Handles the request `Req` asynchronously.
/// Implemented by the user.
/// Gives access to the context `Cx`.
#[async_trait]
pub trait CxAwareAsyncRequestHandler<Cx, Req, Res> {
    #[allow(missing_docs)]
    async fn handle(&self, req: Req, cx: &Cx);
}

/// Advanced builder fuctionality:
/// Adding a context `cx` to the builder.
pub trait CxAwareMediatorBuilderInterface<M, Cx, Ev> {
    #[allow(missing_docs)]
    fn add_context(self, cx: Cx) -> Self
    where
        Ev: Debug;
}

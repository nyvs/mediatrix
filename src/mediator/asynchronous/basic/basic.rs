use std::sync::mpsc::TryRecvError;

use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fmt::Debug;

use super::*;
use crate::synchronous::basic::{BasicMediator, SyncMediatorInternal, SyncMediatorInternalNext};

/// Basic async mediator for asynchronous environments with events of type `Ev`.
///
/// A [`BasicAsyncMediator`] is constructed through its builder.
/// It receives requests through its [`AsyncMediatorInternalHandle::send()`]
/// interface, which are processed by the user-defined [`AsyncRequestHandler`] implementation.
/// From within this `async` handler, events of type `Ev` can be published using the
/// [`BasicAsyncMediator::publish()`] functionality.
/// Listeners injected with [`super::BasicAsyncBuilder::add_listener()`]
/// are invoked when the user calls [`BasicAsyncMediator::next()`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use mediatrix::asynchronous::basic::*;
/// use async_trait::async_trait;
/// use async_std;
///
/// #[derive(Debug)]
/// enum MyEvent {
///     One,
///     Two
/// }
///
/// struct Request(u32);
///
/// #[async_trait]
/// impl AsyncRequestHandler<Request, MyEvent> for BasicAsyncMediator<MyEvent> {
///     async fn handle(&self, req: Request) {
///         match req.0 {
///             1 => self.publish(MyEvent::One).await,
///             2 => self.publish(MyEvent::Two).await,
///             _ => ()
///         };
///     }
/// }
///
/// async_std::task::block_on(async {
///     let mediator = BasicAsyncMediator::<MyEvent>::builder()
///         .add_listener(move |_: &MyEvent| {
///             /* Your listening logic */
///         })
///         .add_listener(move |_: &MyEvent| {
///             /* Your listening logic */
///         })
///         .build();
///
///     mediator.send(Request(1)).await;
///     mediator.next().await.ok();
/// });
///
#[cfg(feature = "async")]
#[derive(Debug)]
pub struct BasicAsyncMediator<Ev>
where
    Ev: Debug + 'static,
{
    pub(crate) basic: Mutex<BasicMediator<Ev>>,
}

#[async_trait]
impl<Ev> AsyncMediatorInternal<Ev> for BasicAsyncMediator<Ev>
where
    Ev: Debug + Send,
{
    /// Publishes an event `Ev` asynchronously.
    ///
    /// This method locks the `Mutex` and instructs
    /// the underlying [`BasicMediator`] to publish an event.
    /// Best used within [`AsyncRequestHandler::handle()`].
    ///
    /// You need to await the `Future` using `.await`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediatrix::asynchronous::basic::*;
    /// use async_trait::async_trait;
    ///
    /// #[derive(Debug)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// struct Request(u32);
    ///
    /// #[async_trait]
    /// impl AsyncRequestHandler<Request, MyEvent> for BasicAsyncMediator<MyEvent> {
    ///     async fn handle(&self, req: Request) {
    ///         match req.0 {
    ///             1 => self.publish(MyEvent::One).await,
    ///             2 => self.publish(MyEvent::Two).await,
    ///             _ => ()
    ///         };
    ///     }
    /// }
    ///
    async fn publish(&self, event: Ev) {
        let m = self.basic.lock().await;
        m.publish(event)
    }
}

#[async_trait]
impl<Ev> AsyncMediatorInternalHandle<Ev> for BasicAsyncMediator<Ev>
where
    Ev: Debug,
{
    /// Send a request of type `Req` to the mediator asynchronously.
    ///
    /// The request will be processed internally by [`AsyncRequestHandler::handle()`].
    /// This is why it is required to implement [`AsyncRequestHandler`] for [`BasicAsyncMediator`].
    ///
    /// You need to await the `Future` using `.await`.
    ///
    async fn send<Req>(&self, req: Req)
    where
        Self: AsyncRequestHandler<Req, Ev>,
        Req: Send,
    {
        <Self as AsyncRequestHandler<Req, Ev>>::handle(self, req).await
    }
}

#[async_trait]
impl<Ev> AsyncMediatorInternalNext for BasicAsyncMediator<Ev>
where
    Ev: Debug + Send,
{
    /// Process the next published event `Ev` asynchronously.
    ///
    /// This method locks the `Mutex` and instructs
    /// the underlying [`BasicMediator`] to process the next event.
    ///
    /// See [`BasicMediator::next()`] for more info.
    ///
    /// You need to await the `Future` using `.await`.
    ///
    async fn next(&self) -> Result<(), TryRecvError> {
        let m = self.basic.lock().await;
        m.next()
    }
}

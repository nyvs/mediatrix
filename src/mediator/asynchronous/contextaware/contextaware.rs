use std::sync::mpsc::TryRecvError;

use async_std::sync::Mutex;
use async_trait::async_trait;
use std::fmt::Debug;

use crate::asynchronous::basic::BasicAsyncMediator;

use super::*;

/// Context aware async mediator for asynchronous environments with events of type `Ev`.
///
/// Uses an underlying [`BasicAsyncMediator`] for base functionality
/// and a `Mutex` to store the user-defined context `Cx`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use mediatrix::asynchronous::contextaware::*;
/// use std::sync::Arc;
/// use async_trait::async_trait;
/// use async_std;
///
/// #[derive(Debug)]
/// enum MyEvent {
///     One,
///     Two
/// }
///
/// #[derive(Debug, Default)]
/// struct MyContext(Arc<u32>);
///
/// struct Request(u32);
///
/// #[async_trait]
/// impl CxAwareAsyncRequestHandler<MyContext, Request, MyEvent> for CxAwareAsyncMediator<MyContext, MyEvent> {
///     async fn handle(&self, req: Request, cx: &MyContext) {
///         let my_context: u32 = *cx.0;
///         match req.0 {
///             1 => self.publish(MyEvent::One).await,
///             2 => self.publish(MyEvent::Two).await,
///             _ => ()
///         };
///     }
/// }
///
/// async_std::task::block_on(async {
///     let mediator = CxAwareAsyncMediator::<MyContext, MyEvent>::builder()
///         .add_listener(move |_: &MyEvent| {
///             /* Your listening logic */
///         })
///         .add_listener(move |_: &MyEvent| {
///             /* Your listening logic */
///         })
///         .add_context(MyContext::default())
///         .build()
///         .unwrap();
///
///     mediator.send(Request(1)).await;
///     mediator.next().await.ok();
/// });
///
#[cfg(feature = "async")]
#[derive(Debug)]
pub struct CxAwareAsyncMediator<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug + 'static,
{
    pub(crate) basic: BasicAsyncMediator<Ev>,
    pub(crate) cx: Mutex<Cx>,
}

#[async_trait]
impl<Cx, Ev> AsyncMediatorInternal<Ev> for CxAwareAsyncMediator<Cx, Ev>
where
    Cx: Debug + Send,
    Ev: Debug + Send,
{
    /// Publishes an event `Ev` asynchronously.
    ///
    /// This method instructs the underlying [`BasicAsyncMediator`]
    /// to publish a user-defined event.
    ///
    /// It should be used within [`CxAwareAsyncRequestHandler::handle()`].
    ///
    /// You need to await the `Future` using `.await`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediatrix::asynchronous::contextaware::*;
    /// use async_trait::async_trait;
    /// use std::sync::Arc;
    ///
    /// #[derive(Debug)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// #[derive(Debug, Default)]
    /// struct MyContext(Arc<u32>);
    ///
    /// struct Request(u32);
    ///
    /// #[async_trait]
    /// impl CxAwareAsyncRequestHandler<MyContext, Request, MyEvent> for CxAwareAsyncMediator<MyContext, MyEvent> {
    ///     async fn handle(&self, req: Request, cx: &MyContext) {
    ///         let my_context: u32 = *cx.0;
    ///         match req.0 {
    ///             1 => self.publish(MyEvent::One).await,
    ///             2 => self.publish(MyEvent::Two).await,
    ///             _ => ()
    ///         };
    ///     }
    /// }
    ///
    async fn publish(&self, event: Ev) {
        self.basic.publish(event).await
    }
}

#[async_trait]
impl<Cx, Ev> CxAwareAsyncMediatorInternalHandle<Cx, Ev> for CxAwareAsyncMediator<Cx, Ev>
where
    Cx: Debug + Send + Sync,
    Ev: Debug + Send,
{
    /// Send a request of type `Req` to the mediator asynchronously.
    ///
    /// The request will be processed internally by [`CxAwareAsyncRequestHandler::handle()`].
    /// This is why it is required to implement [`CxAwareAsyncRequestHandler`] for [`CxAwareAsyncMediator`].
    /// A `Mutex` will be locked in order to gain access to the context `Cx`.
    ///
    /// You need to await the `Future` using `.await`.
    ///
    async fn send<Req>(&self, req: Req)
    where
        Self: CxAwareAsyncRequestHandler<Cx, Req, Ev>,
        Req: Send,
    {
        let m = self.cx.lock().await;
        <Self as CxAwareAsyncRequestHandler<Cx, Req, Ev>>::handle(self, req, &m).await
    }
}

#[async_trait]
impl<Cx, Ev> AsyncMediatorInternalNext for CxAwareAsyncMediator<Cx, Ev>
where
    Cx: Debug + Send,
    Ev: Debug + Send,
{
    /// Process the next published event `Ev` asynchronously.
    ///
    /// This method instructs the underlying [`BasicAsyncMediator`]
    /// to process the next event.
    ///
    /// See [`BasicAsyncMediator::next()`] for more info.
    ///
    /// You need to await the `Future` using `.await`.
    ///
    async fn next(&self) -> Result<(), TryRecvError> {
        self.basic.next().await
    }
}

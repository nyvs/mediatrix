use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use core::fmt::Debug;

use super::*;

/// Basic mediator for synchronous environments with events of type `Ev`.
///
/// A [`BasicMediator`] is constructed through its builder.
/// It receives requests through its [`SyncMediatorInternalHandle::send()`]
/// interface, which are processed by the user-defined [`RequestHandler`] implementation.
/// From within this handler, events of type `Ev` can be published using the
/// [`BasicMediator::publish()`] functionality.
/// Listeners injected with [`super::BasicBuilder::add_listener()`]
/// are invoked when the user calls [`BasicMediator::next()`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use mediatrix::synchronous::basic::*;
///
/// #[derive(Debug)]
/// enum MyEvent {
///     One,
///     Two
/// }
///
/// struct Request(u32);
///
/// impl RequestHandler<Request, MyEvent> for BasicMediator<MyEvent> {
///     fn handle(&self, req: Request) {
///         match req.0 {
///             1 => self.publish(MyEvent::One),
///             2 => self.publish(MyEvent::Two),
///             _ => ()
///         };
///     }
/// }
///
/// let mediator = BasicMediator::<MyEvent>::builder()
///     .add_listener(move |_: &MyEvent| {
///         /* Your listening logic */
///     })
///     .add_listener(move |_: &MyEvent| {
///         /* Your listening logic */
///     })
///     .build();
///
///     mediator.send(Request(1));
///     mediator.next().ok();
///
#[derive(Debug)]
pub struct BasicMediator<Ev>
where
    Ev: Debug,
{
    pub(crate) channel: (Sender<Ev>, Receiver<Ev>),
    pub(crate) listener: Vec<Box<dyn Listener<Ev>>>,
}

impl<Ev> SyncMediatorInternal<Ev> for BasicMediator<Ev>
where
    Ev: Debug,
{
    /// Publishes an event `Ev`.
    ///
    /// This method should be used within [`RequestHandler::handle()`]
    /// to publish a user-defined event.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediatrix::synchronous::basic::*;
    ///
    /// #[derive(Debug)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// struct Request(u32);
    ///
    /// impl RequestHandler<Request, MyEvent> for BasicMediator<MyEvent> {
    ///     fn handle(&self, req: Request) {
    ///         match req.0 {
    ///             1 => self.publish(MyEvent::One),
    ///             2 => self.publish(MyEvent::Two),
    ///             _ => ()
    ///         };
    ///     }
    /// }
    ///
    fn publish(&self, event: Ev) {
        self.channel.0.send(event).ok();
    }
}

impl<Ev> SyncMediatorInternalHandle<Ev> for BasicMediator<Ev>
where
    Ev: Debug,
{
    /// Send a request of type `Req` to the mediator.
    ///
    /// The request will be processed internally by [`RequestHandler::handle()`].
    /// This is why it is required to implement [`RequestHandler`] for [`BasicMediator`].
    ///
    fn send<Req>(&self, req: Req)
    where
        Self: RequestHandler<Req, Ev>,
    {
        <Self as RequestHandler<Req, Ev>>::handle(self, req);
    }
}

impl<Ev> SyncMediatorInternalNext for BasicMediator<Ev>
where
    Ev: Debug,
{
    /// Process the next published event `Ev`.
    ///
    /// [`SyncMediatorInternalNext::next()`] invokes
    /// registered listeners with a `&Ev`
    /// of the published event.
    ///
    fn next(&self) -> Result<(), TryRecvError> {
        match self.channel.1.try_recv() {
            Ok(ev) => {
                for listener in self.listener.iter() {
                    listener(&ev)
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

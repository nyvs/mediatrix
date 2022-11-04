use async_std::sync::Mutex;

use crate::mediator::{
    asynchronous::{
        basic::basic::BasicAsyncMediator,
        contextaware::{
            contextaware::CxAwareAsyncMediator, interface::CxAwareMediatorBuilderInterface,
        },
    },
    builder::{TryBuilderFlow, TryBuilderInternal},
    listener::Listener,
    synchronous::basic::{basic::BasicMediator, interface::BasicMediatorBuilderInterface},
};
use std::{fmt::Debug, sync::mpsc::channel};

/// The [`CxAwareAsyncBuilder`] helps you to create a [`CxAwareAsyncMediator`].
///
/// The [`CxAwareAsyncBuilder`] is part of the builder pattern.
/// It has three functionalities. The first one is adding a [`Listener`] via
/// [`CxAwareAsyncBuilder::add_listener()`].
/// Secondly, a context `Cx` can be added via [`CxAwareAsyncBuilder::add_context()`].
/// This must be done in order to receive a [`CxAwareAsyncMediator`] from [`TryBuilderFlow::build()`].
/// The third functionality is the mandatory [`TryBuilderFlow::build()`], which returns
/// a [`Result`] of type [`Result<CxAwareAsyncMediator<Cx, Ev>, Self::Error>`].
///
pub struct CxAwareAsyncBuilder<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug + 'static,
{
    mediator: BasicMediator<Ev>,
    cx: Option<Cx>,
}

impl<Cx, Ev> TryBuilderInternal<CxAwareAsyncMediator<Cx, Ev>, CxAwareAsyncBuilder<Cx, Ev>>
    for CxAwareAsyncMediator<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug,
{
    /// Creates a [`CxAwareAsyncBuilder`] with the goal of producing a [`CxAwareAsyncMediator`].
    ///
    fn builder() -> CxAwareAsyncBuilder<Cx, Ev> {
        CxAwareAsyncBuilder::<Cx, Ev> {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
            cx: None,
        }
    }
}

impl<M, Cx, Ev> BasicMediatorBuilderInterface<M, Ev> for CxAwareAsyncBuilder<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug,
{
    /// Adds a user-defined listener to the [`CxAwareAsyncBuilder`].
    ///
    /// To be able to supply a closure that implements [`Listener`],
    /// it must satisfy [`Send`] and `'static` bounds.
    ///
    /// Also it must be a `Fn(Ev)` with a return type of `()`
    /// where `Ev` is the user-defined event type
    /// that must be [`Debug`].
    ///
    fn add_listener(mut self, f: impl Listener<Ev>) -> Self {
        self.mediator.listener.push(Box::new(f));
        self
    }
}

impl<M, Cx, Ev> CxAwareMediatorBuilderInterface<M, Cx, Ev> for CxAwareAsyncBuilder<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug,
{
    /// Adds a user-defined context of type `Cx` to the [`CxAwareAsyncBuilder`].
    ///
    /// The context is available in [`super::CxAwareAsyncRequestHandler::handle()`].
    ///
    fn add_context(mut self, cx: Cx) -> Self
    where
        Ev: Debug,
    {
        self.cx = Some(cx);
        self
    }
}

impl<Cx, Ev> CxAwareAsyncBuilder<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug,
{
    /// Adds a user-defined listener to the [`CxAwareAsyncBuilder`].
    ///
    /// The supplied type must be a [`Listener`].
    /// As such, it must implement [`Send`] and `Fn(Ev)`,
    /// besides being `'static`.
    ///
    /// As a side note, here, `Ev` is the user-defined event type
    /// that must be [`Debug`].
    ///
    /// Note: The following example will add a [`Listener`] to the builder,
    /// but the result of `.build()` here will be an `Err` value.
    /// This is because in order to receive a valid [`CxAwareAsyncMediator`]
    /// you need to add a context. See [`CxAwareAsyncBuilder::add_context()`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediatrix::asynchronous::contextaware::*;
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
    /// let mediator = CxAwareAsyncMediator::<MyContext, MyEvent>::builder()
    ///     .add_listener(|_: &MyEvent| {
    ///         /* Your listening logic */
    ///     })
    ///     .build();
    ///
    pub fn add_listener(self, f: impl Listener<Ev>) -> Self {
        <Self as BasicMediatorBuilderInterface<CxAwareAsyncMediator<Cx, Ev>, Ev>>::add_listener(
            self, f,
        )
    }

    /// Adds a user-defined context of type `Cx` to the [`CxAwareAsyncBuilder`].
    ///
    /// The context is available in [`super::CxAwareAsyncRequestHandler::handle()`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediatrix::asynchronous::contextaware::*;
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
    /// let mediator = CxAwareAsyncMediator::<MyContext, MyEvent>::builder()
    ///     .add_context(MyContext::default())
    ///     .build();
    ///
    pub fn add_context(self, cx: Cx) -> Self {
        <Self as CxAwareMediatorBuilderInterface<CxAwareAsyncMediator<Cx, Ev>, Cx, Ev>>::add_context(
            self, cx,
        )
    }
}

#[derive(Debug)]
/// Error: No context was given while building.
pub struct NoCxAvailable;

impl<Cx, Ev> TryBuilderFlow<CxAwareAsyncMediator<Cx, Ev>> for CxAwareAsyncBuilder<Cx, Ev>
where
    Cx: Debug,
    Ev: Debug,
{
    type Error = NoCxAvailable;
    /// Builds the [`CxAwareAsyncMediator`] and returns it.
    ///
    /// Because [`CxAwareAsyncMediator`] implements [`TryBuilderInternal`],
    /// which in turn means, that the [`CxAwareAsyncBuilder`] implements [`TryBuilderFlow`]
    /// this method will return a [`Result<CxAwareAsyncMediator<Cx, Ev>, Self::Error>`] as stated by the return type.
    /// Note that here `Self::Error` is of type [`NoCxAvailable`], which means that no dependecy was added in
    /// the process of building.
    ///
    fn build(self) -> Result<CxAwareAsyncMediator<Cx, Ev>, Self::Error> {
        Ok(CxAwareAsyncMediator {
            basic: BasicAsyncMediator {
                basic: Mutex::new(self.mediator),
            },
            cx: Mutex::new(self.cx.ok_or(NoCxAvailable)?),
        })
    }
}

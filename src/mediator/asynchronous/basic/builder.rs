use async_std::sync::Mutex;

use crate::mediator::{
    asynchronous::basic::basic::BasicAsyncMediator,
    builder::{BuilderFlow, BuilderInternal},
    listener::Listener,
    synchronous::basic::{basic::BasicMediator, interface::BasicMediatorBuilderInterface},
};
use std::{fmt::Debug, sync::mpsc::channel};

/// The [`BasicAsyncBuilder`] helps you to create a [`BasicAsyncMediator`].
///
/// The [`BasicAsyncBuilder`] is part of the builder pattern.
/// It has only two functionalities. The first one is adding a [`Listener`] via
/// [`BasicAsyncBuilder::add_listener()`].
/// The second one is the mandatory [`BuilderFlow::build()`], which returns
/// a [`BasicAsyncMediator`].
///
pub struct BasicAsyncBuilder<Ev>
where
    Ev: Debug + 'static,
{
    mediator: BasicMediator<Ev>,
}

impl<Ev> BuilderInternal<BasicAsyncMediator<Ev>, BasicAsyncBuilder<Ev>> for BasicAsyncMediator<Ev>
where
    Ev: Debug,
{
    /// Creates a [`BasicAsyncBuilder`] with the goal of producing a [`BasicAsyncMediator`].
    ///
    fn builder() -> BasicAsyncBuilder<Ev> {
        BasicAsyncBuilder::<Ev> {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
        }
    }
}

impl<M, Ev> BasicMediatorBuilderInterface<M, Ev> for BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    /// Adds a user-defined listener to the [`BasicAsyncBuilder`].
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

impl<Ev> BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    /// Adds a user-defined listener to the [`BasicAsyncBuilder`].
    ///
    /// The supplied type must be a [`Listener`].
    /// As such, it must implement [`Send`] and `Fn(Ev)`,
    /// besides being `'static`.
    ///
    /// As a side note, here, `Ev` is the user-defined event type
    /// that must be [`Debug`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mediatrix::asynchronous::basic::*;
    ///
    /// #[derive(Debug)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// let mediator = BasicAsyncMediator::<MyEvent>::builder()
    ///     .add_listener(move |_: &MyEvent| {
    ///         /* Your listening logic */
    ///     })
    ///     .build();
    ///
    pub fn add_listener(self, f: impl Listener<Ev>) -> Self {
        <Self as BasicMediatorBuilderInterface<BasicMediator<Ev>, Ev>>::add_listener(self, f)
    }
}

impl<Ev> BuilderFlow<BasicAsyncMediator<Ev>> for BasicAsyncBuilder<Ev>
where
    Ev: Debug,
{
    /// Builds the [`BasicAsyncMediator`] and returns it.
    ///
    /// Because [`BasicAsyncMediator`] implements [`BuilderInternal`],
    /// which in turn means, that the [`BasicAsyncBuilder`] implements [`BuilderFlow`]
    /// and not [`crate::builder::TryBuilderFlow`], this method will
    /// always return a [`BasicAsyncMediator`] as stated by the return type.
    ///
    fn build(self) -> BasicAsyncMediator<Ev> {
        BasicAsyncMediator {
            basic: Mutex::new(self.mediator),
        }
    }
}

use super::{basic::BasicMediator, interface::BasicMediatorBuilderInterface};
use crate::mediator::{
    builder::{BuilderFlow, BuilderInternal},
    listener::Listener,
};
use std::{fmt::Debug, sync::mpsc::channel};

/// The [`BasicBuilder`] helps you to create a [`BasicMediator`].
///
/// The [`BasicBuilder`] is part of the builder pattern.
/// It has only two functionalities. The first one is adding a [`Listener`] via
/// [`BasicBuilder::add_listener()`].
/// The second one is the mandatory [`BuilderFlow::build()`], which returns
/// a [`BasicMediator`].
///
pub struct BasicBuilder<Ev>
where
    Ev: Debug,
{
    mediator: BasicMediator<Ev>,
}

impl<Ev> BuilderInternal<BasicMediator<Ev>, BasicBuilder<Ev>> for BasicMediator<Ev>
where
    Ev: Debug,
{
    /// Creates a [`BasicBuilder`] with the goal of producing a [`BasicMediator`].
    ///
    fn builder() -> BasicBuilder<Ev> {
        BasicBuilder::<Ev> {
            mediator: BasicMediator::<Ev> {
                channel: channel(),
                listener: vec![],
            },
        }
    }
}

impl<M, Ev> BasicMediatorBuilderInterface<M, Ev> for BasicBuilder<Ev>
where
    Ev: Debug,
{
    /// Adds a user-defined listener to the [`BasicBuilder`].
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

impl<Ev> BasicBuilder<Ev>
where
    Ev: Debug,
{
    /// Adds a user-defined listener to the [`BasicBuilder`].
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
    /// use mediatrix::synchronous::basic::*;
    ///
    /// #[derive(Debug)]
    /// enum MyEvent {
    ///     One,
    ///     Two
    /// }
    ///
    /// let mediator = BasicMediator::<MyEvent>::builder()
    ///     .add_listener(move |_: &MyEvent| {
    ///         /* Your listening logic */
    ///     })
    ///     .build();
    ///
    pub fn add_listener(self, f: impl Listener<Ev>) -> Self {
        <Self as BasicMediatorBuilderInterface<BasicMediator<Ev>, Ev>>::add_listener(self, f)
    }
}

impl<Ev> BuilderFlow<BasicMediator<Ev>> for BasicBuilder<Ev>
where
    Ev: Debug,
{
    /// Builds the [`BasicMediator`] and returns it.
    ///
    /// Because [`BasicMediator`] implements [`BuilderInternal`],
    /// which in turn means, that the [`BasicBuilder`] implements [`BuilderFlow`]
    /// and not [`crate::builder::TryBuilderFlow`], this method will
    /// always return a [`BasicMediator`] as stated by the return type.
    ///
    fn build(self) -> BasicMediator<Ev> {
        self.mediator
    }
}

//! mediatrix provides multiple strongly-typed mediators for synchronous and asynchronous needs.
//!
//! # Quick Start
//!
//! For synchronous needs, the [`BasicMediator`] suffices.
//! The [`BasicAsyncMediator`] on the other hand is needed for asynchronous
//! handlers.
//! If you need your handler to include some sort of context,
//! use the [`CxAwareAsyncMediator`]. This mediator requires a user-defined
//! type to be injected through its builder.
//!
//! # Crate Architecture
//!
//! The crate is structured in an additive way.
//! The [`BasicMediator`] is used as a basis for [`BasicAsyncMediator`].
//! In turn, the [`BasicAsyncMediator`] is used as a basis for the [`CxAwareAsyncMediator`].
//! This way, code duplication is minimal and the code in general is less error-prone.
//!
//! Each mediator consists of a module for its own builder, interface and the implementation itself.
//!
//! Builders implement basic functionality [`builder::BuilderInternal`] and [`builder::BuilderFlow`] if
//! the builder is required to always be able to return a valid mediator.
//! Otherwise, builders implement [`builder::TryBuilderInternal`] and [`builder::TryBuilderFlow`],
//! which results in a return value of  [`Result<M, Self::Error>`], where `M` is the mediator type.
//!
//! Interfaces contain trait definitions highlighting the user-facing API,
//! which are implemented by their mediator.
//!
//! Lastly, the module for the mediator itself contains
//! internal structure and implementation details for the
//! respective mediator.
//!
//! [`BasicMediator`]: synchronous::basic::BasicMediator
//! [`BasicAsyncMediator`]: asynchronous::basic::BasicAsyncMediator
//! [`CxAwareAsyncMediator`]: asynchronous::contextaware::CxAwareAsyncMediator

#![doc(html_root_url = "https://docs.rs/mediatrix/1.0.0")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, unused_imports, unsafe_code)]

mod mediator;

#[cfg(feature = "async")]
pub use mediator::asynchronous;
pub use mediator::builder;
pub use mediator::listener;
pub use mediator::synchronous;

#[cfg(test)]
mod test;

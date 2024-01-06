use crate::commands;

use super::{Context, Data};

commands! {
    start,
    stop
}

mod macros {
    /// This macro is used to reduce boilerplate when coalescing commands into a single vector.
    /// Usage:
    /// ```ignore
    /// commands! {
    ///     start,
    ///     stop
    /// }
    /// ```
    /// Expands to:
    /// ```ignore
    /// pub mod start;
    /// pub mod stop;
    /// pub use start::start;
    /// pub use stop::stop;
    /// pub fn commands() -> Vec<poise::Command<Data, Error>> {
    ///    vec![start(), stop()]
    /// }
    /// ```
    #[macro_export]
    macro_rules! commands {
        ($($cmd:ident),* $(,)?) => {
            $(
                pub mod $cmd;
            )*
            $(
                pub use $cmd::$cmd;
            )*
            pub fn commands() -> Vec<poise::Command<Data, anyhow::Error>> {
                vec![$( $cmd() ),*]
            }
        };
    }
}

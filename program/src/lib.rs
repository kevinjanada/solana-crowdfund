#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub mod state;
pub mod instruction;
pub mod processor;

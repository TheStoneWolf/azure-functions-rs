//! # Azure Functions for Rust
//!
//! This crate shares types between the `azure-functions-codegen` and `azure-functions` crates.
#![deny(missing_docs)]
#![deny(unused_extern_crates)]

#[macro_use]
extern crate serde_derive;

#[doc(hidden)]
pub mod codegen;
mod context;

#[doc(hidden)]
pub mod rpc {
  pub mod protocol {
    use azure_functions_shared_codegen::generated_mod;

    generated_mod!(FunctionRpc);
    generated_mod!(FunctionRpc_grpc);

    pub use self::FunctionRpc::*;
    pub use self::FunctionRpc_grpc::*;
  }
}

pub use self::context::*;

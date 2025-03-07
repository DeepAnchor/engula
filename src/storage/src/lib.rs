// Copyright 2021 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An Engula module that provides object storage abstractions and
//! implementations.
//!
//! # Abstraction
//!
//! [`Storage`] is an abstraction to store data objects.
//!
//! # Implementation
//!
//! Some built-in implementations of [`Storage`]:
//!
//! - [`file`](crate::file)
//! - [`grpc`](crate::grpc)
//! - [`mem`](crate::mem)
//!
//! [`Storage`]: crate::Storage

#![feature(type_alias_impl_trait)]

mod bucket;
mod error;
mod storage;

pub mod file;
pub mod grpc;
pub mod mem;

pub use async_trait::async_trait;

pub use self::{
    bucket::Bucket,
    error::{Error, Result},
    storage::Storage,
};

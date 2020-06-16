// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Visual Studio product, workload and component IDs.
//!
//! All of the product, workload and component IDs exposed by this module are derived from the
//! information at [Microsoft Docs][1].
//!
//! [1]: https://docs.microsoft.com/en-us/visualstudio/install/workload-and-component-ids?view=vs-2019

#![no_std]
#![deny(
    warnings,
    future_incompatible,
    rust_2018_idioms,
    rustdoc,
    unused,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_results,
    clippy::all,
    clippy::pedantic
)]

macro_rules! enumerate_id_set {
    (
        #[doc = $enum_doc:expr]
        $visi:vis enum $type_name:ident {
            $(#[doc = $id_doc:expr] $id_name:ident = $id_value:expr,)*
        }
    ) => {
        use core::{
            convert::TryFrom,
            fmt::{self, Display, Formatter},
            str::FromStr,
        };

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[doc = $enum_doc]
        $visi enum $type_name {$(
            #[doc = $id_doc]
            $id_name
        ),*}

        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$id_name => f.write_str($id_value)),*
                }
            }
        }

        impl From<$type_name> for &str {
            fn from(other: $type_name) -> Self {
                match other {
                    $($type_name::$id_name => $id_value),*
                }
            }
        }

        impl FromStr for $type_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = ();

            fn try_from(other: &str) -> Result<Self, Self::Error> {
                match other {
                    $($id_value => Ok(Self::$id_name),)*
                    _ => Err(()),
                }
            }
        }
    };
    (
        #[doc = $enum_doc:expr]
        $visi:vis enum $type_name:ident {
            $(#[doc = $id_doc:expr] $id_name:ident = $str_value:expr),*
        }
    ) => {
        enumerate_id_set! {
            #[doc = $enum_doc]
            $visi enum $type_name {$(
                #[doc = $id_doc]
                $id_name = $str_value
            ),*}
        }
    };
}

mod product;
mod workload;

pub use product::Product;
pub use workload::Workload;

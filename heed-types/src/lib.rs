//! Types that can be used to serialize and deserialize types inside databases.
//!
//! How to choose the right type to store things in this database?
//! For specific types you can choose:
//!   - [`Str`] to store [`str`](primitive@str)s
//!   - [`Unit`] to store `()` types
//!   - [`SerdeBincode`],  [`SerdeJson`], or [`SerdeRmp`] to store [`Serialize`]/[`Deserialize`] types
//!
//! But if you want to store big types that can be efficiently deserialized then
//! here is a little table to help you in your quest:
//!
//! | Available types    | Encoding type | Decoding type | allocations                                             |
//! |--------------------|:-------------:|:-------------:|---------------------------------------------------------|
//! | [`CowSlice`]       | `&[T]`        | `Cow<[T]>`    | will allocate if memory is miss-aligned                 |
//! | [`CowType`]        | `&T`          | `Cow<T>`      | will allocate if memory is miss-aligned                 |
//! | [`OwnedSlice`]     | `&[T]`        | `Vec<T>`      | will _always_ allocate                                  |
//! | [`OwnedType`]      | `&T`          | `T`           | will _always_ allocate                                  |
//! | [`UnalignedSlice`] | `&[T]`        | `&[T]`        | will _never_ allocate because alignment is always valid |
//! | [`UnalignedType`]  | `&T`          | `&T`          | will _never_ allocate because alignment is always valid |
//!
//! [`Serialize`]: serde::Serialize
//! [`Deserialize`]: serde::Deserialize

#![warn(missing_docs)]

mod cow_slice;
mod cow_type;
mod integer;
mod lazy_decode;
mod owned_slice;
mod owned_type;
mod str;
mod unaligned_slice;
mod unaligned_type;
mod unit;

#[cfg(feature = "serde-bincode")]
mod serde_bincode;

#[cfg(feature = "serde-json")]
mod serde_json;

#[cfg(feature = "serde-rmp")]
mod serde_rmp;

use heed_traits::BoxedError;

pub use self::cow_slice::CowSlice;
pub use self::cow_type::CowType;
pub use self::integer::*;
pub use self::lazy_decode::{Lazy, LazyDecode};
pub use self::owned_slice::OwnedSlice;
pub use self::owned_type::OwnedType;
pub use self::str::Str;
pub use self::unaligned_slice::UnalignedSlice;
pub use self::unaligned_type::UnalignedType;
pub use self::unit::Unit;

/// Describes a slice of bytes `[u8]` that is totally
/// borrowed and doesn't depends on any [memory alignment].
///
/// [memory alignment]: std::mem::align_of()
pub type ByteSlice = UnalignedSlice<u8>;

/// A convenient struct made to ignore the type when decoding it.
///
/// It is appropriate to be used to count keys for example
/// or to ensure that an entry exist for example.
pub struct DecodeIgnore;

impl heed_traits::BytesDecode<'_> for DecodeIgnore {
    type DItem = ();

    fn bytes_decode(_bytes: &[u8]) -> Result<Self::DItem, BoxedError> {
        Ok(())
    }
}

#[cfg(feature = "serde-bincode")]
pub use self::serde_bincode::SerdeBincode;
#[cfg(feature = "serde-json")]
pub use self::serde_json::SerdeJson;
#[cfg(feature = "serde-rmp")]
pub use self::serde_rmp::SerdeRmp;

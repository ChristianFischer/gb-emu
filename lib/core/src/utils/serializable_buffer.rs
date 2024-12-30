/*
 * Copyright (C) 2022-2024 by Christian Fischer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

#[cfg(feature = "dyn_alloc")]
pub use serializable_buffer::*;

#[cfg(feature = "dyn_alloc")]
mod serializable_buffer {
    use alloc::vec::Vec;

    use core::ops::{Deref, DerefMut};

    use crate::mmu::memory_data::MemoryDataFixedSize;
    use crate::utils::SerdeSupport;


    /// A struct to be used as a replacement for a `Vec`, which
    /// is intended to be serialized and likely holds a large amount of data.
    /// Unlike a normal `Vec`, this struct serializes its data into
    /// a compressed base64 encoded string instead of a list of numbers.
    #[derive(Clone)]
    pub struct SerializableBuffer<T: SerdeSupport + Clone>(
        Vec<T>
    );

    impl<T: SerdeSupport + Clone> SerializableBuffer<T> {
        /// Get a vector containing a copy of the internal data.
        pub fn to_vec(&self) -> Vec<T> {
            self.0.clone()
        }


        /// Get the data slice stored in this object.
        pub fn as_slice(&self) -> &[T] {
            &self.0
        }
    }

    impl<T: SerdeSupport + Clone, const S: usize> From<[T; S]> for SerializableBuffer<T> {
        fn from(value: [T; S]) -> Self {
            Self(value.into())
        }
    }

    impl<const S: usize> From<MemoryDataFixedSize<S>> for SerializableBuffer<u8> {
        fn from(value: MemoryDataFixedSize<S>) -> Self {
            let array: [u8; S] = value.into();
            Self(array.into())
        }
    }

    impl<T: SerdeSupport + Clone> From<Vec<T>> for SerializableBuffer<T> {
        fn from(value: Vec<T>) -> Self {
            Self(value)
        }
    }

    impl<T: SerdeSupport + Clone> Into<Vec<T>> for SerializableBuffer<T> {
        fn into(self) -> Vec<T> {
            self.0
        }
    }

    impl<T: SerdeSupport + Clone> Deref for SerializableBuffer<T> {
        type Target = Vec<T>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: SerdeSupport + Clone> DerefMut for SerializableBuffer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<'a, T: SerdeSupport + Clone> IntoIterator for &'a SerializableBuffer<T> {
        type Item = &'a T;
        type IntoIter = core::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    impl<'a, T: SerdeSupport + Clone> IntoIterator for &'a mut SerializableBuffer<T> {
        type Item = &'a mut T;
        type IntoIter = core::slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter_mut()
        }
    }

    #[cfg(feature = "serde")]
    impl<T: SerdeSupport + Clone> serde::Serialize for SerializableBuffer<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
                S: serde::Serializer
        {
            crate::utils::serde::serialize::serialize_bytes(serializer, &self.0)
        }
    }

    #[cfg(feature = "serde")]
    impl<'de, T: SerdeSupport + Clone> serde::Deserialize<'de> for SerializableBuffer<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
                D: serde::Deserializer<'de>
        {
            let v = crate::utils::serde::serialize::deserialize_bytes(deserializer)?;

            Ok(SerializableBuffer::from(v))
        }
    }
}

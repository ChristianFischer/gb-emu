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

use crate::utils::SerdeSupport;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::de::Error;


/// A struct to be used instead of fixed size arrays,
/// which can be serialized using serde.
#[derive(Clone)]
pub struct SerializableArray<T: SerdeSupport + Copy + Clone, const SIZE: usize>(
    [T; SIZE],
);


impl<T: SerdeSupport + Copy + Clone, const SIZE: usize> From<[T; SIZE]> for SerializableArray<T, SIZE> {
    fn from(value: [T; SIZE]) -> Self {
        Self(value)
    }
}


impl<T: SerdeSupport + Copy + Clone, const SIZE: usize> Deref for SerializableArray<T, SIZE> {
    type Target = [T; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: SerdeSupport + Copy + Clone, const SIZE: usize> DerefMut for SerializableArray<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl<T: SerdeSupport + Copy + Clone, const SIZE: usize> IntoIterator for SerializableArray<T, SIZE> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}


impl<'a, T: SerdeSupport + Copy + Clone, const SIZE: usize> IntoIterator for &'a SerializableArray<T, SIZE> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}


impl<'a, T: SerdeSupport + Copy + Clone, const SIZE: usize> IntoIterator for &'a mut SerializableArray<T, SIZE> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}



#[cfg(feature = "serde")]
impl<T: SerdeSupport + Copy + Clone, const SIZE: usize> serde::Serialize for SerializableArray<T, SIZE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: serde::Serializer,
    {
        crate::utils::serde::serialize::serialize_bytes(serializer, &self.0)
    }
}


#[cfg(feature = "serde")]
impl<'de, T: SerdeSupport + Copy + Clone, const SIZE: usize> serde::Deserialize<'de> for SerializableArray<T, SIZE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: serde::Deserializer<'de>,
    {
        let v = crate::utils::serde::serialize::deserialize_bytes(deserializer)?;
        let ex_len = v.len();

        let array: [T; SIZE] = v
                .try_into()
                .map_err(|_| {
                    Error::custom(format!(
                        "Failed to convert data into array. [original size: {} / {}]",
                        ex_len, SIZE
                    ))
                })
                ?;

        Ok(SerializableArray::from(array))
    }
}

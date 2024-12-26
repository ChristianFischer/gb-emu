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

use core::ops::{BitOr, BitOrAssign};

use crate::cpu::interrupts::Interrupts;
use crate::debug::DebugEvents;

/// Represents the signals sent from a component back to the memory bus.
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MemoryBusSignals {
    /// Interrupts risen by a component.
    pub interrupts: Interrupts,

    /// Flags of events which occurred during updating a component.
    pub events: DebugEvents,

}


impl BitOr for MemoryBusSignals {
    type Output = MemoryBusSignals;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            interrupts: self.interrupts | rhs.interrupts,
            events:     self.events     | rhs.events,
        }
    }
}


impl BitOrAssign for MemoryBusSignals {
    fn bitor_assign(&mut self, rhs: Self) {
        self.interrupts |= rhs.interrupts;
        self.events     |= rhs.events;
    }
}


/// Trait for objects connected to the memory bus.
/// A connected object may receive memory read and write requests
/// and has to respond on them.
pub trait MemoryBusConnection {
    /// A request to read from a memory address in the components accountability.
    /// The component has to respond to this request, if necessary by providing
    /// a default value.
    fn on_read(&self, address: u16) -> u8;

    /// A request to write to a memory address in the components accountability.
    fn on_write(&mut self, address: u16, value: u8);

    /// Takes the signals sent from a component since the last call.
    /// After calling this, the pending signals of this component are expected to be cleared.
    fn take_signals(&mut self) -> MemoryBusSignals {
        MemoryBusSignals::default()
    }
}


/// A trait to implement a memory map to find the according component for each memory address.
/// To easily implement a memory mapper, the macro `impl_memory_mapper` may be used.
pub trait MemoryMapper<TRootType> {
    /// Get the component responsible to read from a given address.
    fn map<'a>(address: u16, root: &'a TRootType) -> &'a dyn MemoryBusConnection;

    /// Get the component responsible to write to a given address.
    fn map_mut<'a>(address: u16, root: &'a mut TRootType) -> &'a mut dyn MemoryBusConnection;
}


/// A memory bus which interconnects various peripheral components together
/// and provides an interface to read and write data on each of it's components.
/// To find the according component for each IO operation, the memory bus will
/// use an additional memory mapper component.
pub trait MemoryBus<TRootType, TMemoryMapper>
    where TMemoryMapper: MemoryMapper<TRootType>
{
    /// Get the root object owning all peripherals.
    fn get_root(&self) -> &TRootType;

    /// Get the root object owning all peripherals.
    fn get_root_mut(&mut self) -> &mut TRootType;

    /// Loads a single byte from an address via this memory bus.
    /// The memory bus will take the data from the according component.
    fn read(&self, address: u16) -> u8 {
        let root       = self.get_root();
        let connection = TMemoryMapper::map(address, root);
        connection.on_read(address)
    }


    /// Send a single byte to an address via this memory bus.
    /// The memory bus will forward the data to the according component.
    fn write(&mut self, address: u16, value: u8) {
        let root       = self.get_root_mut();
        let connection = TMemoryMapper::map_mut(address, root);
        connection.on_write(address, value);
    }
}


/// An utility macro to easily implement a memory mapper. This will implement both mutable and
/// immutable mapper code with a single set of match expressions.
macro_rules! impl_memory_mapper {
    (MemoryMapper($root:ident : $root_type:ident) for $name:ident { $($pattern:pat => $data:expr),+ }) => {
        impl MemoryMapper<$root_type> for $name {
            fn map(address: u16, $root: &$root_type) -> &dyn MemoryBusConnection {
                match address {
                    $(
                        $pattern => &$data,
                    )+
                }
            }

            fn map_mut(address: u16, $root: &mut $root_type) -> &mut dyn MemoryBusConnection {
                match address {
                    $(
                        $pattern => &mut $data,
                    )+
                }
            }
        }
    }
}


/// Helper macro to map memory addresses into their distinct areas.
macro_rules! memory_map {
    ($addr:expr => { $($from:literal $(..= $to:literal)? => [$($param:ident)?] $code:expr),+ }) => {
        match $addr {
            $(
                $from $(..= $to)? => {
                    $(let $param: usize = ($addr as usize) - ($from as usize);)?
                    $code
                }
            )+

            _ => unreachable!(),
        }
    }
}


pub(crate) use impl_memory_mapper;
pub(crate) use memory_map;

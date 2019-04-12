// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Represents a memory range for which an allocator can allocate.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryRange
{
	/// From (inclusive).
	pub from: MemoryAddress,

	/// To (exclusive).
	pub to: MemoryAddress,
}

impl MemoryRange
{
	/// Create a new instance.
	#[inline(always)]
	pub const fn new(from: MemoryAddress, to: MemoryAddress) -> Self
	{
		Self
		{
			from,
			to,
		}
	}

	#[inline(always)]
	fn contains(&self, from_memory_address: MemoryAddress) -> bool
	{
		from_memory_address >= self.from && from_memory_address < self.to
	}
}

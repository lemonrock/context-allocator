// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct MemoryRange
{
	from: MemoryAddress,
	to: MemoryAddress,
}

impl Default for MemoryRange
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::NotInUse
	}
}

impl MemoryRange
{
	const NotInUse: Self = MemoryRange
	{
		from: MemoryAddress::InvalidOn64BitMaximum,
		to: MemoryAddress::InvalidOn64BitMaximum,
	};

	#[inline(always)]
	fn contains(&self, current_memory: MemoryAddress) -> bool
	{
		self.debug_assert_memory_range_valid();

		current_memory >= self.from && current_memory < self.to
	}

	#[inline(always)]
	fn debug_assert_is_in_use(&self, allocator_name: &str)
	{
		self.debug_assert_memory_range_valid();

		debug_assert!(self.from != MemoryAddress::InvalidOn64BitMaximum && self.to != MemoryAddress::InvalidOn64BitMaximum, "{} allocator is not in use", allocator_name)
	}

	#[inline(always)]
	fn debug_assert_memory_range_contains_end(&self, current_memory: MemoryAddress, non_zero_size: NonZeroUsize, allocator_name: &str)
	{
		self.debug_assert_memory_range_valid();

		debug_assert!(self.contains(current_memory.add_non_zero(non_zero_size)), "Memory to deallocate does not entirely fit within {} allocator's range", allocator_name)
	}

	#[inline(always)]
	fn debug_assert_memory_range_valid(&self)
	{
		debug_assert!(self.from <= self.to, "from must be less than or equal to to");
	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BlockSize
{
	block_size: NonZeroUsize,
	block_size_less_one: usize,
	block_size_power_of_two_exponent: usize,
}

impl BlockSize
{
	#[inline(always)]
	fn new(block_size: NonZeroUsize) -> Self
	{
		debug_assert!(block_size.is_power_of_two(), "block_size `{:?}` is not a power of two", block_size);

		Self
		{
			block_size,
			block_size_less_one: block_size.decrement(),
			block_size_power_of_two_exponent: block_size.logarithm_base2(),
		}
	}

	#[inline(always)]
	fn alignment_is_minimum(&self, non_zero_power_of_two_alignment: NonZeroUsize) -> bool
	{
		non_zero_power_of_two_alignment <= self.block_size
	}

	#[inline(always)]
	fn number_of_blocks_required(&self, non_zero_size: NonZeroUsize) -> NumberOfBits
	{
		NumberOfBits((non_zero_size.get() + self.block_size_less_one) >> self.block_size_power_of_two_exponent)
	}

	#[inline(always)]
	fn blocks_offset(&self, allocations_start_from: MemoryAddress, start_of_allocated_memory: MemoryAddress) -> NumberOfBits
	{
		debug_assert!(start_of_allocated_memory >= allocations_start_from, "start_of_allocated_memory must be >= allocations_start_from");

		NumberOfBits(start_of_allocated_memory.difference(allocations_start_from) >> self.block_size_power_of_two_exponent)
	}

	#[inline(always)]
	fn scale_to_memory_offset_in_bytes(&self, number_of_bits: usize) -> NumberOfBytes
	{
		NumberOfBytes(number_of_bits << self.block_size_power_of_two_exponent)
	}
}

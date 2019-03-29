// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Bitmap-based allocator.
#[derive(Debug)]
pub struct BitmapAllocator
{
	inclusive_start_of_array_of_u64s: MemoryAddress,
	exclusive_end_of_array_of_u64s: MemoryAddress,
	next_allocation_start_from: Cell<MemoryAddress>,
	
	allocations_start_from: MemoryAddress,
	allocations_start_from_blocks: usize,

	block_size_less_one: usize,
	block_size_power_of_two_exponent: usize,
}

impl Allocator for BitmapAllocator
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		let desired_alignment_power_of_two_exponent = non_zero_power_of_two_alignment.logarithm_base2();
		if self.block_size_power_of_two_exponent >= desired_alignment_power_of_two_exponent
		{
			let number_of_blocks_required = self.number_of_blocks_required(non_zero_size);
			self.allocate_number_of_blocks(number_of_blocks_required, 1.non_zero())
		}
		else
		{
			let block_alignment_power_of_two_exponent_less_minimum = desired_alignment_power_of_two_exponent - self.block_size_power_of_two_exponent;
			let block_alignment_power_of_two_less_minimum = 1 << block_alignment_power_of_two_exponent_less_minimum;

			if unlikely!(block_alignment_power_of_two_less_minimum > Self::BitsInAnU64)
			{
				return Err(AllocErr)
			}

			let number_of_blocks_required = self.number_of_blocks_required(non_zero_size);
			self.allocate_number_of_blocks(number_of_blocks_required, block_alignment_power_of_two_less_minimum.non_zero())
		}
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		let number_of_blocks_required = self.number_of_blocks_required(non_zero_size);
		let blocks_offset = current_memory.subtract(self.allocations_start_from) >> self.block_size_power_of_two_exponent;

		let u64s_offset = blocks_offset / Self::BitsInAnU64;
		let bits_offset_with_u64 = blocks_offset - (u64s_offset * Self::BitsInAnU64);
		let start_from_memory_address = self.inclusive_start_of_array_of_u64s.add(u64s_offset * Self::SizeOfU64);
		let (mut contiguous_bits_to_unset_from_memory_address, mut remaining_blocks) = if likely!(bits_offset_with_u64 != 0)
		{
			let lower_bits_to_unset = Self::BitsInAnU64 - bits_offset_with_u64;
			start_from_memory_address.unset_bottom_bits_of_u64(bits_to_unset);
			(start_from_memory_address, number_of_blocks_required - lower_bits_to_unset)
		}
		else
		{
			(start_from_memory_address, number_of_blocks_required)
		};

		while remaining_blocks > (Self::BitsInAnU64 - 1)
		{
			contiguous_bits_to_unset_from_memory_address.write_and_advance(0x0000_0000_0000_0000u64);
			remaining_blocks -= Self::BitsInAnU64;
		}

		if likely!(remaining_blocks != 0)
		{
			contiguous_bits_to_unset_from_memory_address.unset_top_bits_of_u64(remaining_blocks);
		}
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		panic!();
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		panic!();
	}
}

impl BitmapAllocator
{
	const SizeOfU64: usize = size_of::<u64>();

	const SizeOfU64NonZero: NonZeroUsize = non_zero_usize(Self::SizeOfU64);

	const BitsInAByte: usize = 8;

	const BitsInAnU64: usize = Self::SizeOfU64 * Self::BitsInAByte;

	#[inline(always)]
	pub(crate) fn new(inclusive_start_of_array_of_u64s: MemoryAddress, size_in_bytes: NonZeroUsize, allocations_start_from: MemoryAddress, block_size: NonZeroUsize) -> Self
	{
		debug_assert_eq!(size_in_bytes.get() % Self::SizeOfU64, 0, "size_in_bytes `{:?}` is not a multiple of the size of an u64", size_in_bytes);
		debug_assert!(block_size.is_power_of_two(), "block_size `{:?}` is not a power of two", block_size);

		Self
		{
			inclusive_start_of_array_of_u64s,
			exclusive_end_of_array_of_u64s: inclusive_start_of_array_of_u64s.add_non_zero(size_in_bytes),
			next_allocation_start_from: Cell::new(inclusive_start_of_array_of_u64s),

			allocations_start_from,
			allocations_start_from_blocks: inclusive_start_of_array_of_u64s.to_usize() * Self::BitsInAByte,

			block_size_less_one: block_size.decrement(),
			block_size_power_of_two_exponent: block_size.logarithm_base2(),
		}
	}

	#[inline(always)]
	fn number_of_blocks_required(&self, size_to_allocate: NonZeroUsize) -> NonZeroUsize
	{
		((size_to_allocate.get() + self.block_size_less_one) >> self.block_size_power_of_two_exponent).non_zero()
	}

	#[inline(always)]
	fn allocate_number_of_blocks(&self, number_of_blocks_required: NonZeroUsize, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		macro_rules! scan
		{
			($self: ident, $end_memory_address: ident, $block_alignment_power_of_two_less_minimum: ident, $callback: ident) =>
			{
				{
					// keep this aligned.
					let mut contigous_zeros_count = 0;
					let mut memory_address = $self.next_allocation_start_from.get();
					while memory_address != $end_memory_address
					{
						let current = memory_address.read::<u64>();
						let current_leading_zeros = current.leading_zeros() as usize;
						let contiguous_zeros_now_available = contigous_zeros_count + current_leading_zeros;

						if contiguous_zeros_now_available >= number_of_blocks_required.get()
						{
							return $self.allocate_in_contiguous_leading_zeros(contigous_zeros_count, memory_address, number_of_blocks_required, contiguous_zeros_now_available)
						}

						contigous_zeros_count = match $callback($self, number_of_blocks_required, memory_address, current, current_leading_zeros, contiguous_zeros_now_available, $block_alignment_power_of_two_less_minimum)
						{
							Ok(successful_allocation) => return Ok(successful_allocation),
							Err(contigous_zeros_count) => contigous_zeros_count,
						};
						memory_address = memory_address.add_non_zero(Self::SizeOfU64NonZero);
					}
				}
			}
		}

		let callback = if number_of_blocks_required.get() < Self::BitsInAnU64
		{
			Self::number_of_blocks_is_less_than_64
		}
		else
		{
			Self::number_of_blocks_is_64_or_more
		};

		let end_memory_address = self.exclusive_end_of_array_of_u64s;
		scan!(self, end_memory_address, block_alignment_power_of_two_less_minimum, callback);

		let end_memory_address = self.next_allocation_start_from.get();
		self.next_allocation_start_from.set(self.inclusive_start_of_array_of_u64s);
		scan!(self, end_memory_address, block_alignment_power_of_two_less_minimum, callback);

		Err(AllocErr)
	}

	#[inline(always)]
	fn allocate_in_contiguous_leading_zeros(&self, contigous_zeros_count: usize, memory_address: MemoryAddress, number_of_blocks_required: NonZeroUsize, contiguous_zeros_now_available: usize) -> Result<MemoryAddress, AllocErr>
	{
		let initial_block_trailing_zeros = contigous_zeros_count % Self::BitsInAnU64;
		let contiguous_blocks_start_from_memory_address = memory_address.subtract
		({
			let number_of_contiguous_u64s = (contigous_zeros_count - initial_block_trailing_zeros) / Self::BitsInAnU64;
			number_of_contiguous_u64s * Self::SizeOfU64
		});

		if likely!(initial_block_trailing_zeros != 0)
		{
			contiguous_blocks_start_from_memory_address.subtract(Self::SizeOfU64).set_bottom_bits_of_u64(initial_block_trailing_zeros)
		}

		let mut update_contiguous_blocks_memory_address = contiguous_blocks_start_from_memory_address;
		while likely!(update_contiguous_blocks_memory_address != memory_address)
		{
			update_contiguous_blocks_memory_address.write_and_advance(0xFFFF_FFFF_FFFF_FFFFu64)
		}

		let number_of_bits_to_set_in_this_block = (number_of_blocks_required.get() - initial_block_trailing_zeros) % Self::BitsInAnU64;
		debug_assert!(number_of_bits_to_set_in_this_block <= contiguous_zeros_now_available);
		memory_address.set_top_bits_of_u64(initial_block_trailing_zeros);

		let relative_offset_in_number_of_blocks = ((contiguous_blocks_start_from_memory_address.to_usize() * Self::BitsInAByte) - initial_block_trailing_zeros) - self.allocations_start_from_blocks;

		Ok(self.successful_allocation(memory_address, relative_offset_in_number_of_blocks))
	}

	#[inline(always)]
	fn number_of_blocks_is_less_than_64(&self, number_of_blocks_required: NonZeroUsize, memory_address: MemoryAddress, current: u64, current_leading_zeros: usize, contiguous_zeros_now_available: usize, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> Result<MemoryAddress, usize>
	{
		debug_assert_ne!(current_leading_zeros, Self::BitsInAnU64, "current_leading_zeros can not equal BitsInAnU64 `{}` otherwise, since the number of blocks is less than 64, we would have found free space as long as block_alignment_power_of_two_less_minimum does not exceed BitsInAnU64", Self::BitsInAnU64);

		let number_of_blocks_required = number_of_blocks_required.get();
		let bits_to_match = 1 << (number_of_blocks_required as u64) - 1;

		let irrelevant_top_bits_count = current_leading_zeros + 1;
		let aligned_irrelevant_top_bits_count = irrelevant_top_bits_count.round_up_to_power_of_two(block_alignment_power_of_two_less_minimum);

		let top_bits_used = aligned_irrelevant_top_bits_count + number_of_blocks_required;
		if top_bits_used > Self::BitsInAnU64
		{
			return Err(Self::aligned_trailing_zeros(current, block_alignment_power_of_two_less_minimum))
		}

		let maximum_shift = Self::BitsInAnU64 - top_bits_used;
		let mut shift = maximum_shift;
		loop
		{
			let shited_bits_to_match = bits_to_match << shift;
			if (current & shited_bits_to_match) == 0
			{
				memory_address.write(current | shited_bits_to_match);

				let blocks_offset_in_current = Self::BitsInAnU64 - (number_of_blocks_required + shift as usize);
				let relative_offset_in_number_of_blocks = (memory_address.to_usize() * Self::BitsInAByte) + blocks_offset_in_current - self.allocations_start_from_blocks;

				return Ok(self.successful_allocation(memory_address, relative_offset_in_number_of_blocks))
			}

			if unlikely!(shift == 0)
			{
				break
			}

			shift -= block_alignment_power_of_two_less_minimum.get();
		}

		Err(Self::aligned_trailing_zeros(current, block_alignment_power_of_two_less_minimum))
	}

	#[inline(always)]
	fn number_of_blocks_is_64_or_more(&self, _number_of_blocks_required: NonZeroUsize, _memory_address: MemoryAddress, current: u64, current_leading_zeros: usize, contiguous_zeros_now_available: usize, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> Result<MemoryAddress, usize>
	{
		if likely!(current_leading_zeros == Self::SizeOfU64)
		{
			Err(contiguous_zeros_now_available)
		}
		else
		{
			Err(Self::aligned_trailing_zeros(current, block_alignment_power_of_two_less_minimum))
		}
	}

	#[inline(always)]
	fn successful_allocation(&self, memory_address: MemoryAddress, relative_offset_in_number_of_blocks: usize) -> MemoryAddress
	{
		let offset_in_bytes = relative_offset_in_number_of_blocks << self.block_size_power_of_two_exponent;
		self.next_allocation_start_from.set(memory_address);
		self.allocations_start_from.add(offset_in_bytes)
	}

	#[inline(always)]
	fn aligned_trailing_zeros(current: u64, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> usize
	{
		debug_assert!(block_alignment_power_of_two_less_minimum.get() <= Self::BitsInAnU64, "block_alignment_power_of_two_less_minimum `{}` exceeds `{}`", block_alignment_power_of_two_less_minimum, Self::BitsInAnU64);

		let maximum_free_blocks = current.trailing_zeros() as usize;
		maximum_free_blocks.round_down_to_power_of_two(block_alignment_power_of_two_less_minimum)
	}
}

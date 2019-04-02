// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/*
	Real world implementations of free space bitmaps will find ways to centralize information on free space.
		One approach is to split the bitmap into many chunks.
		A separate array then stores the number of free blocks in each chunk, so chunks with insufficient space can be easily skipped over, and the total amount of free space is easier to compute.
		Finding free space now entails searching the summary array first, then searching the associated bitmap chunk for the exact blocks available
			This is very much like using popcnt();

*/


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BitSetWord(u64);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BitSetWordPointer(NonNull<BitSetWord>);

impl BitSetWordPointer
{
	#[inline(always)]
	fn wrap(memory_address: MemoryAddress) -> Self
	{
		debug_assert_eq!(memory_address.to_usize() % BitmapAllocator::SizeOfU64, 0, "memory_address `{}` must be a multiple of 8", memory_address);

		Self(memory_address.cast::<BitSetWord>())
	}

	#[inline(always)]
	fn unset_bottom_bits_of_u64(self, number_of_lower_bits_to_unset: NumberOfBits)
	{
		self.memory_address().unset_bottom_bits_of_u64(number_of_lower_bits_to_unset.0)
	}

	#[inline(always)]
	fn increment(self) -> Self
	{
		self.increment(NumberOfBitSetWords(1))
	}

	#[inline(always)]
	fn increment_in_bit_set_words(self, number_of_bit_set_words: NumberOfBitSetWords) -> Self
	{
		self.increment_in_bytes(number_of_bit_set_words.to_number_of_bytes())
	}

	#[inline(always)]
	fn bit_set_word(self) -> BitSetWord
	{
		BitSetWord(self.memory_address().read_u64())
	}

	#[inline(always)]
	fn increment_in_bytes_non_zero(self, size_in_bytes: NonZeroUsize) -> Self
	{
		self.increment_in_bytes(size_in_bytes.get())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn increment_in_bytes(self, number_of_bytes: NumberOfBytes) -> Self
	{
		let number_of_bytes = number_of_bytes.0;

		debug_assert_eq!(number_of_bytes % Self::SizeOfU64, 0, "number_of_bytes `{:?}` is not a multiple of the size of an u64", number_of_bytes);

		Self(self.memory_address().add(number_of_bytes).cast::<BitSetWord>())
	}
	
	#[doc(hidden)]
	fn memory_address(self) -> MemoryAddress
	{
		self.0.cast::<u8>()
	}
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NumberOfBitSetWords(usize);

impl NumberOfBitSetWords
{
	#[inline(always)]
	fn to_number_of_bytes(self) -> usize
	{
		self.0 * BitmapAllocator::SizeOfU64
	}

	#[inline(always)]
	fn to_number_of_bits(self) -> NumberOfBits
	{
		NumberOfBits(self.0 * BitmapAllocator::BitsInAnU64)
	}
}

impl Sub for NumberOfBitSetWords
{
	type Output = Self;

	#[inline(always)]
	fn sub(&self, other: &Rhs) -> Self::Output
	{
		debug_assert!(self >= other, "self `{:?}` is less than other `{:?}`", self, other);

		Self(self.0 - other.0)
	}
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NumberOfBytes(usize);

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct AbsoluteLocationInBitSet(BitSetWordPointer, NumberOfBits);

impl AbsoluteLocationInBitSet
{
	#[inline(always)]
	fn align_to_next_bit_set_word_pointer<R>(self, value_to_return_if_aligned: R, action_if_unaligned: impl FnOnce(&Self) -> R) -> (Self, R)
	{
		if unlikely!(self.1 == 0)
		{
			(self, value_to_return_if_aligned)
		}
		else
		{
			let value_to_return = action_if_unaligned(&self);
			(Self(self.0.increment(), NumberOfBits(0)), value_to_return);
		}
	}

	#[inline(always)]
	fn unset_bits_between_this_location_and_the_next_bit_set_word_pointer(&self) -> NumberOfBits
	{
		let number_of_lower_bits_to_unset = NumberOfBits(BitmapAllocator::BitsInAnU64 - self.1);
		self.0.unset_bottom_bits_of_u64(number_of_lower_bits_to_unset);
		number_of_lower_bits_to_unset
	}

	#[inline(always)]
	fn unset_all_bits(&self)
	{
		debug_assert_eq!(self.1, NumberOfBits(0), "Can only work for an aligned location");
		// TODO: XXXXX.

		self.0.write_and_advance(0x0000_0000_0000_0000u64);
		asda
		d
		asd
		as
		da
		sd
		ads
	}
}

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RelativeLocationInBitSet(NumberOfBitSetWords, NumberOfBits);

impl RelativeLocationInBitSet
{
	#[inline(always)]
	fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		let blocks_start_from_at_or_within_63_blocks_after_bit_set_word_pointer = inclusive_start_of_bitset.increment_in_bit_set_words(self.0);

		AbsoluteLocationInBitSet(blocks_start_from_at_or_within_63_blocks_after_bit_set_word_pointer, self.1)
	}
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NumberOfBits(usize);

impl Sub for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn sub(&self, other: &Self) -> Self::Output
	{
		debug_assert!(self >= other, "self `{:?}` is less than other `{:?}`", self, other);

		Self(self.0 - other.0)
	}
}

impl SubAssign for NumberOfBits
{
	#[inline(always)]
	fn sub_assign(&mut self, other: Self)
	{
		debug_assert!(self.0 >= other.0, "self `{:?}` is less than other `{:?}`", self, other);

		self.0 -= other.0
	}
}

impl NumberOfBits
{
	#[inline(always)]
	fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		self.to_relative_location_in_bit_set().to_absolute_location_in_bit_set()
	}

	#[inline(always)]
	fn to_relative_location_in_bit_set(self) -> RelativeLocationInBitSet
	{
		let number_of_bit_set_words_rounded_down = self.number_of_bit_set_words_rounded_down();
		let offset_within_bit_set_word = self - number_of_bit_set_words_rounded_down.to_number_of_bits();
		RelativeLocationInBitSet(number_of_bit_set_words_rounded_down, offset_within_bit_set_word)
	}

	#[inline(always)]
	fn less_than_a_bit_set_word_required(self) -> bool
	{
		self.0 < BitmapAllocator::BitsInAnU64
	}

	#[inline(always)]
	fn number_of_bit_set_words_rounded_down(self) -> NumberOfBitSetWords
	{
		NumberOfBitSetWords(self.0 / BitmapAllocator::BitsInAnU64)
	}
}

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
	fn alignment_is_minimum(self, non_zero_power_of_two_alignment: NonZeroUsize) -> bool
	{
		non_zero_power_of_two_alignment <= self.block_size
	}

	#[inline(always)]
	fn number_of_blocks_required(&self, size_to_allocate: NonZeroUsize) -> NumberOfBits
	{
		NumberOfBits((size_to_allocate.get() + self.block_size_less_one) >> self.block_size_power_of_two_exponent)
	}

	#[inline(always)]
	fn blocks_offset(&self, allocations_start_from: MemoryAddress, start_of_allocated_memory: MemoryAddress) -> NumberOfBits
	{
		debug_assert!(start_of_allocated_memory >= allocations_start_from, "start_of_allocated_memory must be >= allocations_start_from");

		NumberOfBits(start_of_allocated_memory.subtract(allocations_start_from) >> self.block_size_power_of_two_exponent)
	}
}

/// Bitmap-based allocator.
#[derive(Debug)]
pub struct BitmapAllocator
{
	inclusive_start_of_bitset: BitSetWordPointer,
	exclusive_end_of_bitset: BitSetWordPointer,
	start_search_for_next_allocation_at: Cell<BitSetWordPointer>,
	
	allocations_start_from: MemoryAddress,
	allocations_start_from_blocks: usize,
	allocations_end_at: MemoryAddress,

	block_size: BlockSize,
}

impl Allocator for BitmapAllocator
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		let desired_alignment_power_of_two_exponent = non_zero_power_of_two_alignment.logarithm_base2();
		if self.block_size.alignment_is_minimum(non_zero_power_of_two_alignment)
		{
			let number_of_blocks_required = self.number_of_blocks_required(non_zero_size);
			self.allocate_number_of_blocks(number_of_blocks_required, 1.non_zero())
		}
		else
		{
			let block_alignment_power_of_two_exponent_less_minimum = desired_alignment_power_of_two_exponent - self.block_size.block_size_power_of_two_exponent;
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
		let location = self.absolute_location_in_bit_set(current_memory);
		let number_of_blocks_required = self.number_of_blocks_required(non_zero_size);

		let (mut location, mut remaining_blocks_to_unset) = location.align_to_next_bit_set_word_pointer(number_of_blocks_required, |location| number_of_blocks_required - location.unset_bits_between_this_location_and_the_next_bit_set_word_pointer());

		while remaining_blocks_to_unset >= NumberOfBits(Self::BitsInAnU64)
		{
			location.unset_all_bits();
			remaining_blocks_to_unset -= NumberOfBits(Self::BitsInAnU64);
		}

		if likely!(remaining_blocks != 0)
		{
			aligned_lots_of_64_blocks_are_contiguous_from_memory_address.unset_top_bits_of_u64(remaining_blocks_to_unset);
		}
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		// TODO
		xxxx; // round to block size;
		
		let would_require_memory_up_to = current_memory.add_non_zero(non_zero_new_size);
		if unlikely!(would_require_memory_up_to > self.allocations_end_at)
		{
			return self.growing_reallocate_by_copying(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}

		let try_to_find_free_memory_after = current_memory.add_non_zero(non_zero_current_size);
		let AbsoluteLocationInBitSet(blocks_start_from_at_or_within_63_blocks_after_bit_set_word_pointer, offset_with_bit_set_word) = self.absolute_location_in_bit_set(try_to_find_free_memory_after);
		let remaining_bits_in_first_u64 = Self::BitsInAnU64 - offset_with_bit_set_word;
		let number_of_additional_blocks_required = self.number_of_blocks_required(non_zero_new_size.difference_non_zero(non_zero_new_size));



		if number_of_additional_blocks_required <= remaining_bits_in_first_u64
		{
			// TODO: Compute mask.



			return if try_to_find_free_memory_after.bit_set_word() & (mask_top_bits | mask_unwanted_bottom_bits) == 0
			{
				// TODO: allocate here and now.
				xxxx;
				Ok(current_memory)
			}
			else
			{
				self.growing_reallocate_by_copying(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
			}
		}

		// TODO: Loop; but be aware of our end-of-bitmap array (or end of memory allocation).
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		// TODO
		xxxx; // round to block size;

		let deallocate_size = non_zero_current_size.difference_non_zero(non_zero_new_size);
		let deallocate_from = current_memory.add(non_zero_new_size);
		self.deallocate(deallocate_size, non_zero_power_of_two_alignment, deallocate_from);
		Ok(current_memory)
	}
}

impl BitmapAllocator
{
	const SizeOfU64: usize = size_of::<u64>();

	const SizeOfU64NonZero: NonZeroUsize = non_zero_usize(Self::SizeOfU64);

	const BitsInAByte: usize = 8;

	const BitsInAnU64: usize = Self::SizeOfU64 * Self::BitsInAByte;

	#[inline(always)]
	pub(crate) fn new(inclusive_start_of_bitset: MemoryAddress, size_in_bytes: NonZeroUsize, allocations_start_from: MemoryAddress, block_size: NonZeroUsize) -> Self
	{
		let inclusive_start_of_bitset = BitSetWordPointer::wrap(inclusive_start_of_bitset);

		Self
		{
			inclusive_start_of_bitset,
			exclusive_end_of_bitset: inclusive_start_of_bitset.increment_in_bytes_non_zero(size_in_bytes),
			start_search_for_next_allocation_at: Cell::new(inclusive_start_of_bitset),

			allocations_start_from,
			allocations_start_from_blocks: inclusive_start_of_bitset.to_usize() * Self::BitsInAByte,
			allocations_end_at: allocations_start_from.add_non_zero(size_in_bytes),

			block_size: BlockSize::new(block_size),
		}
	}

	#[inline(always)]
	fn growing_reallocate_by_copying(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.allocate(non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize);
		// TODO: Finish me
		xxxx
	}

	#[inline(always)]
	fn absolute_location_in_bit_set(&self, start_of_allocated_memory: MemoryAddress) -> AbsoluteLocationInBitSet
	{
		let blocks_offset = self.block_size.blocks_offset(self.allocations_start_from, start_of_allocated_memory);
		blocks_offset.to_absolute_location_in_bit_set(self.inclusive_start_of_bitset)
	}

	#[inline(always)]
	fn number_of_blocks_required(&self, size_to_allocate: NonZeroUsize) -> NumberOfBits
	{
		self.block_size.number_of_blocks_required(size_to_allocate)
	}

	#[inline(always)]
	fn allocate_number_of_blocks(&self, number_of_blocks_required: NumberOfBits, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		macro_rules! scan
		{
			($self: ident, $end_bit_set_word_pointer: ident, $block_alignment_power_of_two_less_minimum: ident, $callback: ident) =>
			{
				{
					let mut contigous_zeros_count = 0;
					let mut bit_set_word_pointer = $self.start_search_for_next_allocation_at.get();
					while bit_set_word_pointer != $end_bit_set_word_pointer
					{
						let current = bit_set_word_pointer.bit_set_word();
						let current_leading_zeros = current.leading_zeros() as usize;
						let contiguous_zeros_now_available = contigous_zeros_count + current_leading_zeros;

						if contiguous_zeros_now_available >= number_of_blocks_required.get()
						{
							return $self.allocate_in_contiguous_leading_zeros(contigous_zeros_count, bit_set_word_pointer, number_of_blocks_required, contiguous_zeros_now_available)
						}

						contigous_zeros_count = match $callback($self, number_of_blocks_required, bit_set_word_pointer, current, current_leading_zeros, contiguous_zeros_now_available, $block_alignment_power_of_two_less_minimum)
						{
							Ok(successful_allocation) => return Ok(successful_allocation),
							Err(contigous_zeros_count) => contigous_zeros_count,
						};
						bit_set_word_pointer = bit_set_word_pointer.add_non_zero(Self::SizeOfU64NonZero);
					}
				}
			}
		}

		let callback = if number_of_blocks_required.less_than_a_bit_set_word_required()
		{
			Self::number_of_blocks_is_less_than_64
		}
		else
		{
			Self::number_of_blocks_is_64_or_more
		};

		let end_bit_set_word_pointer = self.exclusive_end_of_bitset;
		scan!(self, end_bit_set_word_pointer, block_alignment_power_of_two_less_minimum, callback);

		let end_bit_set_word_pointer = self.start_search_for_next_allocation_at.get();
		self.start_search_for_next_allocation_at.set(self.inclusive_start_of_bitset);
		scan!(self, end_bit_set_word_pointer, block_alignment_power_of_two_less_minimum, callback);

		Err(AllocErr)
	}

	#[inline(always)]
	fn allocate_in_contiguous_leading_zeros(&self, contigous_zeros_count: usize, bit_set_word_pointer: MemoryAddress, number_of_blocks_required: NonZeroUsize, contiguous_zeros_now_available: usize) -> Result<MemoryAddress, AllocErr>
	{
		let initial_block_trailing_zeros = contigous_zeros_count % Self::BitsInAnU64;
		let contiguous_blocks_start_from_memory_address = bit_set_word_pointer.subtract
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
		bit_set_word_pointer.set_top_bits_of_u64(initial_block_trailing_zeros);

		let relative_offset_in_number_of_blocks = ((contiguous_blocks_start_from_memory_address.to_usize() * Self::BitsInAByte) - initial_block_trailing_zeros) - self.allocations_start_from_blocks;

		Ok(self.successful_allocation(bit_set_word_pointer, relative_offset_in_number_of_blocks))
	}

	#[inline(always)]
	fn number_of_blocks_is_less_than_64(&self, number_of_blocks_required: NonZeroUsize, bit_set_word_pointer: MemoryAddress, current: u64, current_leading_zeros: usize, contiguous_zeros_now_available: usize, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> Result<MemoryAddress, usize>
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
				bit_set_word_pointer.write(current | shited_bits_to_match);

				let blocks_offset_in_current = Self::BitsInAnU64 - (number_of_blocks_required + shift as usize);
				let relative_offset_in_number_of_blocks = (bit_set_word_pointer.to_usize() * Self::BitsInAByte) + blocks_offset_in_current - self.allocations_start_from_blocks;

				return Ok(self.successful_allocation(bit_set_word_pointer, relative_offset_in_number_of_blocks))
			}

			if unlikely!(shift == 0)
			{
				break
			}

			// TODO: Is this right ?block_alignment_power_of_two_less_minimum? - exponent vs power of two confusion in surrounding code.
			shift -= block_alignment_power_of_two_less_minimum.get();
		}

		Err(Self::aligned_trailing_zeros(current, block_alignment_power_of_two_less_minimum))
	}

	#[inline(always)]
	fn number_of_blocks_is_64_or_more(&self, _number_of_blocks_required: NonZeroUsize, _bit_set_word_pointer: MemoryAddress, current: u64, current_leading_zeros: usize, contiguous_zeros_now_available: usize, block_alignment_power_of_two_less_minimum: NonZeroUsize) -> Result<MemoryAddress, usize>
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
		self.start_search_for_next_allocation_at.set(memory_address);
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

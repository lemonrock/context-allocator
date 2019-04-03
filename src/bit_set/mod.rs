// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


use super::*;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use std::ops::{Add, Shr};


/*
	Real world implementations of free space bitmaps will find ways to centralize information on free space.
		One approach is to split the bitmap into many chunks.
		A separate array then stores the number of free blocks in each chunk, so chunks with insufficient space can be easily skipped over, and the total amount of free space is easier to compute.
		Finding free space now entails searching the summary array first, then searching the associated bitmap chunk for the exact blocks available
			This is very much like using popcnt();

*/



const BitsInAByte: usize = 8;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BitSetWord(u64);

impl BitSetWord
{
	const SizeInBytes: usize = size_of::<u64>();
	
	const SizeInBits: usize = Self::SizeInBytes * BitsInAByte;

	#[inline(always)]
	fn leading_unset_bits(self) -> NumberOfBits
	{
		NumberOfBits(self.0.leading_zeros() as usize)
	}

	#[inline(always)]
	fn trailing_unset_bits(self) -> NumberOfBits
	{
		NumberOfBits(self.0.trailing_zeros() as usize)
	}

	#[inline(always)]
	fn all_unset_but_not_necessarily_contiguous_bits(self) -> NumberOfBits
	{
		NumberOfBits(self.0.count_zeros() as usize)
	}

	#[inline(always)]
	fn to_u64(self) -> u64
	{
		self.0
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BitSetWordPointer(NonNull<BitSetWord>);

impl BitSetWordPointer
{
	#[inline(always)]
	fn wrap(memory_address: MemoryAddress) -> Self
	{
		debug_assert_eq!(memory_address.to_usize() % BitSetWord::SizeInBytes, 0, "memory_address `{:?}` must be a multiple of 8", memory_address);

		Self(memory_address.cast::<BitSetWord>())
	}

	#[inline(always)]
	fn difference_in_number_of_bits(self, lower: Self) -> NumberOfBits
	{
		self.difference_in_number_of_bytes(lower).to_number_of_bits()
	}

	#[inline(always)]
	fn difference_in_number_of_bytes(self, lower: Self) -> NumberOfBytes
	{
		NumberOfBytes(self.memory_address().difference(lower.memory_address()))
	}

	#[inline(always)]
	fn set_bottom_bits(self, number_of_lower_bits_to_set: NumberOfBits)
	{
		self.memory_address().set_bottom_bits_of_u64(number_of_lower_bits_to_set.0)
	}

	#[inline(always)]
	fn set_some_bits(self, current: BitSetWord, bits_to_set: u64)
	{
		self.memory_address().write(current.to_u64() | bits_to_set)
	}

	#[inline(always)]
	fn set_top_bits(self, number_of_upper_bits_to_set: NumberOfBits)
	{
		self.memory_address().set_top_bits_of_u64(number_of_upper_bits_to_set.0)
	}

	#[inline(always)]
	fn set_all_bits_and_increment_assign(&mut self)
	{
		self.set_all_bits_to(0xFFFF_FFFF_FFFF_FFFF)
	}

	#[inline(always)]
	fn unset_bottom_bits(self, number_of_lower_bits_to_unset: NumberOfBits)
	{
		self.memory_address().unset_bottom_bits_of_u64(number_of_lower_bits_to_unset.0)
	}

	#[inline(always)]
	fn unset_middle_bits(self, number_of_bits_to_unset: NumberOfBits, number_of_lower_bits: NumberOfBits)
	{
		self.memory_address().unset_middle_bits_of_u64(number_of_bits_to_unset.0, number_of_lower_bits.0)
	}

	#[inline(always)]
	fn unset_top_bits(self, number_of_upper_bits_to_unset: NumberOfBits)
	{
		self.memory_address().unset_top_bits_of_u64(number_of_upper_bits_to_unset.0)
	}

	#[inline(always)]
	fn unset_all_bits_and_increment_assign(&mut self)
	{
		self.set_all_bits_to(0x0000_0000_0000_0000)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn set_all_bits_to(&mut self, value: u64)
	{
		let mut memory_address = self.memory_address();
		memory_address.write_and_advance(value);
		self.0 = memory_address.cast::<BitSetWord>();
	}

	#[inline(always)]
	fn increment_assign(&mut self)
	{
		*self = (*self).increment()
	}

	#[inline(always)]
	fn increment(self) -> Self
	{
		self.increment_in_bit_set_words(NumberOfBitSetWords::One)
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
		self.increment_in_bytes(NumberOfBytes(size_in_bytes.get()))
	}

	#[inline(always)]
	fn decrement_in_bit_set_words(self, number_of_bit_set_words: NumberOfBitSetWords) -> Self
	{
		self.decrement_in_bytes(number_of_bit_set_words.to_number_of_bytes())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn increment_in_bytes(self, number_of_bytes: NumberOfBytes) -> Self
	{
		let number_of_bytes = number_of_bytes.0;

		debug_assert_eq!(number_of_bytes % BitSetWord::SizeInBytes, 0, "number_of_bytes `{:?}` is not a multiple of the size of an u64", number_of_bytes);

		Self(self.memory_address().add(number_of_bytes).cast::<BitSetWord>())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn decrement_in_bytes(self, number_of_bytes: NumberOfBytes) -> Self
	{
		let number_of_bytes = number_of_bytes.0;

		debug_assert_eq!(number_of_bytes % BitSetWord::SizeInBytes, 0, "number_of_bytes `{:?}` is not a multiple of the size of an u64", number_of_bytes);

		Self(self.memory_address().subtract(number_of_bytes).cast::<BitSetWord>())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn memory_address(self) -> MemoryAddress
	{
		self.0.cast::<u8>()
	}
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NumberOfBitSetWords(usize);

impl NumberOfBitSetWords
{
	const One: Self = Self(1);

	#[inline(always)]
	fn to_number_of_bytes(self) -> NumberOfBytes
	{
		NumberOfBytes(self.0 * BitSetWord::SizeInBytes)
	}

	#[inline(always)]
	fn to_number_of_bits(self) -> NumberOfBits
	{
		NumberOfBits(self.0 * BitSetWord::SizeInBits)
	}
}

impl Sub for NumberOfBitSetWords
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, other: Self) -> Self::Output
	{
		debug_assert!(self >= other, "self `{:?}` is less than other `{:?}`", self, other);

		Self(self.0 - other.0)
	}
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NumberOfBytes(usize);

impl Sub for NumberOfBytes
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, other: Self) -> Self::Output
	{
		debug_assert!(self.0 >= other.0);

		Self(self.0 - other.0)
	}
}

impl NumberOfBytes
{
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self.0 == 0
	}

	#[inline(always)]
	fn is_not_zero(self) -> bool
	{
		self.0 != 0
	}

	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self.0
	}

	#[inline(always)]
	fn to_non_zero(self) -> NonZeroUsize
	{
		self.0.non_zero()
	}

	#[inline(always)]
	fn to_number_of_bits(self) -> NumberOfBits
	{
		NumberOfBits(self.0 * BitsInAByte)
	}
}

/// This is a mixed-radix representation.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct AbsoluteLocationInBitSet
{
	major: BitSetWordPointer,
	minor: NumberOfBits,
}

impl AbsoluteLocationInBitSet
{
	#[inline(always)]
	fn align_upwards_to_next_bit_set_word_pointer<R>(self, value_to_return_if_aligned: R, action_if_unaligned: impl FnOnce(&Self) -> R) -> (BitSetWordPointer, R)
	{
		if unlikely!(self.minor.is_zero())
		{
			(self.major, value_to_return_if_aligned)
		}
		else
		{
			let value_to_return = action_if_unaligned(&self);
			(self.major.increment(), value_to_return)
		}
	}
}

/// This is a mixed-radix representation.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RelativeLocationInBitSet
{
	major: NumberOfBitSetWords,
	minor: NumberOfBits,
}

impl RelativeLocationInBitSet
{
	#[inline(always)]
	fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		AbsoluteLocationInBitSet
		{
			major: inclusive_start_of_bitset.increment_in_bit_set_words(self.major),
			minor: self.minor,
		}
	}
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NumberOfBits(usize);

impl Add for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn add(self, other: Self) -> Self::Output
	{
		Self(self.0 + other.0)
	}
}

impl Add<usize> for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn add(self, other: usize) -> Self::Output
	{
		Self(self.0 + other)
	}
}

impl Sub for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, other: Self) -> Self::Output
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

impl Shr<usize> for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn shr(self, rhs: usize) -> Self::Output
	{
		Self(self.0 >> rhs)
	}
}

impl NumberOfBits
{
	const Zero: Self = Self(0);

	const InBitSetWord: Self = Self(BitSetWord::SizeInBits);

	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self == Self::Zero
	}

	#[inline(always)]
	fn is_not_zero(self) -> bool
	{
		self != Self::Zero
	}

	#[inline(always)]
	fn to_u64(self) -> u64
	{
		self.0 as u64
	}

	#[inline(always)]
	fn remainder_of_bits_that_do_not_fit_in_a_bit_set_word(self) -> Self
	{
		Self(self.0 % BitSetWord::SizeInBits)
	}

	#[inline(always)]
	fn round_up_to_number_of_bit_set_words(self) -> NumberOfBitSetWords
	{
		NumberOfBitSetWords((self.0 + BitSetWord::SizeInBits - 1) / BitSetWord::SizeInBits)
	}

	#[inline(always)]
	fn scale_to_memory_offset_in_bytes(self, block_size: &BlockSize) -> NumberOfBytes
	{
		block_size.scale_to_memory_offset_in_bytes(self.0)
	}

	#[inline(always)]
	fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		self.to_relative_location_in_bit_set().to_absolute_location_in_bit_set(inclusive_start_of_bitset)
	}

	#[inline(always)]
	fn to_relative_location_in_bit_set(self) -> RelativeLocationInBitSet
	{
		let major = self.number_of_bit_set_words_rounded_down();
		let minor = self - major.to_number_of_bits();
		RelativeLocationInBitSet
		{
			major,
			minor
		}
	}

	#[inline(always)]
	fn is_one_bit_set_word(self) -> bool
	{
		self.0 == BitSetWord::SizeInBits
	}

	#[inline(always)]
	fn less_than_a_bit_set_word_required(self) -> bool
	{
		self.0 < BitSetWord::SizeInBits
	}

	#[inline(always)]
	fn number_of_bit_set_words_rounded_down(self) -> NumberOfBitSetWords
	{
		NumberOfBitSetWords(self.0 / BitSetWord::SizeInBits)
	}
}

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

/// Bit set based allocator.
#[derive(Debug)]
pub struct BitSetAllocator
{
	inclusive_start_of_bit_set: BitSetWordPointer,
	exclusive_end_of_bit_set: BitSetWordPointer,
	start_search_for_next_allocation_at: Cell<BitSetWordPointer>,

	allocations_start_from: MemoryAddress,
	allocations_end_at: MemoryAddress,

	block_size: BlockSize,
}

impl Allocator for BitSetAllocator
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		let number_of_bits_required = self.number_of_bits_required(non_zero_size);

		let power_of_two_exponent = if self.block_size.alignment_is_minimum(non_zero_power_of_two_alignment)
		{
			0
		}
		else
		{
			let power_of_two_exponent = non_zero_power_of_two_alignment.logarithm_base2() - self.block_size.block_size_power_of_two_exponent;

			let alignment_exceeds_that_which_can_be_accommodated_in_one_bit_set_word = power_of_two_exponent > BitSetWord::SizeInBits;
			if unlikely!(alignment_exceeds_that_which_can_be_accommodated_in_one_bit_set_word)
			{
				return Err(AllocErr)
			}

			power_of_two_exponent
		};

		self.try_to_set_number_of_bits(number_of_bits_required, power_of_two_exponent)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		#[inline(always)]
		fn unset_unaligned_trailing_bits_at_front(location: AbsoluteLocationInBitSet, number_of_bits_required: NumberOfBits) -> (BitSetWordPointer, NumberOfBits)
		{
			let (location_major, bits_unset_to_reach_alignment) = location.align_upwards_to_next_bit_set_word_pointer(NumberOfBits::Zero, |location|
			{
				let number_of_lower_bits = NumberOfBits::InBitSetWord - location.minor;

				if likely!(number_of_bits_required >= number_of_lower_bits)
				{
					location.major.unset_bottom_bits(number_of_lower_bits);
					number_of_lower_bits
				}
				else
				{
					location.major.unset_middle_bits(number_of_bits_required, number_of_lower_bits);
					number_of_bits_required
				}
			});

			let remaining_bits_to_unset_in_middle_and_at_end = number_of_bits_required - bits_unset_to_reach_alignment;
			(location_major, remaining_bits_to_unset_in_middle_and_at_end)
		}

		#[inline(always)]
		fn unset_aligned_bits_in_middle(mut location_major: BitSetWordPointer, mut remaining_bits_to_unset_in_middle_and_at_end: NumberOfBits) -> (BitSetWordPointer, NumberOfBits)
		{
			while remaining_bits_to_unset_in_middle_and_at_end >= NumberOfBits::InBitSetWord
			{
				location_major.unset_all_bits_and_increment_assign();
				remaining_bits_to_unset_in_middle_and_at_end -= NumberOfBits::InBitSetWord;
			}

			(location_major, remaining_bits_to_unset_in_middle_and_at_end)
		}

		#[inline(always)]
		fn unset_unaligned_leading_bits_at_end(location_major: BitSetWordPointer, remaining_bits_to_unset_at_end: NumberOfBits)
		{
			if likely!(remaining_bits_to_unset_at_end.is_not_zero())
			{
				location_major.unset_top_bits(remaining_bits_to_unset_at_end);
			}
		}

		let location = self.absolute_location_in_bit_set(current_memory);
		let number_of_bits_required = self.number_of_bits_required(non_zero_size);

		let (location_major, remaining_bits_to_unset_in_middle_and_at_end) = unset_unaligned_trailing_bits_at_front(location, number_of_bits_required);
		let (location_major, remaining_bits_to_unset_at_end) = unset_aligned_bits_in_middle(location_major, remaining_bits_to_unset_in_middle_and_at_end);
		unset_unaligned_leading_bits_at_end(location_major, remaining_bits_to_unset_at_end);
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		let current_number_of_bits_required = self.number_of_bits_required(non_zero_current_size);
		let new_number_of_bits_required = self.number_of_bits_required(non_zero_new_size);

		let current_memory_offset_in_bytes = current_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);
		let new_memory_offset_in_bytes = new_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);

		let reallocate_size = new_memory_offset_in_bytes - current_memory_offset_in_bytes;
		if unlikely!(reallocate_size.is_zero())
		{
			return Ok(current_memory)
		}

		self.deallocate(non_zero_current_size, non_zero_power_of_two_alignment, current_memory);
		self.start_search_for_next_allocation_at.set
		({
			let location = self.absolute_location_in_bit_set(current_memory);
			location.major
		});
		let allocated = self.allocate(non_zero_new_size, non_zero_power_of_two_alignment)?;

		if likely!(allocated != current_memory)
		{
			#[inline(always)]
			fn memmove(from: MemoryAddress, to: MemoryAddress, non_zero_current_size: NonZeroUsize)
			{
				unsafe { to.as_ptr().copy_from(from.as_ptr() as *const _, non_zero_current_size.get()) };
			}
			memmove(current_memory, allocated, non_zero_current_size)
		}
		Ok(allocated)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		let current_number_of_bits_required = self.number_of_bits_required(non_zero_current_size);
		let new_number_of_bits_required = self.number_of_bits_required(non_zero_new_size);

		let current_memory_offset_in_bytes = current_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);
		let new_memory_offset_in_bytes = new_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);

		let deallocate_size = current_memory_offset_in_bytes - new_memory_offset_in_bytes;
		if likely!(deallocate_size.is_not_zero())
		{
			let end_of_new_memory = current_memory.add(new_memory_offset_in_bytes.to_usize());
			self.deallocate(deallocate_size.to_non_zero(), non_zero_power_of_two_alignment, end_of_new_memory)
		}
		Ok(current_memory)
	}
}

impl BitSetAllocator
{
	/// Create a new instance.
	#[inline(always)]
	pub fn new(inclusive_start_of_bitset: MemoryAddress, size_in_bytes: NonZeroUsize, allocations_start_from: MemoryAddress, block_size: NonZeroUsize) -> Self
	{
		let inclusive_start_of_bitset = BitSetWordPointer::wrap(inclusive_start_of_bitset);

		Self
		{
			inclusive_start_of_bit_set: inclusive_start_of_bitset,
			exclusive_end_of_bit_set: inclusive_start_of_bitset.increment_in_bytes_non_zero(size_in_bytes),
			start_search_for_next_allocation_at: Cell::new(inclusive_start_of_bitset),

			allocations_start_from,
			allocations_end_at: allocations_start_from.add_non_zero(size_in_bytes),

			block_size: BlockSize::new(block_size),
		}
	}

	#[inline(always)]
	fn absolute_location_in_bit_set(&self, start_of_allocated_memory: MemoryAddress) -> AbsoluteLocationInBitSet
	{
		let blocks_offset = self.block_size.blocks_offset(self.allocations_start_from, start_of_allocated_memory);
		blocks_offset.to_absolute_location_in_bit_set(self.inclusive_start_of_bit_set)
	}

	#[inline(always)]
	fn number_of_bits_required(&self, non_zero_size: NonZeroUsize) -> NumberOfBits
	{
		self.block_size.number_of_blocks_required(non_zero_size)
	}

	#[inline(always)]
	fn try_to_set_number_of_bits(&self, number_of_bits_required: NumberOfBits, power_of_two_exponent: usize) -> Result<MemoryAddress, AllocErr>
	{
		debug_assert!(number_of_bits_required.is_not_zero());

		macro_rules! scan
		{
			($self: ident, $number_of_bits_required: ident, $power_of_two_exponent: ident, $end_bit_set_word_pointer: ident, $callback: ident) =>
			{
				{
					let mut contiguous_unset_bits_count = NumberOfBits::Zero;
					let mut bit_set_word_pointer = $self.start_search_for_next_allocation_at.get();
					while bit_set_word_pointer != $end_bit_set_word_pointer
					{
						let current = bit_set_word_pointer.bit_set_word();

						let current_leading_unset_bits = current.leading_unset_bits();
						let contiguous_unset_bits_now_available = contiguous_unset_bits_count + current_leading_unset_bits;

						// This statement requires no additional corrections as long as alignment can not exceed 64-bits (eg for an 8 byte block, that is an alignment of 512 bytes).
						if contiguous_unset_bits_now_available >= $number_of_bits_required
						{
							return Ok($self.allocate_in_contiguous_unset_bits(contiguous_unset_bits_count, bit_set_word_pointer, $number_of_bits_required))
						}

						contiguous_unset_bits_count = match $callback($self, $number_of_bits_required, bit_set_word_pointer, current, current_leading_unset_bits, contiguous_unset_bits_now_available, $power_of_two_exponent)
						{
							Left(successful_allocation) => return Ok(successful_allocation),

							Right(contiguous_unset_bits_count) => contiguous_unset_bits_count,
						};

						bit_set_word_pointer.increment_assign();
					}
				}
			}
		}

		let callback = if number_of_bits_required.less_than_a_bit_set_word_required()
		{
			Self::number_of_blocks_is_less_than_64
		}
		else
		{
			Self::number_of_blocks_is_64_or_more
		};

		let end_bit_set_word_pointer = self.exclusive_end_of_bit_set;
		scan!(self, number_of_bits_required, power_of_two_exponent, end_bit_set_word_pointer, callback);

		let end_bit_set_word_pointer = self.start_search_for_next_allocation_at.replace(self.inclusive_start_of_bit_set);
		scan!(self, number_of_bits_required, power_of_two_exponent, end_bit_set_word_pointer, callback);

		Err(AllocErr)
	}

	#[inline(always)]
	fn allocate_in_contiguous_unset_bits(&self, bits_to_set_at_front_and_in_middle: NumberOfBits, bit_set_word_pointer: BitSetWordPointer, number_of_bits_required: NumberOfBits) -> MemoryAddress
	{
		#[inline(always)]
		fn set_unaligned_trailing_bits_in_front(bits_to_set_at_front_and_in_middle: NumberOfBits, bit_set_word_pointer: BitSetWordPointer, inclusive_start_of_bit_set: BitSetWordPointer) -> (BitSetWordPointer, NumberOfBits, NumberOfBits)
		{
			let unaligned_trailing_bits_in_front = bits_to_set_at_front_and_in_middle.remainder_of_bits_that_do_not_fit_in_a_bit_set_word();

			let starts_from = bit_set_word_pointer.decrement_in_bit_set_words(bits_to_set_at_front_and_in_middle.round_up_to_number_of_bit_set_words());

			let rounded_down_number_of_bits = starts_from.difference_in_number_of_bits(inclusive_start_of_bit_set);

			if likely!(unaligned_trailing_bits_in_front.is_not_zero())
			{
				starts_from.set_bottom_bits(unaligned_trailing_bits_in_front);
				let offset_into_bit_set = rounded_down_number_of_bits + (NumberOfBits::InBitSetWord - unaligned_trailing_bits_in_front);
				(starts_from.increment(), bits_to_set_at_front_and_in_middle - unaligned_trailing_bits_in_front, offset_into_bit_set)
			}
			else
			{
				(starts_from, bits_to_set_at_front_and_in_middle, rounded_down_number_of_bits)
			}
		}

		#[inline(always)]
		fn set_aligned_bits_in_middle(mut location_major: BitSetWordPointer, mut remaining_bits_to_set_in_middle: NumberOfBits) -> BitSetWordPointer
		{
			while remaining_bits_to_set_in_middle.is_not_zero()
			{
				debug_assert!(remaining_bits_to_set_in_middle >= NumberOfBits::InBitSetWord);

				location_major.set_all_bits_and_increment_assign();
				remaining_bits_to_set_in_middle -= NumberOfBits::InBitSetWord;
			}

			location_major
		}

		#[inline(always)]
		fn set_unaligned_leading_bits_in_end(location_major: BitSetWordPointer, bits_to_set_at_end: NumberOfBits)
		{
			if likely!(bits_to_set_at_end.is_not_zero())
			{
				location_major.set_top_bits(bits_to_set_at_end)
			}
		}

		let (location_major, bits_to_set_in_middle, offset_into_bit_set) = set_unaligned_trailing_bits_in_front(bits_to_set_at_front_and_in_middle, bit_set_word_pointer, self.inclusive_start_of_bit_set);
		let location_major = set_aligned_bits_in_middle(location_major, bits_to_set_in_middle);
		debug_assert_eq!(location_major, bit_set_word_pointer);
		let bits_to_set_at_end = number_of_bits_required - bits_to_set_at_front_and_in_middle;
		set_unaligned_leading_bits_in_end(location_major, bits_to_set_at_end);

		self.successful_allocation(bit_set_word_pointer, offset_into_bit_set)
	}

	#[inline(always)]
	fn number_of_blocks_is_less_than_64(&self, number_of_bits_required: NumberOfBits, bit_set_word_pointer: BitSetWordPointer, current: BitSetWord, current_leading_unset_bits: NumberOfBits, _contiguous_unset_bits_now_available: NumberOfBits, power_of_two_exponent: usize) -> Either<MemoryAddress, NumberOfBits>
	{
		debug_assert!(current_leading_unset_bits < NumberOfBits::InBitSetWord, "If there are 64 leading unset bits, and this allocation is for less than 64 blocks, then it should have been allocated successfully prior to this method");
		debug_assert!(number_of_bits_required > current_leading_unset_bits);

		let quick_check_to_eliminate_most_cases_that_are_likely_to_be_unsuccessful = current.all_unset_but_not_necessarily_contiguous_bits() - current_leading_unset_bits < number_of_bits_required;
		if unlikely!(quick_check_to_eliminate_most_cases_that_are_likely_to_be_unsuccessful)
		{
			return Right(Self::aligned_trailing_unset_bits(current, power_of_two_exponent))
		}

		let (aligned_shift, shift_decrement) =
		{
			let unaligned_shift =
			{
				let lowest_top_bit_count =
				{
					let irrelevant_top_bits_count = current_leading_unset_bits + 1;
					let lowest_top_bit_count = irrelevant_top_bits_count + number_of_bits_required;
					if unlikely!(lowest_top_bit_count > NumberOfBits::InBitSetWord)
					{
						return Right(Self::aligned_trailing_unset_bits(current, power_of_two_exponent))
					}
					lowest_top_bit_count
				};

				(NumberOfBits::InBitSetWord - lowest_top_bit_count).to_u64()
			};

			let shift_decrement = 1 << power_of_two_exponent;

			let too_few_bits_available_for_alignment = unaligned_shift != 0 && unaligned_shift < shift_decrement;
			if unlikely!(too_few_bits_available_for_alignment)
			{
				return Right(Self::aligned_trailing_unset_bits(current, power_of_two_exponent))
			}
			let aligned_shift = unaligned_shift.round_down_to_power_of_two_exponent_usize(power_of_two_exponent);

			(aligned_shift, shift_decrement)
		};

		let unshifted_bits_to_set = (1 << number_of_bits_required.to_u64()) - 1;
		let mut shift = aligned_shift;
		loop
		{
			let bits_to_set = unshifted_bits_to_set << shift;
			let all_bits_to_set_are_currently_unset = current.to_u64() & bits_to_set == 0;
			if all_bits_to_set_are_currently_unset
			{
				return
				{
					bit_set_word_pointer.set_some_bits(current, bits_to_set);

					let offset_into_bit_set =
					{
						let major_location = bit_set_word_pointer.difference_in_number_of_bits(self.inclusive_start_of_bit_set);
						let minor_location = NumberOfBits::InBitSetWord - (number_of_bits_required + NumberOfBits(shift as usize));
						major_location + minor_location
					};

					Left(self.successful_allocation(bit_set_word_pointer, offset_into_bit_set))
				}
			}

			if unlikely!(shift == 0)
			{
				return Right(Self::aligned_trailing_unset_bits(current, power_of_two_exponent))
			}
			shift -= shift_decrement;
		}
	}

	#[inline(always)]
	fn number_of_blocks_is_64_or_more(&self, _number_of_bits_required: NumberOfBits, _bit_set_word_pointer: BitSetWordPointer, current: BitSetWord, current_leading_unset_bits: NumberOfBits, contiguous_unset_bits_now_available: NumberOfBits, power_of_two_exponent: usize) -> Either<MemoryAddress, NumberOfBits>
	{
		if likely!(current_leading_unset_bits.is_one_bit_set_word())
		{
			Right(contiguous_unset_bits_now_available)
		}
		else
		{
			Right(Self::aligned_trailing_unset_bits(current, power_of_two_exponent))
		}
	}

	#[inline(always)]
	fn successful_allocation(&self, bit_set_word_pointer: BitSetWordPointer, offset_into_bit_set: NumberOfBits) -> MemoryAddress
	{
		self.start_search_for_next_allocation_at.set(bit_set_word_pointer);
		self.allocations_start_from.add(offset_into_bit_set.scale_to_memory_offset_in_bytes(&self.block_size).to_usize())
	}

	#[inline(always)]
	fn aligned_trailing_unset_bits(current: BitSetWord, power_of_two_exponent: usize) -> NumberOfBits
	{
		let unaligned_trailing_unset_bits = current.trailing_unset_bits();
		unaligned_trailing_unset_bits >> power_of_two_exponent
	}
}


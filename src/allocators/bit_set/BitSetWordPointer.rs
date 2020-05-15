// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BitSetWordPointer(NonNull<BitSetWord>);

impl BitSetWordPointer
{
	#[inline(always)]
	fn into_memory_address(self) -> MemoryAddress
	{
		self.0.cast()
	}
	
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
	fn decrement_in_bit_set_words(self, number_of_bit_set_words: NumberOfBitSetWords) -> Self
	{
		self.decrement_in_bytes(number_of_bit_set_words.to_number_of_bytes())
	}

	#[inline(always)]
	fn increment_in_bytes(self, number_of_bytes: NumberOfBytes) -> Self
	{
		let number_of_bytes = number_of_bytes.0;

		debug_assert_eq!(number_of_bytes % BitSetWord::SizeInBytes, 0, "number_of_bytes `{:?}` is not a multiple of the size of an u64", number_of_bytes);

		Self(self.memory_address().add(number_of_bytes).cast::<BitSetWord>())
	}

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

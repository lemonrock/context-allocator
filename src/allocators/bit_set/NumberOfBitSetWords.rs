// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


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

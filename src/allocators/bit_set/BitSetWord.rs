// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


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

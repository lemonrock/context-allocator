// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


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

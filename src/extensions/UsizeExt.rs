// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait UsizeExt: Sized + Copy + Ord + Debug
{
	#[inline(always)]
	fn is_odd(self) -> bool
	{
		self.to_usize() & 0b1 == 0b1
	}

	#[inline(always)]
	fn logarithm_base2(self) -> usize
	{
		self.to_usize().trailing_zeros()
	}

	#[inline(always)]
	fn divide_power_of_two_by_power_of_two(self, divisor: usize) -> usize
	{
		debug_assert!(self.is_power_of_two(), "self `{}` is not a power of two", self);
		debug_assert!(divisor.is_power_of_two(), "divisor `{}` is not a power of two", divisor);

		self.to_usize() >> divisor.logarithm_base2()
	}

	#[inline(always)]
	fn non_zero(value: usize) -> Self
	{
		NonZeroUsize::non_zero(value)
	}

	#[doc(hidden)]
	fn to_usize(self) -> usize;
}

impl UsizeExt for usize
{
	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self
	}
}

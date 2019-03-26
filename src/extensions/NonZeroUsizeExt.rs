// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait NonZeroUsizeExt: Sized + Copy + Ord + Debug
{
	#[inline(always)]
	fn next_power_of_two(self) -> Self
	{
		Self::non_zero_unchecked(self.to_usize().next_power_of_two())
	}

	#[inline(always)]
	fn round_up_to_power_of_two(self, non_zero_power_of_two_alignment: NonZeroUsize) -> Self
	{
		let power_of_two = non_zero_power_of_two_alignment.get();
		let power_of_two_less_one = power_of_two - 1;

		debug_assert!(self.checked_add(power_of_two_less_one).is_some(), "non_zero_power_of_two_alignment is far too close to the maximum value of a pointer");

		Self::non_zero(self.add(power_of_two_less_one).to_usize() & !power_of_two_less_one)
	}

	#[inline(always)]
	fn divide_power_of_two_by_power_of_two(self, divisor: NonZeroUsize) -> usize
	{
		debug_assert!(self.is_power_of_two(), "self `{:?}` is not a power of two", self);
		debug_assert!(divisor.is_power_of_two(), "divisor `{:?}` is not a power of two", divisor);

		self.to_usize() >> divisor.logarithm_base2()
	}

	#[inline(always)]
	fn is_power_of_two(self) -> bool
	{
		self.to_usize().is_power_of_two()
	}

	#[inline(always)]
	fn logarithm_base2(self) -> usize
	{
		self.to_usize().trailing_zeros() as usize
	}

	#[inline(always)]
	fn add(self, increment: usize) -> Self
	{
		Self::non_zero(self.to_usize() + increment)
	}

	#[inline(always)]
	fn checked_add(self, increment: usize) -> Option<Self>
	{
		self.to_usize().checked_add(increment).map(Self::non_zero)
	}

	#[inline(always)]
	fn add_assign(&mut self, increment: usize)
	{
		*self = (*self).add(increment)
	}

	#[inline(always)]
	fn doubled(self) -> NonZeroUsize
	{
		(self.to_usize() << 1).non_zero()
	}

	#[inline(always)]
	fn difference(self, other: Self) -> usize
	{
		debug_assert!(self >= other, "other `{:?}` is less than self `{:?}`", other, self);

		self.to_usize() - other.to_usize()
	}

	#[inline(always)]
	fn is_odd(self) -> bool
	{
		self.to_usize().is_odd()
	}

	#[inline(always)]
	fn to_non_zero_u32(self) -> NonZeroU32
	{
		let usize = self.to_usize();
		debug_assert!(usize <= ::std::u32::MAX as usize, "exceeds `{}` ::std::u32::MAX `{}`", usize, ::std::u32::MAX);
		NonZeroU32::non_zero_unchecked(usize as u32)
	}

	#[doc(hidden)]
	fn to_usize(self) -> usize;

	#[inline(always)]
	fn non_zero(value: usize) -> Self
	{
		debug_assert_ne!(value, 0, "value is zero");

		Self::non_zero_unchecked(value)
	}

	fn non_zero_unchecked(value: usize) -> Self;
}

impl NonZeroUsizeExt for NonZeroUsize
{
	#[inline(always)]
	fn difference(self, other: Self) -> usize
	{
		debug_assert!(self >= other, "other `{:?}` is less than self `{:?}`", other, self);

		self.get() - other.get()
	}

	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self.get()
	}

	#[inline(always)]
	fn non_zero_unchecked(value: usize) -> Self
	{
		non_zero_usize(value)
	}
}

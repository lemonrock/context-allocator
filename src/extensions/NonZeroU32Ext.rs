// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait NonZeroU32Ext: Sized + Copy
{
	#[inline(always)]
	fn checked_add(self, increment: Self) -> Option<Self>
	{
		self.to_u32().checked_add(increment.to_u32()).map(Self::non_zero_unchecked)
	}

	#[inline(always)]
	fn add_assign(&mut self, increment: Self)
	{
		*self = Self::non_zero_unchecked(self.to_u32() + increment.to_u32())
	}

	#[inline(always)]
	fn non_zero(value: u32) -> Self
	{
		debug_assert_ne!(value, 0, "value is zero");

		Self::non_zero_unchecked(value)
	}

	fn non_zero_unchecked(value: u32) -> Self;

	#[doc(hidden)]
	fn to_u32(self) -> u32;
}

impl NonZeroU32Ext for NonZeroU32
{
	#[inline(always)]
	fn to_u32(self) -> u32
	{
		self.get()
	}

	#[inline(always)]
	fn non_zero_unchecked(value: u32) -> Self
	{
		unsafe { NonZeroU32::new_unchecked(value) }
	}
}

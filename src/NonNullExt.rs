// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


trait NonNullExt: Sized + Copy + Ord + Debug
{
	#[inline(always)]
	fn round_up_to_power_of_two(self, non_zero_power_of_two_alignment: NonZeroUsize) -> Self
	{
		let power_of_two = non_zero_power_of_two_alignment.get();
		let power_of_two_less_one = power_of_two - 1;

		debug_assert!(self.checked_add(power_of_two_less_one).is_some(), "non_zero_power_of_two_alignment is far too close to the maximum value of a pointer");

		Self::from_usize(self.add(power_of_two_less_one).to_usize() & !power_of_two_less_one)
	}

	#[inline(always)]
	fn add(self, increment: usize) -> Self
	{
		Self::from_usize(self.to_usize() + increment)
	}

	#[inline(always)]
	fn checked_add(self, increment: usize) -> Option<Self>
	{
		self.to_usize().checked_add(increment).map(Self::from_usize)
	}

	#[inline(always)]
	fn add_assign(&mut self, increment: usize)
	{
		*self = (*self).add(increment)
	}

	#[inline(always)]
	fn difference(self, other: Self) -> usize
	{
		debug_assert!(self >= other, "other `{:?}` is less than self `{:?}`", other, self);

		self.to_usize() - other.to_usize()
	}

	#[inline(always)]
	fn read<V: Copy>(&self) -> V
	{
		unsafe { (self.to_usize() as *const V).read() }
	}

	#[inline(always)]
	fn write<V: Copy>(&mut self, value: V)
	{
		unsafe { (self.to_usize() as *mut V).write(value) }
	}

	#[inline(always)]
	fn write_and_advance<V: Copy>(&mut self, value: V)
	{
		self.write(value);
		self.add_assign(size_of::<V>())
	}

	#[inline(always)]
	fn or_u8(self, bits_to_set: u8)
	{
		let pointer = self.to_usize() as *mut u8;
		let current_value = unsafe { *pointer };
		unsafe { *pointer = current_value | bits_to_set }
	}

	#[doc(hidden)]
	fn to_usize(self) -> usize;

	#[doc(hidden)]
	fn from_usize(value: usize) -> Self;
}

impl NonNullExt for NonNull<u8>
{
	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self.as_ptr() as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		unsafe { NonNull::new_unchecked(value as *mut u8) }
	}
}

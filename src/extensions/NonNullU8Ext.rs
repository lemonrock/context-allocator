// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait NonNullU8Ext: Sized + Copy + Ord + Debug
{
	const InvalidOn64BitMaximum: Self;

	#[inline(always)]
	fn round_up_to_power_of_two(self, non_zero_power_of_two_alignment: NonZeroUsize) -> Self
	{
		Self::from_usize(self.to_usize().non_zero().round_up_to_power_of_two(non_zero_power_of_two_alignment).to_usize())
	}

	#[inline(always)]
	fn add(self, increment: usize) -> Self
	{
		Self::from_usize(self.to_usize() + increment)
	}

	#[inline(always)]
	fn add_non_zero(self, increment: NonZeroUsize) -> Self
	{
		Self::from_usize(self.to_usize() + increment.get())
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
	fn add_assign_non_zero(&mut self, increment: NonZeroUsize)
	{
		self.add_assign(increment.get())
	}

	#[inline(always)]
	fn subtract(self, decrement: usize) -> Self
	{
		let usize = self.to_usize();
		debug_assert!(usize >= decrement, "decrement is too large");

		Self::from_usize(usize - decrement)
	}

	#[inline(always)]
	fn difference(self, other: Self) -> usize
	{
		debug_assert!(self >= other, "other `{:?}` is less than self `{:?}`", other, self);

		self.to_usize() - other.to_usize()
	}

	#[inline(always)]
	fn difference_u32(self, other: Self) -> u32
	{
		let difference_usize = self.difference(other);
		debug_assert!(difference_usize <= ::std::u32::MAX as usize, "difference `{}` exceeds ::std::u32::MAX `{}`", difference_usize, ::std::u32::MAX);
		difference_usize as u32
	}

	#[inline(always)]
	fn difference_u32_non_zero(self, other: Self) -> NonZeroU32
	{
		NonZeroU32::non_zero(self.difference_u32(other))
	}

	#[inline(always)]
	fn read<V: Copy>(self) -> V
	{
		unsafe { (self.to_pointer() as *const V).read() }
	}

	#[inline(always)]
	fn read_u64(self) -> u64
	{
		self.read::<u64>()
	}

	#[inline(always)]
	fn write<V: Copy>(self, value: V)
	{
		unsafe { (self.to_pointer() as *mut V).write(value) }
	}

	#[inline(always)]
	fn write_and_advance<V: Copy>(&mut self, value: V)
	{
		self.write(value);
		self.add_assign(size_of::<V>())
	}

	#[inline(always)]
	fn or_u64(self, bits_to_set: u64)
	{
		let current_value = self.read_u64();
		self.write::<u64>(current_value | bits_to_set)
	}

	#[inline(always)]
	fn set_bottom_bits_of_u64(self, number_of_bits_to_set: usize)
	{
		let number_of_bits_to_set = number_of_bits_to_set as u64;
		self.or_u64((1 << number_of_bits_to_set) - 1);
	}

	#[inline(always)]
	fn unset_bottom_bits_of_u64(self, number_of_bits_to_unset: usize)
	{
		let number_of_bits_to_unset = number_of_bits_to_unset as u64;
		let value = self.read_u64();
		let mask = !((1 << number_of_bits_to_unset) - 1);
		self.write::<u64>(value & mask);
	}

	const BitsInAByte: usize = 8;
	const BitsInAnU64: u64 = (size_of::<u64>() * Self::BitsInAByte) as u64;

	#[inline(always)]
	fn set_top_bits_of_u64(self, number_of_bits_to_set: usize)
	{

		let number_of_bits_to_set = number_of_bits_to_set as u64;
		self.or_u64(((1 << number_of_bits_to_set) - 1) << (Self::BitsInAnU64 - number_of_bits_to_set));
	}

	#[inline(always)]
	fn unset_top_bits_of_u64(self, number_of_bits_to_unset: usize)
	{
		let number_of_bits_to_unset = number_of_bits_to_unset as u64;
		let value = self.read_u64();
		let mask = !(((1 << number_of_bits_to_unset) - 1) << (Self::BitsInAnU64 - number_of_bits_to_unset));
		self.write::<u64>(value & mask);
	}

	#[inline(always)]
	fn is_aligned_to(self, non_zero_power_of_two_alignment: NonZeroUsize) -> bool
	{
		let value = self.to_usize();
		let bitmask = non_zero_power_of_two_alignment.get() - 1;

		value & bitmask == 0
	}

	#[inline(always)]
	fn node_pointer(self) -> NodePointer
	{
		NodePointer::from_memory_address(self.to_non_null_u8())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn to_pointer(self) -> *mut u8
	{
		self.to_non_null_u8().as_ptr()
	}

	#[doc(hidden)]
	fn to_non_null_u8(self) -> NonNull<u8>;

	#[doc(hidden)]
	fn to_usize(self) -> usize;

	#[doc(hidden)]
	fn from_usize(value: usize) -> Self;
}

impl NonNullU8Ext for MemoryAddress
{
	const InvalidOn64BitMaximum: Self = unsafe { NonNull::new_unchecked(::std::usize::MAX as *mut u8) };

	#[inline(always)]
	fn to_non_null_u8(self) -> NonNull<u8>
	{
		self
	}

	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self.as_ptr() as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		(value as *mut u8).non_null()
	}
}

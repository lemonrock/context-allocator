// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A helper trait that brings together the core, common functionality required to implement the traits `GlobalAlloc` and `Alloc`.
pub trait Allocator: Debug + Sized
{
	/// The sentinel value used for a zero-sized allocation.
	const ZeroSizedAllocation: MemoryAddress = non_null_pointer(::std::usize::MAX as *mut u8);

	/// Allocate memory.
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>;

	/// Deallocate (free) memory.
	///
	/// The parameter `memory` will never be the value `Self::ZeroSizedAllocation` and will always have been allocated by this `Allocator`.
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress);

	/// Reallocate memory by growing it.
	///
	/// `non_zero_new_size` will always be greater than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>;

	/// Reallocate memory by shrinking it.
	///
	/// `non_zero_new_size` will always be less than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>;

	/// Adapts to a `GlobalAlloc` and `Alloc`.
	#[inline(always)]
	fn adapt<'a>(&'a self) -> AllocatorAdaptor<'a, Self>
	{
		AllocatorAdaptor(self)
	}

	/// Adapts a reference to a `GlobalAlloc` and `Alloc` reference.
	#[inline(always)]
	fn adapt_reference<'a>(&'a self) -> &'a AllocatorAdaptor<'a, Self>
	{
		unsafe { transmute(self) }
	}
}

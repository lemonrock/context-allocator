// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts implementations of `Alloc` to `Allocator`.
pub struct AllocToAllocatorAdaptor<A: Alloc>(UnsafeCell<A>);

impl<A: Alloc> Debug for AllocToAllocatorAdaptor<A>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "AllocToAllocatorAdaptor")
	}
}

impl<A: Alloc> Deref for AllocToAllocatorAdaptor<A>
{
	type Target = A;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.get().reference()
	}
}

impl<A: Alloc> Allocator for AllocToAllocatorAdaptor<A>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		unsafe { self.mutable_reference().alloc(Self::layout(non_zero_size, non_zero_power_of_two_alignment)) }
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		unsafe { self.mutable_reference().dealloc(current_memory, Self::layout(non_zero_size, non_zero_power_of_two_alignment)) }
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		unsafe { self.mutable_reference().realloc(current_memory, Self::layout(non_zero_current_size, non_zero_power_of_two_alignment), non_zero_new_size.get()) }
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		unsafe { self.mutable_reference().realloc(current_memory, Self::layout(non_zero_current_size, non_zero_power_of_two_alignment), non_zero_new_size.get()) }
	}
}

impl<A: Alloc> AllocToAllocatorAdaptor<A>
{
	#[inline(always)]
	fn mutable_reference(&self) -> &mut A
	{
		self.0.get().mutable_reference()
	}

	#[inline(always)]
	fn layout(non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(non_zero_size.get(), non_zero_power_of_two_alignment.get()) }
	}
}

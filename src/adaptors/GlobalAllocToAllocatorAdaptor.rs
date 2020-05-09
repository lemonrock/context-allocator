// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts implementations of `GlobalAlloc` to `Allocator`.
pub struct GlobalAllocToAllocatorAdaptor<GA: GlobalAlloc>(pub GA);

impl<GA: GlobalAlloc> Debug for GlobalAllocToAllocatorAdaptor<GA>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "GlobalAllocToAllocatorAdaptor")
	}
}

impl<GA: GlobalAlloc> Deref for GlobalAllocToAllocatorAdaptor<GA>
{
	type Target = GA;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<GA: GlobalAlloc> Allocator for GlobalAllocToAllocatorAdaptor<GA>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let pointer = unsafe { self.0.alloc(Self::layout(non_zero_size, non_zero_power_of_two_alignment)) };
		Self::adapt_pointer(pointer, non_zero_size)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		unsafe { self.0.dealloc(current_memory.as_ptr(), Self::layout(non_zero_size, non_zero_power_of_two_alignment)) }
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let pointer = unsafe { self.0.realloc(current_memory.as_ptr(), Self::layout(non_zero_current_size, non_zero_power_of_two_alignment), non_zero_new_size.get()) };
		Self::adapt_pointer(pointer, non_zero_new_size)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let pointer = unsafe { self.0.realloc(current_memory.as_ptr(), Self::layout(non_zero_current_size, non_zero_power_of_two_alignment), non_zero_new_size.get()) };
		Self::adapt_pointer(pointer, non_zero_new_size)
	}
}

impl<GA: GlobalAlloc> GlobalAllocToAllocatorAdaptor<GA>
{
	#[inline(always)]
	fn layout(non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(non_zero_size.get(), non_zero_power_of_two_alignment.get()) }
	}

	#[inline(always)]
	fn adapt_pointer(pointer: *mut u8, non_zero_new_size: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		if unlikely!(pointer.is_null())
		{
			Err(AllocErr)
		}
		else
		{
			Ok((unsafe { NonNull::new_unchecked(pointer) }, non_zero_new_size.get()))
		}
	}
}

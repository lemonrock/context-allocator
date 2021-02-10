// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts implementations of `std::alloc::Allocator` (renamed to `Alloc` to avoid a namespace clash and match `GlobalAlloc`) to `Allocator`.
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
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocError>
	{
		Self::map_non_null_slice(self.reference().allocate(Self::layout(non_zero_size, non_zero_power_of_two_alignment)))
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		unsafe { self.reference().deallocate(current_memory, Self::layout(non_zero_size, non_zero_power_of_two_alignment)) }
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, _current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		Self::map_non_null_slice(unsafe { self.reference().grow(current_memory, Self::layout(non_zero_current_size, non_zero_power_of_two_current_alignment), Self::layout(non_zero_new_size, non_zero_power_of_two_new_alignment)) })
	}
	
	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, _current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		Self::map_non_null_slice(unsafe { self.reference().shrink(current_memory, Self::layout(non_zero_current_size, non_zero_power_of_two_current_alignment), Self::layout(non_zero_new_size, non_zero_power_of_two_new_alignment)) })
	}
}

impl<A: Alloc> AllocToAllocatorAdaptor<A>
{
	/// New instance.
	#[inline(always)]
	pub const fn new(underlying: A) -> Self
	{
		Self(UnsafeCell::new(underlying))
	}
	
	#[inline(always)]
	fn reference(&self) -> &A
	{
		self.0.get().reference()
	}

	#[inline(always)]
	fn layout(non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(non_zero_size.get(), non_zero_power_of_two_alignment.get()) }
	}
	
	#[inline(always)]
	fn map_non_null_slice(result: Result<NonNull<[u8]>, AllocError>) -> Result<(NonNull<u8>, usize), AllocError>
	{
		result.map(|non_null_slice| (non_null_slice.as_non_null_ptr(), non_null_slice.len()))
	}
}

impl AllocToAllocatorAdaptor<System>
{
	/// System malloc.
	pub const System: Self = Self::new(System);
}

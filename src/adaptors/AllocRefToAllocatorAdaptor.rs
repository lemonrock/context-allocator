// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts implementations of `AllocRef` to `Allocator`.
pub struct AllocRefToAllocatorAdaptor<A: AllocRef>(UnsafeCell<A>);

impl<A: AllocRef> Debug for AllocRefToAllocatorAdaptor<A>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "AllocToAllocatorAdaptor")
	}
}

impl<A: AllocRef> Deref for AllocRefToAllocatorAdaptor<A>
{
	type Target = A;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.get().reference()
	}
}

impl<A: AllocRef> Allocator for AllocRefToAllocatorAdaptor<A>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		self.mutable_reference().alloc(Self::layout(non_zero_size, non_zero_power_of_two_alignment), AllocInit::Uninitialized).map(|memory_block| (memory_block.ptr, memory_block.size))
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		unsafe { self.mutable_reference().dealloc(current_memory, Self::layout(non_zero_size, non_zero_power_of_two_alignment)) }
	}

	// 	unsafe fn grow(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize, placement: ReallocPlacement, init: AllocInit) -> Result<MemoryBlock, AllocErr>
	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		use self::ReallocPlacement::*;
		let placement = if unlikely!(current_memory_can_not_be_moved)
		{
			InPlace
		}
		else
		{
			MayMove
		};
		(unsafe { self.mutable_reference().grow(current_memory, Self::layout(non_zero_current_size, non_zero_power_of_two_alignment), non_zero_new_size.get(), placement, AllocInit::Uninitialized) }).map(|memory_block| (memory_block.ptr, memory_block.size))
	}
	
	// 	unsafe fn shrink(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize, placement: ReallocPlacement) -> Result<MemoryBlock, AllocErr>
	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		use self::ReallocPlacement::*;
		let placement = if unlikely!(current_memory_can_not_be_moved)
		{
			InPlace
		}
		else
		{
			MayMove
		};
		(unsafe { self.mutable_reference().shrink(current_memory, Self::layout(non_zero_current_size, non_zero_power_of_two_alignment), non_zero_new_size.get(), placement) }).map(|memory_block| (memory_block.ptr, memory_block.size))
	}
}

impl<A: AllocRef> AllocRefToAllocatorAdaptor<A>
{
	/// New instance.
	#[inline(always)]
	pub const fn new(underlying: A) -> Self
	{
		Self(UnsafeCell::new(underlying))
	}
	
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

impl AllocRefToAllocatorAdaptor<System>
{
	/// System malloc.
	pub const System: Self = Self::new(System);
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// An allocator designed for contexts with different lifetimes.
///
/// This allocator NEVER grows or shrinks its memory region.
///
/// This allocator is not thread-safe.
#[derive(Debug)]
pub enum ContextAllocator<MS: MemorySource>
{
	/// Use this variant for contexts with short-lived lifetimes.
	///
	/// Very fast allocation and almost costless deallocation, at the expense of the strong likelihood of running out of memory.
	///
	/// Reallocation is very expensive when growing unless reallocating the most recently made allocation.
	ShortLived(BumpAllocator<MS>),

	/// Use this variant for contexts with slightly longer than short-lived lifetimes.
	///
	/// Slower allocation and deallocation but reallocation is less expensive than for `ShortLived`.
	MediumLived(BitSetAllocator<MS>),

	/// Use this variant for contexts with long-lived lifetimes.
	LongLived(MultipleBinarySearchTreeAllocator<MS>),
}

impl<MS: MemorySource> Allocator for ContextAllocator<MS>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocError>
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),

			MediumLived(ref allocator) => allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),

			LongLived(ref allocator) => allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),
		}
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory),

			MediumLived(ref allocator) => allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory),

			LongLived(ref allocator) => allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory),
		}
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved),

			MediumLived(ref allocator) => allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved),

			LongLived(ref allocator) => allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved),
		}
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved),

			MediumLived(ref allocator) => allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved),

			LongLived(ref allocator) => allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved),
		}
	}
}

impl<MS: MemorySource> LocalAllocator<MS> for ContextAllocator<MS>
{
	#[inline(always)]
	fn new_local_allocator(memory_source: MS, lifetime_hint: LifetimeHint, block_size_hint: NonZeroUsize) -> Self
	{
		use self::LifetimeHint::*;
		
		match lifetime_hint
		{
			ShortLived => ContextAllocator::ShortLived(BumpAllocator::new_local_allocator(memory_source, lifetime_hint, block_size_hint)),
			MediumLived => ContextAllocator::MediumLived(BitSetAllocator::new_local_allocator(memory_source, lifetime_hint, block_size_hint)),
			LongLived => ContextAllocator::LongLived(MultipleBinarySearchTreeAllocator::new_local_allocator(memory_source, lifetime_hint, block_size_hint)),
		}
	}
	
	#[inline(always)]
	fn memory_range(&self) -> MemoryRange
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.memory_range(),

			MediumLived(ref allocator) => allocator.memory_range(),

			LongLived(ref allocator) => allocator.memory_range(),
		}
	}
}

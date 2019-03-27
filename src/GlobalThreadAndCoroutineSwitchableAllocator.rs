// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// An allocator suitable for use as a global allocator.
///
/// Can be controlled so that allocations are global, thread local or coroutine local; overcomes serious limitations of Rust's collections.
#[derive(Debug)]
pub struct GlobalThreadAndCoroutineSwitchableAllocator<CoroutineLocalAllocator: Allocator, ThreadLocalAllocator: Allocator, GlobalAllocator: Allocator>
{
	global_allocator: GlobalAllocator,
	marker: PhantomData<(CoroutineLocalAllocator, ThreadLocalAllocator)>,
}

// These are effectively thread-local fields of the struct `GlobalThreadAndCoroutineSwitchableAllocator`.
#[thread_local] static mut current_allocator_in_use: CurrentAllocatorInUse = CurrentAllocatorInUse::Global;
#[thread_local] static mut coroutine_local_allocator_state: AllocatorState = AllocatorState::new("coroutine local");
#[thread_local] static mut thread_local_allocator_state: AllocatorState = AllocatorState::new("thread local");

impl<CoroutineLocalAllocator: Allocator, ThreadLocalAllocator: Allocator, GlobalAllocator: Allocator> Allocator for GlobalThreadAndCoroutineSwitchableAllocator<CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		use self::CurrentAllocatorInUse::*;

		match Self::save_current_allocator_in_use()
		{
			CoroutineLocal => AllocatorState::coroutine_local_allocator_state().allocate::<CoroutineLocalAllocator>(non_zero_size, non_zero_power_of_two_alignment),

			ThreadLocal => AllocatorState::thread_local_allocator_state().allocate::<ThreadLocalAllocator>(non_zero_size, non_zero_power_of_two_alignment),

			Global => self.global_allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),
		}
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		let coroutine_local_allocator_state_ref = AllocatorState::coroutine_local_allocator_state();
		if likely!(coroutine_local_allocator_state_ref.contains(current_memory))
		{
			return coroutine_local_allocator_state_ref.deallocate::<CoroutineLocalAllocator>(non_zero_size, non_zero_power_of_two_alignment, current_memory)
		}

		let thread_local_allocator_state_ref = AllocatorState::thread_local_allocator_state();
		if likely!(thread_local_allocator_state_ref.contains(current_memory))
		{
			return thread_local_allocator_state_ref.deallocate::<ThreadLocalAllocator>(non_zero_size, non_zero_power_of_two_alignment, current_memory)
		}

		self.global_allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		let coroutine_local_allocator_state_ref = AllocatorState::coroutine_local_allocator_state();
		if likely!(coroutine_local_allocator_state_ref.contains(current_memory))
		{
			return coroutine_local_allocator_state_ref.growing_reallocate::<CoroutineLocalAllocator>(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}

		let thread_local_allocator_state_ref = AllocatorState::thread_local_allocator_state();
		if likely!(thread_local_allocator_state_ref.contains(current_memory))
		{
			return thread_local_allocator_state_ref.growing_reallocate::<ThreadLocalAllocator>(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}

		self.global_allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		let coroutine_local_allocator_state_ref = AllocatorState::coroutine_local_allocator_state();
		if likely!(coroutine_local_allocator_state_ref.contains(current_memory))
		{
			return coroutine_local_allocator_state_ref.shrinking_reallocate::<CoroutineLocalAllocator>(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}

		let thread_local_allocator_state_ref = AllocatorState::thread_local_allocator_state();
		if likely!(thread_local_allocator_state_ref.contains(current_memory))
		{
			return thread_local_allocator_state_ref.shrinking_reallocate::<ThreadLocalAllocator>(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}

		self.global_allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}
}

impl<CoroutineLocalAllocator: Allocator, ThreadLocalAllocator: Allocator, GlobalAllocator: Allocator> GlobalThreadAndCoroutineSwitchableAllocator<CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	/// Obtain the current coroutine local allocator.
	///
	/// May not be valid, of course.
	#[inline(always)]
	pub fn coroutine_local_allocator_unchecked<'a>() -> &'a CoroutineLocalAllocator
	{
		AllocatorState::coroutine_local_allocator_state().allocator()
	}

	/// Obtain the thread local allocator.
	///
	/// May not be valid, of course.
	#[inline(always)]
	pub fn thread_local_allocator_unchecked<'a>() -> &'a ThreadLocalAllocator
	{
		AllocatorState::thread_local_allocator_state().allocator()
	}

	/// Obtain the global allocator.
	///
	/// May not be valid, of course.
	#[inline(always)]
	pub fn global_allocator(&self) -> &GlobalAllocator
	{
		&self.global_allocator
	}

	/// Save the current allocator in use.
	#[inline(always)]
	pub fn save_current_allocator_in_use() -> CurrentAllocatorInUse
	{
		unsafe { current_allocator_in_use }
	}

	/// Restore the current allocator in use.
	#[inline(always)]
	pub fn restore_current_allocator_in_use(restore_to: CurrentAllocatorInUse)
	{
		unsafe { current_allocator_in_use = restore_to }
	}

	/// Save the coroutine local allocator.
	#[inline(always)]
	pub fn save_coroutine_local_allocator() -> AllocatorState
	{
		AllocatorState::save_coroutine_local_allocator()
	}

	/// Restore the coroutine local allocator.
	#[inline(always)]
	pub fn restore_coroutine_local_allocator(allocator_state: AllocatorState)
	{
		AllocatorState::restore_coroutine_local_allocator(allocator_state)
	}
}

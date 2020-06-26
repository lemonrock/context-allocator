// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for a global allocator with the `#[global_allocator]` trait.
///
/// See documentation of `new()`.
#[derive(Debug)]
pub struct GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator>
{
	global_allocator: GlobalAllocator,
	
	per_thread_state: fn() -> NonNull<PerThreadState<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator>>,
	
	marker: PhantomData<(CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator)>,
}

impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> RefUnwindSafe for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
}

unsafe impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> Send for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
}

unsafe impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> Sync for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
}

unsafe impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> GlobalAlloc for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	global_alloc!();
}

unsafe impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> AllocRef for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	alloc_ref!();
}

macro_rules! choose_allocator
{
	($self: ident, $current_memory: ident, $callback: ident, $($argument: ident),*) =>
	{
		{
			if let Some(coroutine_local_allocator) = $self.coroutine_local_allocator()
			{
				if likely!(coroutine_local_allocator.contains($current_memory))
				{
					return coroutine_local_allocator.$callback($($argument, )*)
				}
			}

			if let Some(thread_local_allocator) = $self.thread_local_allocator()
			{
				if likely!(thread_local_allocator.contains($current_memory))
				{
					return thread_local_allocator.$callback($($argument, )*)
				}
			}

			$self.global_allocator().$callback($($argument, )*)
		}
	}
}

impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> Allocator for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		use self::CurrentAllocatorInUse::*;

		match self.save_current_allocator_in_use()
		{
			CoroutineLocal => self.coroutine_local_allocator().expect("Should have assigned a coroutine local allocator").allocate(non_zero_size, non_zero_power_of_two_alignment),

			ThreadLocal => self.thread_local_allocator().expect("Should have assigned a thread local allocator").allocate(non_zero_size, non_zero_power_of_two_alignment),

			Global => self.global_allocator().allocate(non_zero_size, non_zero_power_of_two_alignment),
		}
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		choose_allocator!(self, current_memory, deallocate, non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		choose_allocator!(self, current_memory, growing_reallocate, non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory, current_memory_can_not_be_moved)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		choose_allocator!(self, current_memory, growing_reallocate, non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory, current_memory_can_not_be_moved)
	}
}

impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize> for GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	type CoroutineLocalAllocator = CoroutineLocalAllocator;
	
	type ThreadLocalAllocator = ThreadLocalAllocator;
	
	type GlobalAllocator = GlobalAllocator;
	
	#[inline(always)]
	fn per_thread_state(&self) -> fn() -> NonNull<PerThreadState<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator>>
	{
		self.per_thread_state
	}
}

impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>, GlobalAllocator: Allocator> GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocator>
{
	/// New instance, intended to only be used once to construct a static global allocator field.
	///
	/// Prefer `system()` to this.
	#[inline(always)]
	pub const fn new(global_allocator: GlobalAllocator, per_thread_state: fn() -> NonNull<PerThreadState<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator>>) -> Self
	{
		Self
		{
			global_allocator,
			per_thread_state,
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	fn coroutine_local_allocator(&self) -> Option<&CoroutineLocalAllocator>
	{
		self.use_per_thread_state(|per_thread_state| match &per_thread_state.coroutine_local_allocator
		{
			&Some(ref x) => Some(unsafe { & * (x as *const CoroutineLocalAllocator) }),
			&None => None,
		})
	}

	#[inline(always)]
	fn thread_local_allocator(&self) -> Option<&ThreadLocalAllocator>
	{
		self.use_per_thread_state(|per_thread_state| match &per_thread_state.thread_local_allocator
		{
			&Some(ref x) => Some(unsafe { & * (x as *const ThreadLocalAllocator) }),
			&None => None,
		})
	}
	
	#[inline(always)]
	fn global_allocator(&self) -> &GlobalAllocator
	{
		&self.global_allocator
	}
}

impl<CoroutineHeapSize: MemorySize, CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>, ThreadLocalAllocator: LocalAllocator<MemoryMapSource>> GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, GlobalAllocToAllocatorAdaptor<System>>
{
	/// New instance, intended to only be used once to construct a static global allocator field.
	///
	/// `per_thread_state` is an inlined function to a genuinely thread-local static, viz:-
	///
	/// ```
	/// use context_allocator::allocators::global::PerThreadState;
	/// use std::ptr::NonNull;
	///
	/// #[inline(always)]
	/// fn per_thread_state() -> NonNull<PerThreadState<HeapSize, CoroutineLocalAllocator, ThreadLocalAllocator>>
	/// {
	/// 	#[thread_local] static mut per_thread_state: PerThreadState<HeapSize, CoroutineLocalAllocator, ThreadLocalAllocator> = PerThreadState::empty();
	/// 	unsafe { NonNull::new_unchecked(&mut per_thread_state) }
	/// }
	/// ```
	///
	/// It can be used as follows:-
	/// ```
	///	use context_allocator::allocators::global::GlobalThreadAndCoroutineSwitchableAllocatorInstance;
	/// use std::alloc::System;
	/// #[global_allocator] static GLOBAL: GlobalThreadAndCoroutineSwitchableAllocatorInstance<HeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, System> = GlobalThreadAndCoroutineSwitchableAllocatorInstance::system
	/// (
	/// 	per_thread_state,
	/// );
	/// ```
	#[inline(always)]
	pub const fn system(per_thread_state: fn() -> NonNull<PerThreadState<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator>>) -> Self
	{
		Self::new(GlobalAllocToAllocatorAdaptor::System, per_thread_state)
	}
}

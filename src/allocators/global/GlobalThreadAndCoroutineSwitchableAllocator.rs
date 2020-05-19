// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A trait that all such allocators implement.
///
/// Create a new instance using `GlobalThreadAndCoroutineSwitchableAllocatorInstance`.
pub trait GlobalThreadAndCoroutineSwitchableAllocator<HeapSize: MemorySize>: Sync + GlobalAlloc + AllocRef + Allocator
{
	/// Type of the coroutine local allocator.
	type CoroutineLocalAllocator: LocalAllocator<CoroutineHeapMemorySource<HeapSize>>;

	/// Type of the thread local allocator.
	type ThreadLocalAllocator: LocalAllocator<MemoryMapSource>;

	/// Type of the global allocator.
	type GlobalAllocator: Allocator;

	/// Swaps the coroutine local allocator.
	///
	/// Used before calling a coroutine.
	///
	/// Used after calling a coroutine.
	#[inline(always)]
	fn replace_coroutine_local_allocator(&self, replacement: Option<Self::CoroutineLocalAllocator>) -> Option<Self::CoroutineLocalAllocator>
	{
		self.use_per_thread_state(|per_thread_state| replace(&mut per_thread_state.coroutine_local_allocator, replacement))
	}

	/// Initializes the thread local allocator.
	#[inline(always)]
	fn initialize_thread_local_allocator(&self, thread_local_allocator: Self::ThreadLocalAllocator)
	{
		self.use_per_thread_state(|per_thread_state|
		{
			debug_assert!(per_thread_state.thread_local_allocator.is_none(), "Already initialized thread local allocator");
			
			per_thread_state.thread_local_allocator = Some(thread_local_allocator)
		})
	}

	/// Drops the thread local allocator.
	///
	/// Panics in debug if no thread local allocator has been initialized with `initialize_thread_local_allocator()`.
	#[inline(always)]
	fn drop_thread_local_allocator(&self)
	{
		self.use_per_thread_state(|per_thread_state|
		{
			debug_assert!(per_thread_state.thread_local_allocator.is_some(), "Already deinitialized thread local allocator");
			
			per_thread_state.thread_local_allocator = None
		})
	}

	/// Save the current allocator in use.
	#[inline(always)]
	fn save_current_allocator_in_use(&self) -> CurrentAllocatorInUse
	{
		self.use_per_thread_state(|per_thread_state| per_thread_state.current_allocator_in_use)
	}

	/// Restore the current allocator in use.
	#[inline(always)]
	fn restore_current_allocator_in_use(&self, restore_to: CurrentAllocatorInUse)
	{
		self.use_per_thread_state(|per_thread_state| per_thread_state.current_allocator_in_use = restore_to)
	}

	/// Replace the current allocator in use.
	#[inline(always)]
	fn replace_current_allocator_in_use(&self, replacement: CurrentAllocatorInUse) -> CurrentAllocatorInUse
	{
		let was = self.save_current_allocator_in_use();
		self.restore_current_allocator_in_use(replacement);
		was
	}

	/// Switch the current allocator in use to coroutine local and execute the callback; restore it after calling the callback unless a panic occurs.
	#[inline(always)]
	fn callback_with_coroutine_local_allocator<R>(&self, callback: impl FnOnce() -> R) -> R
	{
		self.callback_with_different_current_allocator(CurrentAllocatorInUse::CoroutineLocal, callback)
	}

	/// Switch the current allocator in use to thread local and execute the callback; restore it after calling the callback unless a panic occurs.
	#[inline(always)]
	fn callback_with_thread_local_allocator<R>(&self, callback: impl FnOnce() -> R) -> R
	{
		self.callback_with_different_current_allocator(CurrentAllocatorInUse::ThreadLocal, callback)
	}

	/// Switch the current allocator in use to global and execute the callback; restore it after calling the callback unless a panic occurs.
	#[inline(always)]
	fn callback_with_global_allocator<R>(&self, callback: impl FnOnce() -> R) -> R
	{
		self.callback_with_different_current_allocator(CurrentAllocatorInUse::Global, callback)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn callback_with_different_current_allocator<R>(&self, different: CurrentAllocatorInUse, callback: impl FnOnce() -> R) -> R
	{
		let restore_to = self.save_current_allocator_in_use();
		self.restore_current_allocator_in_use(different);
		let result = callback();
		self.restore_current_allocator_in_use(restore_to);
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn use_per_thread_state<User: FnOnce(&mut PerThreadState<HeapSize, Self::CoroutineLocalAllocator, Self::ThreadLocalAllocator>) -> R, R>(&self, user: User) -> R
	{
		unsafe { user(&mut * (self.per_thread_state())().as_ptr()) }
	}
	
	#[doc(hidden)]
	fn per_thread_state(&self) -> fn() -> NonNull<PerThreadState<HeapSize, Self::CoroutineLocalAllocator, Self::ThreadLocalAllocator>>;
}

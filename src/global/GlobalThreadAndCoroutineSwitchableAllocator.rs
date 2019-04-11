// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A trait that all such allocators implement.
///
/// Create a new instance using the macro `global_thread_and_coroutine_switchable_allocator`.
pub trait GlobalThreadAndCoroutineSwitchableAllocator: Sync + GlobalAlloc + Alloc + Allocator
{
	/// Type of the coroutine local allocator.
	type CoroutineLocalAllocator: LocalAllocator;

	/// Type of the thread local allocator.
	type ThreadLocalAllocator: LocalAllocator;

	/// Type of the global allocator.
	type GlobalAllocator: Allocator;

	/// Swaps the coroutine local allocator.
	///
	/// Used before calling a coroutine.
	///
	/// Used after calling a coroutine.
	fn replace_coroutine_local_allocator(&self, replacement: Option<Self::CoroutineLocalAllocator>) -> Option<Self::CoroutineLocalAllocator>;

	/// Initializes the thread local allocator.
	fn initialize_thread_local_allocator(&self, thread_local_allocator: Self::ThreadLocalAllocator);

	/// Drops the thread local allocator.
	///
	/// Panics in debug if no thread local allocator has been initialized with `initialize_thread_local_allocator()`.
	#[inline(always)]
	fn drop_thread_local_allocator(&self);

	/// Save the current allocator in use.
	#[inline(always)]
	fn save_current_allocator_in_use(&self) -> CurrentAllocatorInUse;

	/// Restore the current allocator in use.
	#[inline(always)]
	fn restore_current_allocator_in_use(&self, restore_to: CurrentAllocatorInUse);

	/// Obtain the current coroutine local allocator.
	///
	/// Panics in debug if no coroutine local allocator has been initialized with `initialize_coroutine_local_allocator()`.
	#[inline(always)]
	fn coroutine_local_allocator(&self) -> &Self::CoroutineLocalAllocator;

	/// Obtain the thread local allocator.
	///
	/// Panics in debug if no thread local allocator has been initialized with `initialize_thread_local_allocator()`.
	#[inline(always)]
	fn thread_local_allocator(&self) -> &Self::ThreadLocalAllocator;

	/// Obtain the global allocator.
	#[inline(always)]
	fn global_allocator(&self) -> &Self::GlobalAllocator;
}

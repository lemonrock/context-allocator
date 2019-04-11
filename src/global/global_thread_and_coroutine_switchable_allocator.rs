// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Creates a new global, switchable allocator inside a module `$mod_name`.
///
/// Parameters:-
///
/// * `$mod_name`: A `pub(crate)` module.
/// * `$CoroutineLocalAllocator`: the type of the coroutine local allocator. Must implement `LocalAllocator`.
/// * `$ThreadLocalAllocator`: the type of the thread local allocator. Must implement `LocalAllocator`.
/// * `$GlobalAllocator`: the type of the thread local allocator. Must implement `Allocator`; a common usage is `GlobalAllocToAllocatorAdaptor<System>`.
/// * `global_allocator_instance`: a constant expression for instantiating the global allocator. A common usage is `GlobalAllocToAllocatorAdaptor(System)`.
///
/// To access the switchable allocator, call `$mod_name::global_thread_and_coroutine_switchable_allocator()`; this returns an object reference that implements the trait `GlobalThreadAndCoroutineSwitchableAllocator`.
///
/// Done using a macro due to a limitation when combining thread-local statics with generics (which could be solved using pthread keys, but these aren't always the most efficient of approaches); in essence, a thread-local struct field is needed.
///
/// # Example
///
/// ```Rust
///
/// global_thread_and_coroutine_switchable_allocator!(MyGlobalAllocator, BumpAllocator<ArenaMemorySource<MemoryMapAllocator>>, MultipleBinarySearchTreeAllocator<MemoryMapAllocator>, GlobalAllocToAllocatorAdaptor<System>, GlobalAllocToAllocatorAdaptor(System));
///
/// ...
///
/// let global = MyGlobalAllocator::global_thread_and_coroutine_switchable_allocator();
///
/// ...
///
/// ```
#[macro_export]
macro_rules! global_thread_and_coroutine_switchable_allocator
{
	($mod_name: ident, $CoroutineLocalAllocator: ty, $ThreadLocalAllocator: ty, $GlobalAllocator: ty, $global_allocator_instance: expr) =>
	{
		#[global_allocator] static GLOBAL: $mod_name::GlobalThreadAndCoroutineSwitchableAllocatorInstance = $mod_name::GlobalThreadAndCoroutineSwitchableAllocatorInstance
		{
			global_allocator: $global_allocator_instance,
		};

		pub(crate) mod $mod_name
		{
			use $crate::*;
			use ::std::mem::replace;

			/// Effectively this is a field of `GlobalThreadAndCoroutineSwitchableAllocatorInstance` with a different value for each thread.
			///
			/// It is this piece of logic that necessitates this macro definition.
			#[thread_local] static mut per_thread_state: PerThreadState<$CoroutineLocalAllocator, $ThreadLocalAllocator> = PerThreadState::empty();
			
			#[derive(Debug)]
			pub(crate) struct GlobalThreadAndCoroutineSwitchableAllocatorInstance
			{
				pub(crate) global_allocator: $GlobalAllocator,
			}

			unsafe impl Sync for GlobalThreadAndCoroutineSwitchableAllocatorInstance
			{
			}

			unsafe impl GlobalAlloc for GlobalThreadAndCoroutineSwitchableAllocatorInstance
			{
				global_alloc!();
			}

			unsafe impl Alloc for GlobalThreadAndCoroutineSwitchableAllocatorInstance
			{
				alloc!();
			}

			impl Allocator for GlobalThreadAndCoroutineSwitchableAllocatorInstance
			{
				#[inline(always)]
				fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
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
				fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
				{
					choose_allocator!(self, current_memory, deallocate, non_zero_size, non_zero_power_of_two_alignment, current_memory)
				}

				#[inline(always)]
				fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
				{
					choose_allocator!(self, current_memory, growing_reallocate, non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
				}

				#[inline(always)]
				fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
				{
					choose_allocator!(self, current_memory, growing_reallocate, non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
				}
			}

			impl GlobalThreadAndCoroutineSwitchableAllocator for GlobalThreadAndCoroutineSwitchableAllocatorInstance
			{
				type CoroutineLocalAllocator = $CoroutineLocalAllocator;

				type ThreadLocalAllocator = $ThreadLocalAllocator;

				type GlobalAllocator = $GlobalAllocator;

				#[inline(always)]
				fn replace_coroutine_local_allocator(&self, replacement: Option<Self::CoroutineLocalAllocator>) -> Option<Self::CoroutineLocalAllocator>
				{
					unsafe { replace(&mut per_thread_state.coroutine_local_allocator, replacement) }
				}

				#[inline(always)]
				fn initialize_thread_local_allocator(&self, thread_local_allocator: Self::ThreadLocalAllocator)
				{
					debug_assert!(unsafe { per_thread_state.thread_local_allocator.is_none() }, "Already initialized thread local allocator");

					unsafe { per_thread_state.thread_local_allocator = Some(thread_local_allocator) }
				}

				#[inline(always)]
				fn drop_thread_local_allocator(&self)
				{
					debug_assert!(unsafe { per_thread_state.thread_local_allocator.is_some() }, "Already deinitialized thread local allocator");

					unsafe { per_thread_state.thread_local_allocator = None }
				}

				#[inline(always)]
				fn save_current_allocator_in_use(&self) -> CurrentAllocatorInUse
				{
					unsafe { per_thread_state.current_allocator_in_use }
				}

				#[inline(always)]
				fn restore_current_allocator_in_use(&self, restore_to: CurrentAllocatorInUse)
				{
					unsafe { per_thread_state.current_allocator_in_use = restore_to }
				}

				#[inline(always)]
				fn coroutine_local_allocator(&self) -> Option<&Self::CoroutineLocalAllocator>
				{
					unsafe { per_thread_state.coroutine_local_allocator.as_ref() }
				}

				#[inline(always)]
				fn thread_local_allocator(&self) -> Option<&Self::ThreadLocalAllocator>
				{
					unsafe { per_thread_state.thread_local_allocator.as_ref() }
				}

				#[inline(always)]
				fn global_allocator(&self) -> &Self::GlobalAllocator
				{
					&self.global_allocator
				}
			}
		}
	}
}

#[doc(hidden)]
#[macro_export]
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

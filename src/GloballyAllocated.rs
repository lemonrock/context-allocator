// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Needed for `Vec`, `String`, `Arc`, `Rc` and the like when shared between threads in messages.
///
/// This is because `String` and `Vec`, for example, don't allocate if constructed with an empty capacity; thus it is possible to create an instance whilst in the context, say, of a ThreadAllocator but then resize in the context of a GlobalAllocator.
/// Since no allocation has occurred, the ThreadAllocator won't be tracking the memory for, say, the `String`.
/// Hence memory gets freed in a thread-unsafe manner.
/// Oops!
#[derive(Debug)]
pub struct GloballyAllocated<T: RefUnwindSafe, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	value: ManuallyDrop<T>,
	global_allocator: &'static GTACSA,
	marker: PhantomData<HeapSize>,
}

impl<T: RefUnwindSafe, HeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> Drop for GloballyAllocated<T, HeapSize, GTACSA>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_mut(|t| unsafe { drop_in_place(t) })
	}
}

impl<T: RefUnwindSafe, HeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> Deref for GloballyAllocated<T, HeapSize, GTACSA>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.value.deref()
	}
}

impl<T: RefUnwindSafe, HeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> GloballyAllocated<T, HeapSize, GTACSA>
{
	/// Allocate.
	#[inline(always)]
	pub fn allocate(global_allocator: &'static GTACSA, callback: impl FnOnce() -> T + UnwindSafe) -> Self
	{
		Self
		{
			value: ManuallyDrop::new(global_allocator.callback_with_global_allocator(callback)),
			global_allocator,
			marker: PhantomData,
		}
	}
	
	/// Mutable reference.
	#[inline(always)]
	pub fn as_mut<F: FnOnce(&mut T) -> R + UnwindSafe, R>(&mut self, callback: F) -> R
	{
		self.global_allocator.callback_with_global_allocator(AssertUnwindSafe(|| callback(self.value.deref_mut())))
	}
}

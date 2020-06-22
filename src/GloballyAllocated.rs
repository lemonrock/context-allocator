// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Needed for `Vec`, `String`, `Arc`, `Rc` and the like when shared between threads in messages.
///
/// This is because `String` and `Vec`, for example, don't allocate if constructed with an empty capacity; thus it is possible to create an instance whilst in the context, say, of a ThreadAllocator but then resize in the context of a GlobalAllocator.
/// Since no allocation has occurred, the ThreadAllocator won't be tracking the memory for, say, the `String`.
/// Hence memory gets freed in a thread-unsafe manner.
/// Oops!
pub struct GloballyAllocated<T: RefUnwindSafe, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	value: ManuallyDrop<T>,
	global_allocator: &'static GTACSA,
	marker: PhantomData<CoroutineHeapSize>,
}

impl<T: RefUnwindSafe, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Drop for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_mut(|t| unsafe { drop_in_place(t) })
	}
}

impl<T: RefUnwindSafe, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Deref for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.value.deref()
	}
}

impl<T: RefUnwindSafe + Debug, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Debug for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.deref().fmt(f)
	}
}

impl<T: RefUnwindSafe + Display, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Display for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.deref().fmt(f)
	}
}

impl<T: RefUnwindSafe + PartialEq, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> PartialEq for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.deref() == rhs.deref()
	}
}

impl<T: RefUnwindSafe + Eq, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Eq for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
}

impl<T: RefUnwindSafe + PartialOrd, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> PartialOrd for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.deref().partial_cmp(rhs.deref())
	}
}

impl<T: RefUnwindSafe + Ord, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Ord for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.deref().cmp(rhs.deref())
	}
}

impl<T: RefUnwindSafe + Hash, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Hash for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.deref().hash(state)
	}
}

impl<T: RefUnwindSafe + Clone, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Clone for GloballyAllocated<T, CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::allocate(self.global_allocator, ||
		{
			self.value.deref().clone()
		})
	}
}

impl<T: RefUnwindSafe, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> GloballyAllocated<T, CoroutineHeapSize, GTACSA>
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

impl<T: RefUnwindSafe, CoroutineHeapSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>  GloballyAllocated<Arc<T>, CoroutineHeapSize, GTACSA>
{
	/// Clone specialized for `Arc<T>`.
	#[inline(always)]
	pub fn clone_arc(&self) -> Self
	{
		Self
		{
			value: ManuallyDrop::new(self.value.deref().clone()),
			global_allocator: self.global_allocator,
			marker: PhantomData,
		}
	}
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Heap memory.
///
/// We align to the most common page size, 4Kb, which will minimize alignment problems of memory allocations from this heap.
#[repr(C, align(4096))]
pub struct CoroutineHeapMemory<HeapSize: MemorySize>
{
	sizing: HeapSize
}

impl<HeapSize: MemorySize> Debug for CoroutineHeapMemory<HeapSize>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "CoroutineHeapMemorySource({})", size_of::<Self>())
	}
}

impl<HeapSize: MemorySize> CoroutineHeapMemory<HeapSize>
{
	/// Into a memory source.
	#[inline(always)]
	pub const fn into_memory_source(&self) -> CoroutineHeapMemorySource<HeapSize>
	{
		CoroutineHeapMemorySource(unsafe { NonNull::new_unchecked(self as *const CoroutineHeapMemory<HeapSize> as *mut CoroutineHeapMemory<HeapSize>) })
	}
}

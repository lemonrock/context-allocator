// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Heap memory.
///
/// We align to the most common page size, 4Kb, which will minimize alignment problems of memory allocations from this heap.
#[repr(C, align(4096))]
pub struct CoroutineHeapMemory<CoroutineHeapSize: MemorySize>
{
	sizing: CoroutineHeapSize
}

impl<CoroutineHeapSize: MemorySize> Debug for CoroutineHeapMemory<CoroutineHeapSize>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "CoroutineHeapMemorySource({})", size_of::<Self>())
	}
}

impl<CoroutineHeapSize: MemorySize> CoroutineHeapMemory<CoroutineHeapSize>
{
	/// Into a memory source.
	#[inline(always)]
	pub const fn into_memory_source(&self) -> CoroutineHeapMemorySource<CoroutineHeapSize>
	{
		CoroutineHeapMemorySource(new_non_null(self as *const CoroutineHeapMemory<CoroutineHeapSize> as *mut CoroutineHeapMemory<CoroutineHeapSize>))
	}
}

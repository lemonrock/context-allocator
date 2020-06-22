// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Heap memory.
///
/// We align to the most common page size, 4Kb, which will minimize alignment problems of memory allocations from this heap.
#[derive(Debug)]
#[repr(C, align(4096))]
pub struct CoroutineHeapMemorySource<CoroutineHeapSize: MemorySize>(NonNull<CoroutineHeapMemory<CoroutineHeapSize>>);

impl<CoroutineHeapSize: MemorySize> MemorySource for CoroutineHeapMemorySource<CoroutineHeapSize>
{
	#[inline(always)]
	fn size(&self) -> NonZeroUsize
	{
		let size = size_of::<CoroutineHeapMemory<CoroutineHeapSize>>();
		debug_assert_ne!(size, 0, "Unsized values are not supported");
		unsafe { NonZeroUsize::new_unchecked(size) }
	}
	
	#[inline(always)]
	fn allocations_start_from(&self) -> MemoryAddress
	{
		self.0.cast()
	}
}

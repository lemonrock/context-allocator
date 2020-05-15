// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A wrapper type.
pub type CoroutineHeapMemorySource<HeapSize> = ReferenceCountedLargeRingQueueElement<CoroutineHeapMemory<HeapSize>>;

impl<HeapSize: Sized> MemorySource for CoroutineHeapMemorySource<HeapSize>
{
	#[inline(always)]
	fn size(&self) -> NonZeroUsize
	{
		let size = size_of::<CoroutineHeapMemory<HeapSize>>();
		debug_assert_ne!(size, 0, "Unsized values are not supported");
		unsafe { NonZeroUsize::new_unchecked(size) }
	}
	
	#[inline(always)]
	fn allocations_start_from(&self) -> MemoryAddress
	{
		unsafe { self.element() }.cast()
	}
}

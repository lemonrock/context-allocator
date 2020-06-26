// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[doc(hidden)]
#[macro_export]
macro_rules! alloc_ref
{
	() =>
	{
		#[inline(always)]
		fn alloc(&mut self, layout: Layout, init: AllocInit)-> Result<MemoryBlock, AllocErr>
		{
			self.AllocRef_alloc(layout, init)
		}
		
		#[inline(always)]
		unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout)
		{
			self.AllocRef_dealloc(ptr, layout)
		}
		
		#[inline(always)]
		unsafe fn grow(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize, placement: ReallocPlacement, init: AllocInit) -> Result<MemoryBlock, AllocErr>
		{
			self.AllocRef_grow(ptr, layout, new_size, placement, init)
		}

		#[inline(always)]
		unsafe fn shrink(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize, placement: ReallocPlacement) -> Result<MemoryBlock, AllocErr>
		{
			self.AllocRef_shrink(ptr, layout, new_size, placement)
		}
	}
}

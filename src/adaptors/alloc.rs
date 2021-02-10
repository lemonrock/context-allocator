// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[doc(hidden)]
#[macro_export]
macro_rules! alloc
{
	() =>
	{
		#[inline(always)]
		fn allocate(&self, layout: Layout)-> Result<NonNull<[u8]>, AllocError>
		{
			self.Alloc_allocate(layout)
		}
		
		#[inline(always)]
		unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout)
		{
			self.Alloc_deallocate(ptr, layout)
		}
		
		#[inline(always)]
		unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>
		{
			self.Alloc_grow(ptr, old_layout, new_layout)
		}

		#[inline(always)]
		unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>
		{
			self.Alloc_shrink(ptr, old_layout, new_layout)
		}
	}
}

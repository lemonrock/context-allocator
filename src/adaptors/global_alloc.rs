// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[doc(hidden)]
#[macro_export]
macro_rules! global_alloc
{
	() =>
	{
		#[inline(always)]
		unsafe fn alloc(&self, layout: Layout) -> *mut u8
		{
			self.GlobalAlloc_alloc(layout)
		}

		#[inline(always)]
		unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
		{
			self.GlobalAlloc_alloc_zeroed(layout)
		}

		#[inline(always)]
		unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
		{
			self.GlobalAlloc_dealloc(ptr, layout)
		}

		#[inline(always)]
		unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
		{
			self.GlobalAlloc_realloc(ptr, layout, new_size)
		}
	}
}

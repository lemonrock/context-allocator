// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[doc(hidden)]
#[macro_export]
macro_rules! alloc
{
	() =>
	{
		#[inline(always)]
		fn alloc(&mut self, layout: Layout) -> Result<(NonNull<u8>, usize), AllocErr>
		{
			self.AllocRef_alloc(layout)
		}

		#[inline(always)]
		fn alloc_zeroed(&mut self, layout: Layout) -> Result<(NonNull<u8>, usize), AllocErr>
		{
			self.AllocRef_alloc_zeroed(layout)
		}

		#[inline(always)]
		unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout)
		{
			self.AllocRef_dealloc(ptr, layout)
		}

		#[inline(always)]
		unsafe fn realloc(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<(NonNull<u8>, usize), AllocErr>
		{
			self.AllocRef_realloc(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn realloc_zeroed(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<(NonNull<u8>, usize), AllocErr>
		{
			self.AllocRef_realloc_zeroed(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn grow_in_place(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<usize, CannotReallocInPlace>
		{
			self.AllocRef_grow_in_place(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn grow_in_place_zeroed(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<usize, CannotReallocInPlace>
		{
			self.AllocRef_grow_in_place_zeroed(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn shrink_in_place(&mut self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<usize, CannotReallocInPlace>
		{
			self.AllocRef_shrink_in_place(ptr, layout, new_size)
		}
	}
}

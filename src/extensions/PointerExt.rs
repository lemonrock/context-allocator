// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait PointerExt<T>: Sized
{
	fn non_null(self) -> NonNull<T>;

	fn add_bytes(self, offset: usize) -> Self;

	#[inline(always)]
	fn add_bytes_u32(self, offset: u32) -> Self
	{
		self.add_bytes(offset as usize)
	}

	#[inline(always)]
	fn add_bytes_non_zero_u32(self, offset: NonZeroU32) -> Self
	{
		self.add_bytes_u32(offset.get())
	}
}

impl<T> PointerExt<T> for *const T
{
	#[inline(always)]
	fn non_null(self) -> NonNull<T>
	{
		non_null_pointer(self as *mut T)
	}

	#[inline(always)]
	fn add_bytes(self, offset: usize) -> Self
	{
		((self as usize) + offset) as *const T
	}
}

impl<T> PointerExt<T> for *mut T
{
	#[inline(always)]
	fn non_null(self) -> NonNull<T>
	{
		non_null_pointer(self)
	}

	#[inline(always)]
	fn add_bytes(self, offset: usize) -> Self
	{
		((self as usize) + offset) as *mut T
	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait NonNullExt<T>
{
	fn reference<'any>(self) -> &'any T;

	fn mutable_reference<'any>(self) -> &'any mut T;
}

impl<T> NonNullExt<T> for NonNull<T>
{
	#[inline(always)]
	fn reference<'any>(self) -> &'any T
	{
		unsafe { & * self.as_ptr() }
	}

	#[inline(always)]
	fn mutable_reference<'any>(self) -> &'any mut T
	{
		unsafe { &mut * self.as_ptr() }
	}
}

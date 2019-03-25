// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait PointerMutExt<T>: PointerExt<T>
{
	fn mutable_reference<'a>(self) -> &'a mut T;
}

impl<T> PointerMutExt<T> for *mut T
{
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut T
	{
		debug_assert!(self.is_not_null(), "null pointers can not be derefenced");

		unsafe { &mut * self }
	}
}

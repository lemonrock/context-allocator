// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait UsizeExt: Sized + Copy + Ord + Debug
{
	#[inline(always)]
	fn is_odd(self) -> bool
	{
		self.to_usize() & 0b1 == 0b1
	}

	#[inline(always)]
	fn non_zero(self) -> NonZeroUsize
	{
		NonZeroUsize::non_zero(self.to_usize())
	}

	#[doc(hidden)]
	fn to_usize(self) -> usize;
}

impl UsizeExt for usize
{
	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self
	}
}

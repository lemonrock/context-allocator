// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
enum Color
{
	Red = 0,

	Black = 1,
}

impl Color
{
	#[inline(always)]
	pub(crate) fn color_bit(self) -> usize
	{
		self as usize
	}

	#[inline(always)]
	pub(crate) fn is_red(self) -> bool
	{
		self == Color::Red
	}

	#[inline(always)]
	pub(crate) fn is_black(self) -> bool
	{
		self == Color::Black
	}
}

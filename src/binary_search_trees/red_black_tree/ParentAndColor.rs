// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ParentAndColor(usize);

impl Default for ParentAndColor
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(NodePointer::default(), Red)
	}
}

impl ParentAndColor
{
	const ColorBitmask: usize = 0b1;

	const ParentBitmask: usize = !Self::ColorBitmask;

	#[inline(always)]
	pub(crate) fn new(parent: NodePointer, color: Color) -> Self
	{
		debug_assert!(align_of::<Node>() >= 2, "Node needs to be aligned to 2 bytes or more otherwise we can not set the color_bit using unused bits in the parent pointer");

		Self((parent.0 as usize & Self::ParentBitmask) | color.color_bit())
	}

	#[inline(always)]
	pub(crate) fn parent(self) -> NodePointer
	{
		NodePointer((self.0 & Self::ParentBitmask) as *const Node)
	}

	#[inline(always)]
	pub(crate) fn color(self) -> Color
	{
		unsafe { transmute(self.0 & Self::ColorBitmask) }
	}
}

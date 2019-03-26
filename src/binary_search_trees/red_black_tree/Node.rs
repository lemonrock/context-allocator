// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[repr(align(16))]
#[derive(Debug)]
pub(crate) struct Node
{
	left: Cell<NodePointer>,
	right: Cell<NodePointer>,
	parent_and_color: Cell<ParentAndColor>,
}

impl Node
{
	#[inline(always)]
	pub(crate) fn reset(&mut self)
	{
		self.left = Cell::default();
		self.right = Cell::default();
		self.parent_and_color = Cell::default();
	}

	#[inline(always)]
	pub(crate) fn parent(&self) -> NodePointer
	{
		self.parent_and_color().parent()
	}

	#[inline(always)]
	pub(crate) fn set_parent(&self, parent: NodePointer)
	{
		self.set_parent_and_color(parent, self.color())
	}

	#[inline(always)]
	pub(crate) fn color(&self) -> Color
	{
		self.parent_and_color().color()
	}

	#[inline(always)]
	pub(crate) fn set_color(&self, color: Color)
	{
		self.set_parent_and_color(self.parent(), color)
	}

	#[inline(always)]
	pub(crate) fn parent_and_color(&self) -> ParentAndColor
	{
		self.parent_and_color.get()
	}

	#[inline(always)]
	pub(crate) fn set_parent_and_color(&self, parent: NodePointer, color: Color)
	{
		self.parent_and_color.set(ParentAndColor::new(parent, color))
	}

	#[inline(always)]
	pub(crate) fn left(&self) -> NodePointer
	{
		self.left.get()
	}

	#[inline(always)]
	pub(crate) fn set_left(&self, left: NodePointer)
	{
		self.left.set(left);
	}

	#[inline(always)]
	pub(crate) fn right(&self) -> NodePointer
	{
		self.right.get()
	}

	#[inline(always)]
	pub(crate) fn set_right(&self, right: NodePointer)
	{
		self.right.set(right);
	}
}

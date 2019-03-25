// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// An iterator over references to the items of a `RedBlackTree`.
///
/// Expensive to construct.
pub struct RedBlackTreeDoubleEndedIterator<'a>
{
	head: NodePointer,
	tail: NodePointer,
	tree: &'a RedBlackTree,
}

impl<'a> Iterator for RedBlackTreeDoubleEndedIterator<'a>
{
	type Item = MemoryAddress;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let head = self.head;

		if unlikely!(head.is_null())
		{
			return None
		}

		self.head = if head == self.tail
		{
			self.tail = NodePointer::default();

			NodePointer::default()
		}
		else
		{
			head.next()
		};

		Some(head.value())
	}
}

impl<'a> DoubleEndedIterator for RedBlackTreeDoubleEndedIterator<'a>
{
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item>
	{
		let tail = self.tail;

		if unlikely!(tail.is_null())
		{
			return None
		}

		self.tail = if tail == self.head
		{
			self.head = NodePointer::default();

			NodePointer::default()
		}
		else
		{
			tail.previous()
		};

		Some(tail.value())
	}
}

impl<'a> Clone for RedBlackTreeDoubleEndedIterator<'a>
{
	#[inline(always)]
	fn clone(&self) -> RedBlackTreeDoubleEndedIterator<'a>
	{
		Self
		{
			head: self.head,
			tail: self.tail,
			tree: self.tree,
		}
	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A cursor which provides mutable access to a `RedBlackTree`.
pub struct Cursor<'a>
{
	current: NodePointer,
	tree: &'a mut RedBlackTree,
}

impl<'a> Cursor<'a>
{
	/// Checks if the cursor is currently pointing to the null object.
	#[inline(always)]
	pub(crate) fn is_null(&self) -> bool
	{
		self.current.is_null()
	}

	/// Returns a reference to the object that the cursor is currently pointing to.
	///
	/// This returns `None` if the cursor is currently pointing to the null object.
	#[inline(always)]
	pub(crate) fn get(&self) -> Option<MemoryAddress>
	{
		self.current.optional_value()
	}

	/// Removes the current element from the `RedBlackTree`.
	///
	/// A pointer to the element that was removed is returned, and the cursor is moved to point to the next element in the `RedBlackTree`.
	///
	/// If the cursor is currently pointing to the null object then no element is removed and `None` is returned.
	#[inline(always)]
	pub(crate) fn remove(&mut self) -> Option<MemoryAddress>
	{
		if self.is_null()
		{
			return None
		}

		let next = self.current_next();
		let result = self.current_value();

		self.current.remove(self.tree_root_mut());
		self.current = next;

		result
	}

	/// Removes the current element from the `RedBlackTree` and inserts another object in its place.
	///
	/// A pointer to the element that was removed is returned, and the cursor is modified to point to the newly added element.
	///
	/// When using this function you must ensure that the elements in the collection are maintained in increasing order.
	/// Failure to do this may lead to `find`, `upper_bound`, `lower_bound` and `range` returning incorrect results.
	///
	/// If the cursor is currently pointing to the null object then an error is returned containing the given `value` parameter.
	#[inline(always)]
	pub(crate) fn replace_with(&mut self, value: MemoryAddress) -> Result<MemoryAddress, MemoryAddress>
	{
		if self.is_null()
		{
			return Err(value)
		}

		let new = self.node_from_value(value);
		let result = self.current.value();

		self.current.replace_with(new, self.tree_root_mut());
		self.current = new;

		Ok(result)
	}

	/// Inserts a new element into the `RedBlackTree` after the current one.
	///
	/// When using this function you must ensure that the elements in the collection are maintained in increasing order.
	/// Failure to do this may lead to `find`, `upper_bound`, `lower_bound` and `range` returning incorrect results.
	///
	/// If the cursor is pointing at the null object then the new element is inserted at the start of the `RedBlackTree`.
	#[inline(always)]
	pub(crate) fn insert_after(&mut self, value: MemoryAddress)
	{
		let new = self.node_from_value(value);

		if unlikely!(self.tree_is_empty())
		{
			self.insert_root(new)
		}
		else
		{
			if unlikely!(self.is_null())
			{
				self.tree_root_first_child().insert_left(new, self.tree_root_mut())
			}
			else
			{
				let current = self.current;

				if unlikely!(current.right().is_null())
				{
					current.insert_right(new, self.tree_root_mut())
				}
				else
				{
					current.next().insert_left(new, self.tree_root_mut())
				}
			}
		}
	}

	/// Inserts a new element into the `RedBlackTree` before the current one.
	///
	/// When using this function you must ensure that the elements in the collection are maintained in increasing order.
	/// Failure to do this may lead to `find`, `upper_bound`, `lower_bound` and `range` returning incorrect results.
	///
	/// If the cursor is pointing at the null object then the new element is inserted at the end of the `RedBlackTree`.
	#[inline(always)]
	pub(crate) fn insert_before(&mut self, value: MemoryAddress)
	{
		let new = self.node_from_value(value);

		if unlikely!(self.tree_is_empty())
		{
			self.insert_root(new)
		}
		else
		{
			if unlikely!(self.is_null())
			{
				self.tree_root_last_child().insert_right(new, self.tree_root_mut())
			}
			else
			{
				let current = self.current;

				if unlikely!(current.left().is_null())
				{
					current.insert_left(new, self.tree_root_mut())
				}
				else
				{
					current.previous().insert_right(new, self.tree_root_mut());
				}
			}
		}
	}

	/// Moves the cursor to the next element of the `RedBlackTree`.
	///
	/// If the cursor is pointer to the null object then this will move it to the first element of the `RedBlackTree` (this is potentially an expensive operation).
	/// If it is pointing to the last element of the `RedBlackTree` then this will move it to the null object.
	#[inline(always)]
	pub(crate) fn move_next(&mut self)
	{
		if unlikely!(self.is_null())
		{
			self.current = self.tree_root_first_child()
		}
		else
		{
			self.current = self.current_next()
		}
	}

	/// Returns a cursor pointing to the previous element of the `RedBlackTree` (by doing a shallow a clone).
	///
	/// If the cursor is pointer to the null object then this will return the last element of the `RedBlackTree` (this is potentially an expensive operation).
	/// If it is pointing to the first element of the `RedBlackTree` then this will return a null cursor.
	#[inline(always)]
	pub(crate) fn move_previous(&mut self)
	{
		if unlikely!(self.is_null())
		{
			self.current = self.tree_root_last_child()
		}
		else
		{
			self.current = self.current_previous()
		}
	}

	#[inline(always)]
	fn current_value(&self) -> Option<MemoryAddress>
	{
		Some(self.current.value())
	}

	#[inline(always)]
	fn current_next(&self) -> NodePointer
	{
		self.current.next()
	}

	#[inline(always)]
	fn current_previous(&self) -> NodePointer
	{
		self.current.previous()
	}

	#[inline(always)]
	fn node_from_value(&self, value: MemoryAddress) -> NodePointer
	{
		self.tree.node_from_value(value)
	}

	#[inline(always)]
	fn tree_is_empty(&self) -> bool
	{
		self.tree.is_empty()
	}

	#[inline(always)]
	fn insert_root(&mut self, new: NodePointer)
	{
		self.tree.insert_root(new)
	}

	#[inline(always)]
	fn tree_root_first_child(&self) -> NodePointer
	{
		self.tree_root().first_child()
	}

	#[inline(always)]
	fn tree_root_last_child(&self) -> NodePointer
	{
		self.tree_root().last_child()
	}

	#[inline(always)]
	fn tree_root(&self) -> NodePointer
	{
		self.tree.root
	}

	#[inline(always)]
	fn tree_root_mut(&mut self) -> &mut NodePointer
	{
		&mut self.tree.root
	}
}

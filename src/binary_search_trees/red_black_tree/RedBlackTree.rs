// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct RedBlackTree
{
	root: NodePointer,
}

impl RedBlackTree
{
	/// Creates an empty `RedBlackTree`.
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		Self
		{
			root: NodePointer::default(),
		}
	}

	/// Returns `true` if the tree is empty.
	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool
	{
		self.root.is_null()
	}

	/// Returns a `Cursor` pointing to the first element of the tree.
	///
	/// If the the tree is empty then a null cursor is returned.
	#[inline(always)]
	pub(crate) fn front<'a>(&'a mut self) -> Cursor<'a>
	{
		let mut cursor = self.cursor();
		cursor.move_next();
		cursor
	}

	/// Returns a `Cursor` pointing to the last element of the tree.
	///
	/// If the tree is empty then a null cursor is returned.
	#[inline]
	pub(crate) fn back<'a>(&'a mut self) -> Cursor<'a>
	{
		let mut cursor = self.cursor();
		cursor.move_previous();
		cursor
	}

	/// Returns a null `Cursor` for this tree.
	#[inline(always)]
	pub(crate) fn cursor<'a>(&'a mut self) -> Cursor<'a>
	{
		Cursor
		{
			current: NodePointer::default(),
			tree: self,
		}
	}

    /// Returns a `Cursor` pointing to an element with the given key.
    ///
	/// If no such element is found then a null cursor is returned.
    ///
    /// If multiple elements with an identical key are found then an arbitrary one is returned.
	///
	/// Creating the cursor is not efficient.
    #[inline(always)]
    pub(crate) fn find<'a>(&'a mut self, key: &MemoryAddress) -> Cursor<'a>
    {
        Cursor
		{
            current: self.find_internal(key),
            tree: self,
        }
    }

    /// Returns a `Cursor` pointing to the first element whose key is above the given bound.
    ///
    /// If no such element is found then a null cursor is returned.
	///
	/// Creating the cursor is not efficient.
    #[inline(always)]
    pub(crate) fn lower_bound<'a>(&'a mut self, bound: Bound<&MemoryAddress>) -> Cursor<'a>
    {
        Cursor
		{
            current: self.lower_bound_internal(bound),
            tree: self,
        }
    }

	/// Returns a `Cursor` pointing to the last element whose key is below the given bound.
	///
	/// If no such element is found then a null cursor is returned.
	///
	/// Creating the cursor is not efficient.
    #[inline(always)]
    pub(crate) fn upper_bound<'a>(&'a mut self, bound: Bound<&MemoryAddress>) -> Cursor<'a>
    {
        Cursor
		{
            current: self.upper_bound_internal(bound),
            tree: self,
        }
    }

	/// Inserts a new element into the `RedBlackTree`.
	///
	/// The new element will be inserted at the correct position in the tree based on its key.
	///
	/// Returns a mutable cursor pointing to the newly added element.
	#[inline(always)]
	pub(crate) fn insert<'a>(&'a mut self, value: MemoryAddress) -> Cursor<'a>
	{
		let new = self.node_from_value(value);
		if unlikely!(self.is_empty())
		{
			self.insert_root(new);
		}
		else
		{
			let key = value;
			let mut tree = self.root;
			loop
			{
				if key < tree.key()
				{
					if unlikely!(tree.left().is_null())
					{
						tree.insert_left(new, &mut self.root);
						break;
					}
					else
					{
						tree = tree.left();
					}
				}
				else
				{
					if unlikely!(tree.right().is_null())
					{
						tree.insert_right(new, &mut self.root);
						break;
					}
					else
					{
						tree = tree.right();
					}
				}
			}
		}

		Cursor
		{
			current: new,
			tree: self,
		}
	}

	/// Gets an iterator over the objects in the `RedBlackTree`, in ascending key order.
	///
	/// Creating the iterator itself is not efficient.
	#[inline(always)]
	pub(crate) fn double_ended_iterate<'a>(&'a self) -> RedBlackTreeDoubleEndedIterator<'a>
	{
		if self.is_empty()
		{
			RedBlackTreeDoubleEndedIterator
			{
				head: NodePointer::default(),
				tail: NodePointer::default(),
				tree: self,
			}
		}
		else
		{
			RedBlackTreeDoubleEndedIterator
			{
				head: self.root.first_child(),
				tail: self.root.last_child(),
				tree: self,
			}
		}
	}

    /// Constructs a double-ended iterator over a sub-range of elements in the tree, starting at `minimum`, and ending at `maximum`.
    ///
    /// If `minimum` is `Unbounded`, then it will be treated as "negative infinity", and if `maximum` is `Unbounded`, then it will be treated as "positive infinity".
    /// Thus `range(Unbounded, Unbounded)` will yield the whole collection, and so is a more expensive choise than using `double_ended_iterate()`.
	///
	/// If `maximum` is less than `minimum` then an empty iterator is returned.
	/// If `maximum` or `minimum` is not found then a then an empty iterator is returned.
	///
	/// Creating the iterator itself is not efficient.
    #[inline(always)]
    pub(crate) fn double_ended_range_iterate<'a>(&'a self, minimum: Bound<&MemoryAddress>, maximum: Bound<&MemoryAddress>) -> RedBlackTreeDoubleEndedIterator<'a>
    {
        let lower = self.lower_bound_internal(minimum);
        let upper = self.upper_bound_internal(maximum);
        if likely!(lower.is_not_null() && upper.is_not_null())
		{
            let lower_key = lower.key();
            let upper_key = upper.key();

            if upper_key >= lower_key
			{
                RedBlackTreeDoubleEndedIterator
				{
                    head: lower,
                    tail: upper,
                    tree: self,
                }
            }
			else
			{
				self.empty_iterator()
			}
        }
		else
		{
			self.empty_iterator()
		}
    }

	#[inline(always)]
	fn empty_iterator<'a>(&'a self) -> RedBlackTreeDoubleEndedIterator<'a>
	{
		RedBlackTreeDoubleEndedIterator
		{
			head: NodePointer::default(),
			tail: NodePointer::default(),
			tree: self,
		}
	}

    #[inline(always)]
    fn find_internal<'a>(&self, key: &MemoryAddress) -> NodePointer
    {
		use self::Ordering::*;

        let mut tree = self.root;
        while tree.is_not_null()
		{
            match key.cmp(&tree.key())
			{
                Less => tree = tree.left(),
                Equal => return tree,
                Greater => tree = tree.right(),
            }
        }

        NodePointer::default()
    }

    #[inline(always)]
    fn lower_bound_internal<'a>(&self, bound: Bound<&MemoryAddress>) -> NodePointer
    {
        let mut tree = self.root;
        let mut result = NodePointer::default();
        while tree.is_not_null()
		{
			let cond = match bound
			{
                Unbounded => true,

                Included(key) => key <= &tree.key(),

                Excluded(key) => key < &tree.key(),
            };

            if cond
			{
                result = tree;
                tree = tree.left();
            }
			else
			{
                tree = tree.right();
            }
        }
        result
    }

    #[inline(always)]
    fn upper_bound_internal<'a>(&self, bound: Bound<&MemoryAddress>) -> NodePointer
    {
        let mut tree = self.root;
        let mut result = NodePointer::default();
        while tree.is_not_null()
		{
            let cond = match bound
			{
                Unbounded => false,

                Included(key) => key < &tree.key(),

                Excluded(key) => key <= &tree.key(),
            };

            if cond
			{
                tree = tree.left();
            }
			else
			{
                result = tree;
                tree = tree.right();
            }
        }
        result
    }

	#[inline(always)]
	fn node_from_value(&self, value: MemoryAddress) -> NodePointer
	{
		NodePointer(value.cast::<Node>().as_ptr() as *const Node)
	}

	#[inline(always)]
	fn insert_root(&mut self, node: NodePointer)
	{
		node.set_parent_and_color(NodePointer::default(), Black);
		node.set_left(NodePointer::default());
		node.set_right(NodePointer::default());
		self.root = node;
	}
}

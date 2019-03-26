// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NodePointer(*const Node);

impl Default for NodePointer
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::null()
	}
}

impl NodePointer
{
	pub(crate) fn furthest_back_contiguous_with(self, block_size: NonZeroUsize) -> MemoryAddress
	{
		let mut after = self;
		let mut before = self.previous();
		loop
		{
			if unlikely!(before.is_contiguous_before(after, block_size))
			{
				after = before;
				before = before.previous();
				continue
			}
			break after.value()
		}
	}

	pub(crate) fn furthest_forward_contiguous_with(self, block_size: NonZeroUsize) -> MemoryAddress
	{
		let mut before = self;
		let mut after = self.next();
		loop
		{
			if unlikely!(before.is_contiguous_before(after, block_size))
			{
				before = after;
				after = after.next();
				continue
			}
			break before.value()
		}
	}

	#[inline(always)]
	fn is_contiguous_before(self, after: Self, block_size: NonZeroUsize) -> bool
	{
		debug_assert!(after.is_not_null(), "after must not be null");

		let before = self;

		before.is_not_null() && before.end_memory_address(block_size) == after
	}

	#[inline(always)]
	fn end_memory_address(self, size: NonZeroUsize) -> MemoryAddress
	{
		self.value().add_non_zero(size)
	}

	#[inline(always)]
	pub(crate) fn optional_value(self) -> Option<MemoryAddress>
	{
		if unlikely!(self.is_null())
		{
			None
		}
		else
		{
			Some(self.value())
		}
	}

	#[inline(always)]
	pub(crate) fn value(self) -> MemoryAddress
	{
		debug_assert!(self.is_not_null(), "null NodePointers do not have a value");

		self.0.non_null().cast::<u8>()
	}

	#[inline(always)]
	pub(crate) fn key(self) -> MemoryAddress
	{
		debug_assert!(self.is_not_null(), "null NodePointers do not have a key");

		self.value()
	}

	#[inline(always)]
	pub(crate) fn is_null(self) -> bool
	{
		self.0.is_null()
	}

	#[inline(always)]
	pub(crate) fn is_not_null(self) -> bool
	{
		self.0.is_not_null()
	}

	pub(crate) fn previous(self) -> Self
	{
		let left = self.left();
		if likely!(left.is_not_null())
		{
			return left.last_child_without_null_check()
		}

		let mut x = self;
		loop
		{
			let parent = x.parent();

			if unlikely!(parent.is_null())
			{
				return Self::null()
			}

			if x.is_not_left_child()
			{
				return parent
			}

			x = parent
		}
	}

	#[inline(always)]
	pub(crate) fn first_child(self) -> Self
	{
		if unlikely!(self.is_null())
		{
			Self::null()
		}
		else
		{
			self.first_child_without_null_check()
		}
	}

	#[inline(always)]
	fn first_child_without_null_check(self) -> Self
	{
		let mut x = self;
		while
		{
			let left = x.left();
			likely!(left.is_not_null())
		}
		{
			x = left
		}
		x
	}

	pub(crate) fn next(self) -> Self
	{
		let right = self.right();
		if likely!(right.is_not_null())
		{
			return right.first_child_without_null_check()
		}

		let mut x = self;
		loop
		{
			let parent = x.parent();

			if unlikely!(parent.is_null())
			{
				return Self::null()
			}

			if x.is_left_child()
			{
				return parent
			}

			x = parent
		}
	}

	#[inline(always)]
	pub(crate) fn last_child(self) -> Self
	{
		if unlikely!(self.is_null())
		{
			Self::null()
		}
		else
		{
			self.last_child_without_null_check()
		}
	}

	#[inline(always)]
	fn last_child_without_null_check(self) -> Self
	{
		let mut x = self;
		while
		{
			let right = x.right();
			likely!(right.is_not_null())
		}
		{
			x = right
		}
		x
	}

	#[inline(always)]
	pub(crate) fn reset(self)
	{
		debug_assert!(self.is_not_null(), "Can not reset() on a null NodePointer");
		self.mutable_node_reference().reset()
	}

	/// This code is based on the red-black tree implementation in libc++.
	pub(crate) fn remove(self, root: &mut Self)
	{
		debug_assert!(self.is_not_null(), "Can not remove null");

		let y = if unlikely!(self.left().is_null() || self.right().is_null())
		{
			self
		}
		else
		{
			self.next()
		};

		let x =
		{
			let y_left = y.left();
			if likely!(y_left.is_not_null())
			{
				y_left
			}
			else
			{
				y.right()
			}
		};

		let w =
		{
			let y_parent = y.parent();

			if likely!(x.is_not_null())
			{
				x.set_parent(y_parent);
			}

			if unlikely!(y_parent.is_null())
			{
				*root = x;
				Self::null()
			}
			else if y.is_left_child()
			{
				y_parent.set_left(x);
				y_parent.right()
			}
			else
			{
				y_parent.set_right(x);
				y_parent.left()
			}
		};

		let removed_black = y.is_black();

		if y != self
		{
			let self_parent = self.parent();
			y.set_parent(self_parent);

			if unlikely!(self_parent.is_null())
			{
				*root = y;
			}
			else
			{
				let y_parent = self_parent;
				if self.is_left_child()
				{
					y_parent.set_left(y);
				}
				else
				{
					y_parent.set_right(y);
				}
			}

			let y_left = self.left();
			y.set_left(y_left);
			y_left.set_parent(y);

			let y_right = self.right();
			y.set_right(y_right);
			if likely!(y_right.is_not_null())
			{
				y_right.set_parent(y);
			}

			y.set_color(self.color());
		}

		if removed_black && root.is_not_null()
		{
			if likely!(x.is_not_null())
			{
				x.set_black();
				return
			}

			let mut w = w;
			loop
			{
				if w.is_not_left_child()
				{
					if w.is_red()
					{
						w.set_black();
						let w_parent = w.parent();
						w_parent.set_red();
						w_parent.rotate_left(root);
						w = w.left().right();
					}

					let w_right_is_null_or_black = w.right().is_null_or_black();

					if w_right_is_null_or_black && w.left().is_null_or_black()
					{
						w.set_red();
						let x = w.parent();

						let x_parent = x.parent();

						if x_parent.is_null_or_red()
						{
							x.set_black();
							return
						}

						w = if x.is_left_child()
						{
							x_parent.right()
						}
						else
						{
							x_parent.left()
						};
						continue
					}

					let w = if w_right_is_null_or_black
					{
						w.left().set_black();
						w.set_red();
						w.rotate_right(root);
						w.parent()
					}
					else
					{
						w
					};

					let w_parent = w.parent();
					w.set_color(w_parent.color());
					w_parent.set_black();
					w.right().set_black();
					w_parent.rotate_left(root);
					return
				}
				else
				{
					if w.is_red()
					{
						w.set_black();
						let w_parent = w.parent();
						w_parent.set_red();
						w_parent.rotate_right(root);
						w = w.right().left();
					}

					let w_left_is_null_or_black = w.left().is_null_or_black();

					if w_left_is_null_or_black && w.right().is_null_or_black()
					{
						w.set_red();
						let x = w.parent();

						let x_parent = x.parent();

						if x_parent.is_null_or_red()
						{
							x.set_black();
							return
						}

						w = if x.is_left_child()
						{
							x_parent.right()
						}
						else
						{
							x_parent.left()
						};
						continue
					}

					let w = if w_left_is_null_or_black
					{
						w.right().set_black();
						w.set_red();
						w.rotate_left(root);
						w.parent()
					}
					else
					{
						w
					};

					let w_parent = w.parent();
					w.set_color(w_parent.color());
					w_parent.set_black();
					w.left().set_black();
					w_parent.rotate_right(root);
					return
				}
			}
		}
	}

	pub(crate) fn replace_with(self, new: Self, root: &mut Self)
	{
		let parent = self.parent();

		if unlikely!(parent.is_null())
		{
			*root = new;
		}
		else if self.is_left_child()
		{
			parent.set_left(new);
		}
		else
		{
			parent.set_right(new);
		}

		let left = self.left();
		if likely!(left.is_not_null())
		{
			left.set_parent(new);
		}

		let right = self.right();
		if likely!(right.is_not_null())
		{
			right.set_parent(new);
		}

		new.set_left(left);
		new.set_right(right);
		new.set_parent_and_color(parent, self.color())
	}

	#[inline(always)]
	pub(crate) fn insert_left(self, new: Self, root: &mut Self)
	{
		new.set_parent_and_color(self, Red);
		new.set_left(Self::null());
		new.set_right(Self::null());
		self.set_left(new);
		new.post_insert(root)
	}

	#[inline(always)]
	pub(crate) fn insert_right(self, new: Self, root: &mut Self)
	{
		new.set_parent_and_color(self, Red);
		new.set_left(Self::null());
		new.set_right(Self::null());
		self.set_right(new);
		new.post_insert(root)
	}

	#[inline(always)]
	pub(crate) fn parent(self) -> Self
	{
		self.node_reference().parent()
	}

	#[inline(always)]
	pub(crate) fn color(self) -> Color
	{
		self.node_reference().color()
	}

	#[inline(always)]
	pub(crate) fn left(self) -> Self
	{
		self.node_reference().left()
	}

	#[inline(always)]
	pub(crate) fn set_left(self, left: Self)
	{
		self.node_reference().left.set(left);
	}

	#[inline(always)]
	pub(crate) fn right(self) -> Self
	{
		self.node_reference().right()
	}

	#[inline(always)]
	pub(crate) fn set_right(self, right: Self)
	{
		self.node_reference().right.set(right);
	}

	/// This code is based on the red-black tree implementation in libc++.
	fn post_insert(self, root: &mut Self)
	{
		macro_rules! deduplicate_continue
		{
			($left_or_right: ident, $x: ident, $x_parent: ident, $x_parent_parent: ident) =>
			{
				{
					let y = x_parent_parent.$left_or_right()

					if y.is_not_null_and_red()
					{
						$x_parent.set_black();
						if $x_parent_parent.parent().is_null()
						{
							$x_parent_parent.set_black();
						}
						else
						{
							$x_parent_parent.set_red();
						}
						y.set_black();

						$x = $x_parent_parent;
						continue
					}
				}
			}
		}

		macro_rules! finish_and_return
		{
			($x: ident, $root: ident, $rotation1: path, $rotation2: path) =>
			{
				{
					let x = if $x.is_not_left_child()
					{
						x_parent.$rotation1($root);
						x_parent
					}
					else
					{
						$x
					};

					let x_parent = x.parent();
					x_parent.set_black();

					let x_parent_parent = x_parent.parent();
					x_parent_parent.set_red();
					x_parent_parent.$rotation2($root);

					return
				}
			}
		}

		let mut x = self;
		let mut x_parent;
		while
		{
			let x_parent = x.parent();
			likely!(x_parent.is_not_null_and_red())
		}
		{
			let x_parent_parent = x_parent.parent();

			if x_parent.is_left_child()
			{
				deduplicate_continue!(right, x, x_parent, x_parent_parent);

				finish_and_return!(x, root, rotate_left, rotate_right)
			}
			else
			{
				deduplicate_continue!(left, x, x_parent, x_parent_parent);

				finish_and_return!(x, root, rotate_right, rotate_left)
			}
		}
	}

	fn rotate_left(self, root: &mut Self)
	{
		let y = self.right();

		let y_left = y.left();
		self.set_right(y_left);

		let self_right = y_left;
		if likely!(self_right.is_not_null())
		{
			self_right.set_parent(self);
		}

		let self_parent = self.parent();
		y.set_parent(self_parent);

		if unlikely!(self_parent.is_null())
		{
			*root = y;
		}
		else if self.is_left_child()
		{
			self_parent.set_left(y);
		}
		else
		{
			self_parent.set_right(y);
		}
		y.set_left(self);

		self.set_parent(y)
	}

	fn rotate_right(self, root: &mut Self)
	{
		let y = self.left();
		let y_right = y.right();

		self.set_left(y_right);
		let self_left = y_right;

		if unlikely!(self_left.is_not_null())
		{
			self_left.set_parent(self);
		}

		let self_parent = self.parent();
		y.set_parent(self_parent);

		if likely!(self_parent.is_null())
		{
			*root = y;
		}
		else if self.is_left_child()
		{
			self_parent.set_left(y);
		}
		else
		{
			self_parent.set_right(y);
		}
		y.set_right(self);

		self.set_parent(y)
	}

	#[inline(always)]
	fn is_null_or_black(self) -> bool
	{
		self.is_null() || self.is_black()
	}

	#[inline(always)]
	fn is_null_or_red(self) -> bool
	{
		self.is_null() || self.is_red()
	}

	#[inline(always)]
	fn is_not_null_and_red(self) -> bool
	{
		self.is_not_null() && self.is_red()
	}

	#[inline(always)]
	fn set_red(self)
	{
		self.set_color(Color::Red)
	}

	#[inline(always)]
	fn set_black(self)
	{
		self.set_color(Color::Black)
	}

	#[inline(always)]
	fn is_red(self) -> bool
	{
		self.color().is_red()
	}

	#[inline(always)]
	fn is_black(self) -> bool
	{
		self.color().is_black()
	}

	#[inline(always)]
	fn is_left_child(self) -> bool
	{
		self.parent().left() == self
	}

	#[inline(always)]
	fn is_not_left_child(self) -> bool
	{
		self.parent().left() != self
	}

	#[inline(always)]
	fn set_parent(self, parent: Self)
	{
		self.node_reference().set_parent(parent)
	}

	#[inline(always)]
	fn set_color(self, color: Color)
	{
		self.node_reference().set_color(color)
	}

	#[inline(always)]
	fn set_parent_and_color(self, parent: Self, color: Color)
	{
		self.node_reference().set_parent_and_color(parent, color)
	}

	#[inline(always)]
	fn node_reference<'a>(self) -> &'a Node
	{
		self.0.reference()
	}

	#[inline(always)]
	fn mutable_node_reference<'a>(self) -> &'a mut Node
	{
		self.0.mutable_reference()
	}

	#[inline(always)]
	const fn null() -> Self
	{
		Self(null())
	}
}
// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// An allocator which uses sorted lists (red-black binary search trees) of different block sizes.
///
/// Each sorted list contains a different block size.
///
/// Block sizes are powers of 2; the smallest is currently 16.
pub struct MultipleBinarySearchTreeAllocator(BinarySearchTreesWithCachedKnowledgeOfFirstChild);

impl Debug for MultipleBinarySearchTreeAllocator
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.0.fmt(f)
	}
}

impl Default for MultipleBinarySearchTreeAllocator
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(BinarySearchTreesWithCachedKnowledgeOfFirstChild::default())
	}
}

impl Allocator for MultipleBinarySearchTreeAllocator
{
	#[inline(always)]
	fn allocate(&mut self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<NonNull<u8>, AllocErr>
	{
		macro_rules! try_to_allocate_exact_size_block
		{
			($node_pointer: ident, $is_cached_first_child: expr, $non_zero_power_of_two_alignment: ident, $binary_search_tree: ident, $_block_size: ident, $_exact_block_size: ident, $_self: ident) =>
			{
				{
					let memory_address = $node_pointer.value();

					if likely!(memory_address.is_aligned_to($non_zero_power_of_two_alignment))
					{
						$binary_search_tree.remove($node_pointer, $is_cached_first_child);

						return Ok(memory_address)
					}
				}
			}
		}
		
		macro_rules! try_to_allocate_larger_sized_block
		{
			($node_pointer: ident, $is_cached_first_child: expr, $floored_non_zero_power_of_two_alignment: ident, $binary_search_tree: ident, $block_size: ident, $exact_block_size: ident, $self: ident) =>
			{
				{
					let start_memory_address = $node_pointer.value();
					let mut memory_address = start_memory_address;
					let end_memory_address =  memory_address.add($block_size);
					while
					{
						if likely!(memory_address.is_aligned_to($floored_non_zero_power_of_two_alignment))
						{
							$binary_search_tree.remove($node_pointer, $is_cached_first_child);

							// Block(s) at front.
							$self.split_up_block(start_memory_address, memory_address);

							// Blocks(s) at end.
							$self.split_up_block(memory_address.add($exact_block_size), end_memory_address);

							return Ok(memory_address)
						}

						memory_address.add_assign_non_zero($floored_non_zero_power_of_two_alignment);
						likely!(memory_address != end_memory_address)
					}
					{
					}
				}
			}
		}

		macro_rules! try_to_satisfy_allocation
		{
			($callback: ident, $binary_search_tree_index: ident, $non_zero_power_of_two_alignment: ident, $block_size: ident, $exact_block_size: ident, $self: ident) =>
			{
				{
					let binary_search_tree = self.binary_search_tree_for($binary_search_tree_index);
					let original_first_child = binary_search_tree.cached_first_child();
					if likely!(original_first_child.is_not_null())
					{
						$callback!(original_first_child, true, $non_zero_power_of_two_alignment, binary_search_tree, $block_size, $exact_block_size, $self);

						let mut node_pointer = original_first_child.next();
						while likely!(node_pointer.is_not_null())
						{
							$callback!(node_pointer, false, $non_zero_power_of_two_alignment, binary_search_tree, $block_size, $exact_block_size, $self);
							node_pointer = node_pointer.next();
						}
					}
				}
			}
		}

		if unlikely!(BinarySearchTreesWithCachedKnowledgeOfFirstChild::size_exceeds_maximum_allocation_size(non_zero_size))
		{
			return Err(AllocErr)
		}

		if unlikely!(BinarySearchTreesWithCachedKnowledgeOfFirstChild::alignment_exceeds_maximum_alignment(non_zero_power_of_two_alignment))
		{
			return Err(AllocErr)
		}

		// (1) Try to satisfy allocation from a binary search tree of blocks of the same size.
		let binary_search_tree_index_for_blocks_of_exact_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index(Self::block_size(non_zero_size));
		#[allow(dead_code)] const Unused: () = ();
		try_to_satisfy_allocation!(try_to_allocate_exact_size_block, binary_search_tree_index_for_blocks_of_exact_size, non_zero_power_of_two_alignment, Unused, Unused, Unused);

		// (2) Try to satisfy allocation from binary search trees of blocks of larger size (either because of exhaustion or a large alignment).
		let floored_non_zero_power_of_two_alignment = BinarySearchTreesWithCachedKnowledgeOfFirstChild::floor_alignment_to_minimum(non_zero_power_of_two_alignment);
		let exact_block_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index_to_block_size(binary_search_tree_index_for_blocks_of_exact_size);
		for binary_search_tree_index_of_larger_size_block in (binary_search_tree_index_for_blocks_of_exact_size + 1) .. BinarySearchTreesWithCachedKnowledgeOfFirstChild::NumberOfBinarySearchTrees
		{
			let block_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index_to_block_size(binary_search_tree_index_of_larger_size_block);

			try_to_satisfy_allocation!(try_to_allocate_larger_sized_block, binary_search_tree_index_of_larger_size_block, floored_non_zero_power_of_two_alignment, block_size, exact_block_size, self);
		}

		Err(AllocErr)
	}

	#[inline(always)]
	fn deallocate(&mut self, non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		let block_size = Self::block_size(non_zero_size);

		let binary_search_tree_index = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index(block_size);
		#[cfg(debug_assertions)]
		{
			if block_size.get() > 32
			{
				debug_assert_ne!(binary_search_tree_index, 0, "WTF?")
			}
		}

		// TODO: Optimization - can we use lower bound / upper bound rather than doing an insert in order to find blocks to coalesce?
		let binary_search_tree = self.binary_search_tree_for(binary_search_tree_index);
		let has_blocks = binary_search_tree.has_blocks();
		let inserted_node_pointer = binary_search_tree.insert_memory_address(current_memory);
		if likely!(has_blocks)
		{
			self.coalesce(inserted_node_pointer, block_size, binary_search_tree_index);
		}
	}

	#[inline(always)]
	fn shrinking_reallocate(&mut self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		debug_assert_ne!(non_zero_current_size.difference(non_zero_new_size), 0, "Should never be called with no difference");

		let old_block_size = Self::block_size(non_zero_current_size);
		let new_block_size = Self::block_size(non_zero_new_size);

		self.split_up_block(current_memory.add_non_zero(new_block_size), current_memory.add_non_zero(old_block_size));

		Ok(current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&mut self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		let difference = non_zero_new_size.difference(non_zero_current_size);
		debug_assert_ne!(difference, 0, "Should never be called with a zero difference");

		let old_block_size = Self::block_size(non_zero_current_size);
		let new_block_size = Self::block_size(non_zero_new_size);

		// (1) Satisfy from within existing block.
		if new_block_size == old_block_size
		{
			return Ok(current_memory)
		}

		// (2) For a simple doubling, it can be more efficient to try to coalesce two blocks.
		//
		// This technique could work for other approaches, eg quadrupling, but it becomes a lot more complex - and the gain over an efficient memory copy is probably lost.
		if new_block_size == old_block_size.doubled()
		{
			let binary_search_tree = self.binary_search_tree_for_block_size(old_block_size);
			let contiguous_block_node_pointer = binary_search_tree.find(current_memory.add_non_zero(old_block_size));
			if contiguous_block_node_pointer.is_not_null()
			{
				let is_first_child = contiguous_block_node_pointer == binary_search_tree.cached_first_child();
				binary_search_tree.remove(contiguous_block_node_pointer, is_first_child);

				return Ok(current_memory)
			}
		}

		// (3) Allocate a new block and copy over data.
		let block_to_copy_into = self.allocate(non_zero_new_size, non_zero_power_of_two_alignment)?;
		unsafe { current_memory.as_ptr().copy_to_nonoverlapping(block_to_copy_into.as_ptr(), non_zero_current_size.get()) };
		self.deallocate(non_zero_current_size, non_zero_power_of_two_alignment, current_memory);
		Ok(block_to_copy_into)
	}
}

impl MultipleBinarySearchTreeAllocator
{
	/// Calling this more than once is legal (but odd; blocks will not be coalesced), but calling it more than once with overlapping slices is not.
	///
	/// If the provided memory's length is not a multiple of 2, then the remainder is unused.
	///
	/// The provided memory must be at least as long as the minimum block size.
	///
	/// The memory must be aligned to `BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAlignment`, which is the same as the size of a `Node`.
	#[inline(always)]
	pub fn make_use_of(&mut self, memory: &mut [u8])
	{
		let mut size = memory.len();
		debug_assert_ne!(size, 0, "size can not be zero");

		let mut memory_address = memory.as_mut_ptr().non_null();
		debug_assert!(memory_address.is_aligned_to(BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAlignment), "memory is not aligned to `{:?}`", BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAlignment);

		debug_assert_ne!(BinarySearchTreesWithCachedKnowledgeOfFirstChild::NumberOfBinarySearchTrees, 0, "There must be at least one binary search stree");
		let mut last_binary_search_tree_index = BinarySearchTreesWithCachedKnowledgeOfFirstChild::NumberOfBinarySearchTrees;
		while likely!(last_binary_search_tree_index > 0)
		{
			let binary_search_tree_index = last_binary_search_tree_index - 1;

			let block_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index_to_block_size(binary_search_tree_index);

			if unlikely!(size < block_size)
			{
				if unlikely!(BinarySearchTreesWithCachedKnowledgeOfFirstChild::size_is_less_than_minimum_allocation_size(size))
				{
					break
				}

				last_binary_search_tree_index = binary_search_tree_index;
				continue
			}

			let binary_search_tree = self.binary_search_tree_for(binary_search_tree_index);
			while
			{
				binary_search_tree.insert_memory_address(memory_address);

				memory_address.add_assign(block_size);
				size -= block_size;

				likely!(size >= block_size)
			}
			{
			}
			last_binary_search_tree_index = binary_search_tree_index;
		}
	}

	#[inline(always)]
	fn split_up_block(&mut self, mut from: MemoryAddress, to: MemoryAddress)
	{
		let mut difference = to.difference(from);
		while likely!(difference != 0)
		{
			let smallest_power_of_two_difference = BinarySearchTreesWithCachedKnowledgeOfFirstChild::smallest_power_of_two_difference(difference);

			self.deallocate(smallest_power_of_two_difference, smallest_power_of_two_difference, from);

			from.add_assign_non_zero(smallest_power_of_two_difference);
			difference -= smallest_power_of_two_difference.get();
		}
	}

	fn coalesce(&mut self, inserted_node_pointer: NodePointer, block_size: NonZeroUsize, binary_search_tree_index: usize)
	{
		let furthest_back_contiguous_with_inserted_node_pointer_memory_address = inserted_node_pointer.furthest_back_contiguous_with(block_size);

		let furthest_forward_contiguous_with_inserted_node_pointer_memory_address = inserted_node_pointer.furthest_forward_contiguous_with(block_size);

		let difference = furthest_forward_contiguous_with_inserted_node_pointer_memory_address.difference(furthest_back_contiguous_with_inserted_node_pointer_memory_address);

		let nothing_to_coalesce = difference == 0;

		if likely!(nothing_to_coalesce)
		{
			return
		}

		let first_block_memory_address =
		{
			let binary_search_tree = self.binary_search_tree_for(binary_search_tree_index);

			let (first_block_memory_address, last_block_memory_address) = binary_search_tree.blocks_to_coalesce(inserted_node_pointer, difference.non_zero(), block_size, furthest_back_contiguous_with_inserted_node_pointer_memory_address, furthest_forward_contiguous_with_inserted_node_pointer_memory_address);

			binary_search_tree.remove_contiguous_blocks(first_block_memory_address, last_block_memory_address, block_size);

			first_block_memory_address
		};

		// TODO: Do we actually need a loop and all the stuff above? Would we ever have more than 3 potentially coalescing blocks at once?
		let mut difference = difference;
		let mut from = first_block_memory_address;
		while
		{
			let smallest_power_of_two_difference = BinarySearchTreesWithCachedKnowledgeOfFirstChild::smallest_power_of_two_difference(difference);
			debug_assert_ne!(smallest_power_of_two_difference, block_size, "difference should never be block_size");

			self.deallocate(smallest_power_of_two_difference, smallest_power_of_two_difference, from);

			from.add_assign_non_zero(smallest_power_of_two_difference);
			difference -= smallest_power_of_two_difference.get();
			likely!(difference != 0)
		}
		{
		}
	}

	#[inline(always)]
	fn binary_search_tree_for_block_size(&mut self, block_size: NonZeroUsize) -> &mut BinarySearchTreeWithCachedKnowledgeOfFirstChild
	{
		self.binary_search_tree_for(BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index(block_size))
	}

	#[inline(always)]
	fn block_size(non_zero_size: NonZeroUsize) -> NonZeroUsize
	{
		BinarySearchTreesWithCachedKnowledgeOfFirstChild::floor_size_to_minimum(non_zero_size).next_power_of_two()
	}

	#[inline(always)]
	fn binary_search_tree_for(&mut self, binary_search_tree_index: usize) -> &mut BinarySearchTreeWithCachedKnowledgeOfFirstChild
	{
		self.0.binary_search_tree_for(binary_search_tree_index)
	}
}

#[cfg(test)]
mod MultipleBinarySearchTreeAllocatorTests
{
	use super::*;
	use ::std::alloc::Global;
	use ::std::slice::from_raw_parts_mut;

	#[test]
	pub fn repeated_small_allocations()
	{
		test_repeated_small_allocations(32);
		test_repeated_small_allocations(64);
		test_repeated_small_allocations(96);
		test_repeated_small_allocations(128);
		test_repeated_small_allocations(160);
		test_repeated_small_allocations(192);
		test_repeated_small_allocations(256);

		// TODO: Optimization - split_blocks calls deallocate with a block size; deallocate then computes the same block_size.

		// TODO: Optimization - can we use lower bound / upper bound rather than doing an insert in order to find blocks to coalesce?

		// TODO: Do we actually need a loop and all the stuff above? Would we ever have more than 3 potentially coalescing blocks at once?
	}

	#[test]
	pub fn mixed_allocations()
	{
		let (mut allocator, memory) = new_allocator(256);
		
		allocator.allocate(32.non_zero(), 8.non_zero()).expect(&format!("Did not allocate"));
		allocator.allocate(128.non_zero(), 8.non_zero()).expect(&format!("Did not allocate"));
		allocator.allocate(64.non_zero(), 8.non_zero()).expect(&format!("Did not allocate"));
		allocator.allocate(32.non_zero(), 8.non_zero()).expect(&format!("Did not allocate"));
		assert_allocator_is_empty(&mut allocator);

		destroy_memory(memory)
	}

	fn test_repeated_small_allocations(memory_size: usize)
	{
		let (mut allocator, memory) = new_allocator(memory_size);

		for allocation_loop_count in 0 .. memory_size / SmallestAllocation
		{
			let _ = allocator.allocate(1.non_zero(), 1.non_zero()).expect(&format!("Did not allocate for loop `{}`", allocation_loop_count));
		}
		assert_allocator_is_empty(&mut allocator);

		destroy_memory(memory)
	}

	fn assert_allocator_is_empty(allocator: &mut MultipleBinarySearchTreeAllocator)
	{
		assert_eq!(allocator.allocate(1.non_zero(), 1.non_zero()), Err(AllocErr), "Allocator was not empty");
	}

	fn new_allocator<'a>(memory_size: usize) -> (MultipleBinarySearchTreeAllocator, (Layout, &'a mut [u8]))
	{
		let mut allocator = MultipleBinarySearchTreeAllocator::default();

		let (some_memory, layout) = unsafe
		{
			let layout = layout(memory_size);
			let memory_address = Global.alloc(layout).unwrap();
			let some_memory = from_raw_parts_mut(memory_address.as_ptr(), memory_size);
			(some_memory, layout)
		};

		allocator.make_use_of(some_memory);

		(allocator, (layout, some_memory))
	}

	fn destroy_memory((layout, some_memory): (Layout, &mut [u8]))
	{
		unsafe { Global.dealloc(some_memory.as_ptr().non_null(), layout) };
	}

	unsafe fn layout(memory_size: usize) -> Layout
	{
		Layout::from_size_align_unchecked(memory_size, SmallestAllocation)
	}

	const SmallestAllocation: usize = BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAllocationSize.get();
}

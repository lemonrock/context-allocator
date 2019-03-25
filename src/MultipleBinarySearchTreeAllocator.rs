// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/*

			A new idea: Multiple binary search trees, sorted, one for each block size

				Block Size   Block Size   Block Size   Block Size
				    8            16           32           64
*/

struct BinarySearchTreeWithCachedKnowledgeOfFirstChild
{
	tree: RedBlackTree,
	first_child: NodePointer,
}

struct BinarySearchTreesWithCachedKnowledgeOfFirstChild
{
	binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two: [BinarySearchTreeWithCachedKnowledgeOfFirstChild; Self::NumberOfSinglyLinkedListsOfFreeBlocks],
}

impl BinarySearchTreesWithCachedKnowledgeOfFirstChild
{
	const SmallestInclusivePowerOfTwoExponent: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(FreeBlock::MinimumBlockSize.trailing_zeros()) };

	const NumberOfSinglyLinkedListsOfFreeBlocks: usize = 16;

	const LargestInclusiveBinarySearchTreeIndex: usize = Self::NumberOfSinglyLinkedListsOfFreeBlocks - 1;

	const LargestInclusivePowerOfTwoExponent: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(Self::binary_search_tree_index_to_power_of_two_exponent(Self::LargestInclusiveBinarySearchTreeIndex)) };

	const MinimumAllocationSize: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1 << Self::SmallestInclusivePowerOfTwoExponent.get()) };

	const MaximumAllocationSize: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1 << Self::LargestInclusivePowerOfTwoExponent.get()) };

	const MaximumAlignment: NonZeroUsize = Self::MaximumAllocationSize;

	const MaximumBinarySearchTreeIndex: usize = Self::binary_search_tree_index(Self::LargestInclusivePowerOfTwoExponent);

	#[inline(always)]
	const fn binary_search_tree_index(not_necessarily_power_of_two: NonZeroU32) -> usize
	{
		let power_of_two = not_necessarily_power_of_two.next_power_of_two();

		logarithm_base2_as_usize(non_zero_u32_right_shift_as_u32(power_of_two, Self::SmallestInclusivePowerOfTwoExponent))
	}

	#[inline(always)]
	const fn binary_search_tree_index_to_power_of_two_exponent(binary_search_tree_index: usize) -> usize
	{
		Self::SmallestInclusivePowerOfTwoExponent.get() + binary_search_tree_index
	}

	#[inline(always)]
	const fn binary_search_tree_index_to_block_size(binary_search_tree_index: usize) -> usize
	{
		1 << Self::binary_search_tree_index_to_power_of_two_exponent(binary_search_tree_index)
	}

	#[inline(always)]
	const fn size_exceeds_maximum_allocation_size(non_zero_size: NonZeroUsize) -> bool
	{
		non_zero_size > Self::MaximumAllocationSize
	}

	#[inline(always)]
	const fn alignment_exceeds_maximum_alignment(non_zero_power_of_two_alignment: NonZeroUsize) -> bool
	{
		non_zero_power_of_two_alignment > Self::MaximumAlignment
	}

	#[inline(always)]
	const fn floor_size_to_minimum(unfloored_non_zero_size: NonZeroUsize) -> NonZeroUsize
	{
		max(unfloored_non_zero_power_of_two_alignment, Self::MinimumAllocationSize)
	}

	#[inline(always)]
	const fn floor_alignment_to_minimum(unfloored_non_zero_power_of_two_alignment: NonZeroUsize) -> NonZeroUsize
	{
		max(unfloored_non_zero_power_of_two_alignment, Self::MinimumAlignment)
	}

	#[inline(always)]
	fn binary_search_tree_for(&mut self, binary_search_tree_index: usize) -> &mut BinarySearchTreeWithCachedKnowledgeOfFirstChild
	{
		unsafe { self.binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two.get_unchecked_mut(binary_search_tree_index) }
	}
}

pub struct MultipleBinarySearchTreeAllocator(BinarySearchTreesWithCachedKnowledgeOfFirstChild);

impl Allocator for MultipleBinarySearchTreeAllocator
{
	#[inline(always)]
	fn allocate(&mut self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<NonNull<u8>, AllocErr>
	{
		macro_rules! remove_allocated_block
		{
			($node_pointer: ident, $is_original_first_child: ident, $binary_search_tree: ident) =>
			{
				{
					if $is_original_first_child
					{
						$binary_search_tree.first_child = $node_pointer.next();
						debug_assert!($binary_search_tree.first_child, $binary_search_tree.root.first_child(), "Assumption invalid");

					}
					$node_pointer.remove(&mut $binary_search_tree.root);
					debug_assert!($binary_search_tree.first_child, $binary_search_tree.root.first_child(), "Assumption invalid");
				}
			}
		}

		macro_rules! try_to_allocate_exact_size_block
		{
			($node_pointer: ident, $is_original_first_child: expr, $non_zero_power_of_two_alignment: ident, $binary_search_tree: ident, $_block_size: ident, $_exact_block_size: ident, $_self: ident) =>
			{
				{
					let memory_address = $node_pointer.value();

					if likely!($memory_address.is_aligned_to($non_zero_power_of_two_alignment))
					{
						remove_allocated_block!($node_pointer, $is_original_first_child, $binary_search_tree);

						return Ok(memory_address)
					}
				}
			}
		}
		
		macro_rules! try_to_allocate_larger_sized_block
		{
			($node_pointer: ident, $is_original_first_child: expr, $floored_non_zero_power_of_two_alignment: ident, $binary_search_tree: ident, $block_size: ident, $exact_block_size: ident, $self: ident) =>
			{
				{
					let start_memory_address = $node_pointer.value();
					let mut memory_address = start_memory_address;
					let end_memory_address =  memory_address.add($block_size);
					while
					{
						if likely!(memory_address.is_aligned_to($floored_non_zero_power_of_two_alignment))
						{
							remove_allocated_block!($node_pointer, $is_original_first_child, $binary_search_tree);

							// Block(s) at front.
							$self.split_up_block(start_memory_address, memory_address)

							// Blocks(s) at end.
							$self.split_up_block(memory_address.add($exact_block_size), end_memory_address)

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
					let original_first_child = binary_search_tree.first_child;
					if likely!(original_first_child.is_not_null())
					{
						callback!(original_first_child, true, non_zero_power_of_two_alignment, binary_search_tree, $block_size, $exact_block_size, $self);

						let mut node_pointer = original_first_child;
						while likely!(node_pointer.is_not_null())
						{
							callback!(node_pointer, false, non_zero_power_of_two_alignment, binary_search_tree, $block_size, $exact_block_size, $self);
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
		let binary_search_tree_index_for_blocks_of_exact_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index(non_zero_size);
		const Unused: () = ();
		try_to_satisfy_allocation!(try_to_allocate_exact_size_block, binary_search_tree_index_for_blocks_of_exact_size, non_zero_power_of_two_alignment, Unused, Unused, Unused);

		// (2) Try to satisfy allocation from binary search trees of blocks of larger size (either because of exhaustion or a large alignment).
		let floored_non_zero_power_of_two_alignment = BinarySearchTreesWithCachedKnowledgeOfFirstChild::floor_alignment_to_minimum(non_zero_power_of_two_alignment);
		let exact_block_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index_to_block_size(binary_search_tree_index_for_blocks_of_exact_size);
		for binary_search_tree_index_of_larger_size_block in (binary_search_tree_index_for_blocks_of_exact_size + 1) .. BinarySearchTreesWithCachedKnowledgeOfFirstChild::MaximumBinarySearchTreeIndex
		{
			let block_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::binary_search_tree_index_to_block_size(binary_search_tree_index);

			try_to_satisfy_allocation!(try_to_allocate_larger_sized_block, binary_search_tree_index_of_larger_size_block, floored_non_zero_power_of_two_alignment, block_size, exact_block_size, self);
		}

		Err(AllocErr)
	}

	#[inline(always)]
	fn deallocate(&mut self, _non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, _current_memory: NonNull<u8>)
	{
		unimplemented!()
	}

	#[inline(always)]
	fn shrinking_reallocate(&mut self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		debug_assert_ne!(non_zero_current_size.difference(non_zero_new_size), 0, "Should never be called with no difference");

		let old_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::floor_size_to_minimum(non_zero_current_size).round_up_to_power_of_two();
		let new_size = BinarySearchTreesWithCachedKnowledgeOfFirstChild::floor_size_to_minimum(non_zero_new_size).round_up_to_power_of_two();

		let difference = new_size.difference(old_size);
		if unlikely!(difference == 0)
		{
			return Ok(current_memory)
		}

		self.split_up_block(current_memory.add_non_zero(new_size), current_memory.add_non_zero(old_size));

		Ok(current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&mut self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, _current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		let difference = non_zero_new_size.difference(non_zero_current_size);
		debug_assert_ne!(difference, 0, "Should never be called with no difference");

		unimplemented!()
	}
}

impl MultipleBinarySearchTreeAllocator
{
	#[inline(always)]
	fn split_up_block(&mut self, mut from: MemoryAddress, to: MemoryAddress)
	{
		let mut difference = to.difference(from);
		while difference != 0
		{
			debug_assert!(difference >= BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAllocationSize.get(), "difference `{}` is too small to be a block");
			
			let smallest_power_of_two_difference = (1 << difference.trailing_zeros());

			self.deallocate(smallest_power_of_two_difference.non_zero(), smallest_power_of_two_difference.non_zero(), from);

			from.add_assign(smallest_power_of_two_difference);
			difference -= smallest_power_of_two_difference;
		}
	}

	#[inline(always)]
	fn binary_search_tree_for(&mut self, binary_search_tree_index: usize) -> &mut BinarySearchTreeWithCachedKnowledgeOfFirstChild
	{
		self.0.binary_search_tree_for(binary_search_tree_index)
	}
}

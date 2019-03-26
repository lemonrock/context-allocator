// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct BinarySearchTreesWithCachedKnowledgeOfFirstChild
{
	binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two: [BinarySearchTreeWithCachedKnowledgeOfFirstChild; Self::NumberOfBinarySearchTrees],
}

impl BinarySearchTreesWithCachedKnowledgeOfFirstChild
{
	const SmallestInclusivePowerOfTwoExponent: NonZeroUsize = Self::logarithm_base2(size_of::<Node>());

	pub(crate) const NumberOfBinarySearchTrees: usize = 16;

	const LargestInclusiveBinarySearchTreeIndex: usize = Self::NumberOfBinarySearchTrees - 1;

	const LargestInclusivePowerOfTwoExponent: NonZeroUsize = non_zero_usize(Self::binary_search_tree_index_to_power_of_two_exponent(Self::LargestInclusiveBinarySearchTreeIndex));

	const MinimumAllocationSize: NonZeroUsize = non_zero_usize(1 << Self::SmallestInclusivePowerOfTwoExponent.get());

	const MaximumAllocationSize: NonZeroUsize = non_zero_usize(1 << Self::LargestInclusivePowerOfTwoExponent.get());

	const MinimumAlignment: NonZeroUsize = Self::MinimumAllocationSize;

	const MaximumAlignment: NonZeroUsize = Self::MaximumAllocationSize;

	#[inline(always)]
	const fn logarithm_base2(value: usize) -> NonZeroUsize
	{
		non_zero_usize(logarithm_base2_as_usize(value))
	}

	#[inline(always)]
	pub(crate) fn binary_search_tree_index(not_necessarily_power_of_two: NonZeroUsize) -> usize
	{
		let power_of_two = not_necessarily_power_of_two.next_power_of_two();

		let shifted = power_of_two.get() >> Self::SmallestInclusivePowerOfTwoExponent.get();

		logarithm_base2_as_usize(shifted)
	}

	#[inline(always)]
	const fn binary_search_tree_index_to_power_of_two_exponent(binary_search_tree_index: usize) -> usize
	{
		Self::SmallestInclusivePowerOfTwoExponent.get() + binary_search_tree_index
	}

	#[inline(always)]
	pub(crate) fn binary_search_tree_index_to_block_size(binary_search_tree_index: usize) -> usize
	{
		1 << Self::binary_search_tree_index_to_power_of_two_exponent(binary_search_tree_index)
	}

	#[inline(always)]
	pub(crate) fn size_exceeds_maximum_allocation_size(non_zero_size: NonZeroUsize) -> bool
	{
		non_zero_size > Self::MaximumAllocationSize
	}

	#[inline(always)]
	pub(crate) fn alignment_exceeds_maximum_alignment(non_zero_power_of_two_alignment: NonZeroUsize) -> bool
	{
		non_zero_power_of_two_alignment > Self::MaximumAlignment
	}

	#[inline(always)]
	pub(crate) fn floor_size_to_minimum(unfloored_non_zero_size: NonZeroUsize) -> NonZeroUsize
	{
		max(unfloored_non_zero_size, Self::MinimumAllocationSize)
	}

	#[inline(always)]
	pub(crate) fn floor_alignment_to_minimum(unfloored_non_zero_power_of_two_alignment: NonZeroUsize) -> NonZeroUsize
	{
		max(unfloored_non_zero_power_of_two_alignment, Self::MinimumAlignment)
	}

	#[inline(always)]
	pub(crate) fn binary_search_tree_for(&mut self, binary_search_tree_index: usize) -> &mut BinarySearchTreeWithCachedKnowledgeOfFirstChild
	{
		debug_assert!(binary_search_tree_index < Self::NumberOfBinarySearchTrees, "binary_search_tree_index is too large");

		unsafe { self.binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two.get_unchecked_mut(binary_search_tree_index) }
	}

	#[inline(always)]
	pub(crate) fn smallest_power_of_two_difference(difference: usize) -> NonZeroUsize
	{
		debug_assert!(difference >= BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAllocationSize.get(), "difference `{}` is too small to be a block");

		(1 << difference.trailing_zeros()).non_zero()
	}

	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn largest_power_of_two_difference(difference: usize) -> NonZeroUsize
	{
		debug_assert!(difference >= BinarySearchTreesWithCachedKnowledgeOfFirstChild::MinimumAllocationSize.get(), "difference `{}` is too small to be a block");

		const BitsInAByte: usize = 8;
		const BitsInAnUsize: usize = size_of::<usize>() * BitsInAByte;
		const ZeroBased: usize = BitsInAnUsize - 1;

		let shift = ZeroBased - difference.leading_zeros() as usize;

		(1 << shift).non_zero()

	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Minimum allocation size is 8 bytes.
#[derive(Debug)]
pub struct LinkedListAllocator
{
	// This is not a valid pointer if `first_free_block_non_null == end_of_all_blocks_non_null`.
	first_free_block_non_null: NonNull<FreeBlock>,
	end_of_all_blocks_non_null: NonNull<FreeBlock>,
}

impl LinkedListAllocator
{
	/// Creates a new wrapper around the provided memory.
	///
	/// Uses 8-byte blocks; a minimum allocation is 8 bytes.
	///
	/// Panics with an assertion failure if `non_zero_memory_size` exceeds 4Gb (`:std::u32::MAX`).
	#[inline(always)]
	pub fn new(memory_starts_at: MemoryAddress, non_zero_memory_size: NonZeroUsize) -> Self
	{
		const MaximumAddressableMemory: NonZeroUsize = non_zero_usize(::std::u32::MAX as usize);
		assert!(non_zero_memory_size <= MaximumAddressableMemory, "non_zero_memory_size `{}` exceeds MaximumAddressableMemory (`{}`)", non_zero_memory_size, MaximumAddressableMemory);

		let first_free_block_non_null = memory_starts_at.cast::<FreeBlock>();

		let end_of_all_blocks_non_null = unsafe
		{
			let first_free_block = first_free_block_non_null.mutable_reference();
			let non_zero_memory_size_u32 = non_zero_memory_size.to_non_zero_u32();
			write(&mut first_free_block.size_of_this_block, non_zero_memory_size_u32);
			write(&mut first_free_block.offset_from_start_of_this_block_to_next_block, non_zero_memory_size_u32);
			first_free_block.next_free_block_non_null()
		};

		Self
		{
			first_free_block_non_null,
			end_of_all_blocks_non_null,
		}
	}
}

impl Allocator for LinkedListAllocator
{
	#[inline(always)]
	fn allocate(&mut self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<NonNull<u8>, AllocErr>
	{
		let end_of_all_blocks_non_null = self.end_of_all_blocks_non_null;

		if unlikely!(non_zero_size > FreeBlock::MaximumAllocationSize)
		{
			return Err(AllocErr)
		}

		if unlikely!(non_zero_power_of_two_alignment > FreeBlock::MaximumAlignment)
		{
			return Err(AllocErr)
		}

		let floored_non_zero_size = FreeBlock::floor_size_to_minimum(non_zero_size);

		let floored_non_zero_power_of_two_alignment = FreeBlock::floor_alignment_to_minimum(non_zero_power_of_two_alignment);

		self.allocate_loop(floored_non_zero_size, floored_non_zero_power_of_two_alignment, end_of_all_blocks_non_null)
	}

	#[inline(always)]
	fn deallocate(&mut self, _non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, _current_memory: NonNull<u8>)
	{
		// Given current memory, we would have to walk (forward) through the singly linked list until we were just after it; then insert it; then coalesce it (rightward then leftware) as appropriate

		// We can make the walk less expensive by using a doubly-linked list and walking from the end if it appears the memory is closer to that end.

		// At least a binary search would be useful.

		// Some sort of binary tree or B-heap might be useful


		/*
                         Middle
                           /\
                          /  \
                         /    \
                        /      \
                      Less   Greater
                       /\
                      /  \
                     /    \
                    L      G


			Given a newly-free node, N.

				Compare N's address with Middle address.
					If Middle address == N, panic.
					If Middle address null, set middle to N. Stop.
						? coalesce
					If N less than Middle, look up Less
					If N greater than Middle, look up Greater (behave as for Less).

				Compare N's address with Less
					If Less address == N, panic.
					If Less address null, set Less to N. Stop.
						? coalesce
					If N less than less, look up L (behave as for Less).
					If N greater than less, look up G (behave as for Less).


			Allocating a free block

				- if free block wholly consumed, what happens to its children?
					- if no children, that's ok
					- if one child, assign child to replace parent in grandparent
					- if two children?

				- if front of free block remains - no changes required

				- if back of free block remains - adjust its parent (the grandparent above) to point to back

				- front and back of free block - uggh
					- assign lefthand child to front;
					- assign righthand child to back;
					- but we now have two children (front and back) where there once was one.

				So two difficult scenarios:-
					- parent block disappears, leaving two orphaned child nodes
					- parent block splits into two, leaving two parents

					- in both scenarios, a grandparent has two descendents where there was once one.
						- grandparent had only one child (ie now would have no children), so, we assign both descendents to the grandparent
						- we make one of the two orphans the child of the other by following the scenario for adding a newly-freed (deallocated) node, N.

			So quite a bit more complicated than a singly-linked list, and we still haven't solved how to coalesce blocks.

			The only point is to be able to find successors or predecessors that are contiguous.













		*/




		unimplemented!()
	}

	#[inline(always)]
	fn shrinking_reallocate(&mut self, _non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, _non_zero_current_size: NonZeroUsize, _current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		unimplemented!()
	}

	#[inline(always)]
	fn growing_reallocate(&mut self, _non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, _non_zero_current_size: NonZeroUsize, _current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		unimplemented!()
	}
}

impl LinkedListAllocator
{
	#[inline(always)]
	fn allocate_loop(&mut self, floored_non_zero_size: NonZeroUsize, floored_non_zero_power_of_two_alignment: NonZeroUsize, end_of_all_blocks_non_null: NonNull<FreeBlock>) -> Result<MemoryAddress, AllocErr>
	{
		#[inline(always)]
		fn allocation_will_never_fit_as_there_are_not_enough_blocks_remaining(floored_allocation_must_end_at: MemoryAddress, end_of_all_blocks_non_null: NonNull<FreeBlock>) -> bool
		{
			let end_of_all_blocks_ends_at = end_of_all_blocks_non_null.cast::<u8>();
			floored_allocation_must_end_at > end_of_all_blocks_ends_at
		}

		#[inline(always)]
		fn allocation_fits(floored_allocation_must_end_at: MemoryAddress, free_block: &FreeBlock) -> bool
		{
			floored_allocation_must_end_at <= free_block.ends_at()
		}

		let mut previous_free_block_raw: *mut FreeBlock = null_mut();
		let mut free_block_non_null = self.first_free_block_non_null;
		while likely!(FreeBlock::more_free_blocks(free_block_non_null, end_of_all_blocks_non_null))
		{
			let free_block: &mut FreeBlock = free_block_non_null.mutable_reference();

			// TODO: Currently not needed.
			// free_block.coalesce(end_of_all_blocks_non_null);

			let (floored_allocation_must_start_at, floored_allocation_must_end_at) = free_block.floored_allocation_must_start_at_and_end_at(floored_non_zero_size, floored_non_zero_power_of_two_alignment);

			if unlikely!(allocation_will_never_fit_as_there_are_not_enough_blocks_remaining(floored_allocation_must_end_at, end_of_all_blocks_non_null))
			{
				return Err(AllocErr)
			}

			if allocation_fits(floored_allocation_must_end_at, free_block)
			{
				return Ok(self.allocated(previous_free_block_raw, floored_non_zero_size, free_block, floored_allocation_must_start_at, floored_allocation_must_end_at, end_of_all_blocks_non_null))
			}

			previous_free_block_raw = free_block_non_null.as_ptr();
			free_block_non_null = free_block.next_free_block_non_null();
		}

		Err(AllocErr)
	}

	#[inline(always)]
	fn allocated(&mut self, previous_free_block_raw: *mut FreeBlock, floored_non_zero_size: NonZeroUsize, free_block: &mut FreeBlock, floored_allocation_must_start_at: MemoryAddress, floored_allocation_must_end_at: MemoryAddress, end_of_all_blocks_non_null: NonNull<FreeBlock>) -> MemoryAddress
	{
		if unlikely!(free_block.used_entire_block(floored_non_zero_size))
		{
			self.allocated_the_entire_block(free_block, previous_free_block_raw)
		}
		else
		{
			self.allocated_in_the_middle_of_the_block(free_block, previous_free_block_raw, floored_allocation_must_start_at, floored_allocation_must_end_at, end_of_all_blocks_non_null);
		}

		floored_allocation_must_start_at
	}

	#[inline(always)]
	fn allocated_the_entire_block(&mut self, free_block: &mut FreeBlock, previous_free_block_raw: *mut FreeBlock)
	{
		let next_free_block_non_null = free_block.next_free_block_non_null();

		let is_first_free_block_in_list = previous_free_block_raw.is_null();
		if unlikely!(is_first_free_block_in_list)
		{
			self.first_free_block_non_null = next_free_block_non_null;
		}
		else
		{
			let previous_free_block_non_null = previous_free_block_raw.non_null();
			let previous_free_block: &mut FreeBlock = next_free_block_non_null.mutable_reference();
			previous_free_block.offset_from_start_of_this_block_to_next_block = FreeBlock::difference_u32_non_zero(next_free_block_non_null, previous_free_block_non_null);
		}
	}

	#[inline(always)]
	fn allocated_in_the_middle_of_the_block(&mut self, free_block: &mut FreeBlock, previous_free_block_raw: *mut FreeBlock, floored_allocation_must_start_at: MemoryAddress, floored_allocation_must_end_at: MemoryAddress, end_of_all_blocks_non_null: NonNull<FreeBlock>)
	{
		let needs_front = floored_allocation_must_start_at > free_block.starts_at();
		let needs_back = floored_allocation_must_end_at < free_block.ends_at();

		if unlikely!(needs_front)
		{
			let next_free_block_non_null = free_block.next_free_block_non_null();
			let next_block_non_null = if likely!(needs_back)
			{
				FreeBlock::initialize_back_block(next_free_block_non_null, free_block, floored_allocation_must_end_at, end_of_all_blocks_non_null)
			}
			else
			{
				next_free_block_non_null
			};

			let front_block = free_block;
			front_block.initialize_front_block_and_coalesce_if_possible(floored_allocation_must_start_at, next_block_non_null, previous_free_block_raw);
		}
		else
		{
			if likely!(needs_back)
			{
				let next_free_block_non_null = free_block.next_free_block_non_null();
				let back_block_non_null = FreeBlock::initialize_back_block(next_free_block_non_null, free_block, floored_allocation_must_end_at, end_of_all_blocks_non_null);

				let is_first_free_block_in_list = previous_free_block_raw.is_null();
				if unlikely!(is_first_free_block_in_list)
				{
					self.first_free_block_non_null = back_block_non_null;
				}
				else
				{
					let previous_free_block_non_null = previous_free_block_raw.non_null();
					let difference = FreeBlock::difference_u32_non_zero(back_block_non_null, previous_free_block_non_null);

					let previous_free_block: &mut FreeBlock = previous_free_block_non_null.mutable_reference();
					previous_free_block.offset_from_start_of_this_block_to_next_block = difference;
				}
			}
		}
	}
}

/// Is always exactly 8 bytes, which is the minimum block size.
#[derive(Debug)]
struct FreeBlock
{
	// TODO: Since our minimum block size is 8 bytes, we can actually scale this value by << 3 (ie achieve a 8x fold increase, from 4Gb to 32Gb).
	offset_from_start_of_this_block_to_next_block: NonZeroU32,

	// TODO: Since our minimum block size is 8 bytes, we can actually scale this value by << 3 (ie achieve a 8x fold increase, from 4Gb to 32Gb).
	size_of_this_block: NonZeroU32,
}

impl FreeBlock
{
	const MinimumBlockSize: usize = size_of::<Self>();

	const MinimumAllocationSize: NonZeroUsize = non_zero_usize(Self::MinimumBlockSize);

	const MaximumAllocationSize: NonZeroUsize = non_zero_usize(size_of::<u32>()));

	const MinimumAlignment: NonZeroUsize = Self::MinimumAllocationSize;

	const MaximumAlignment: NonZeroUsize = Self::MaximumAllocationSize;

	#[inline(always)]
	pub(crate) fn floored_allocation_must_start_at_and_end_at(&self, floored_non_zero_size: NonZeroUsize, floored_non_zero_power_of_two_alignment: NonZeroUsize) -> (MemoryAddress, MemoryAddress)
	{
		Self::debug_assert_floored_non_zero_size(floored_non_zero_size);
		Self::debug_assert_floored_non_zero_power_of_two_alignment(floored_non_zero_power_of_two_alignment);

		let starts_at = self.starts_at();

		let floored_allocation_must_start_at = starts_at.round_up_to_power_of_two(floored_non_zero_power_of_two_alignment);

		debug_assert!(floored_allocation_must_start_at >= starts_at, "floored_allocation_must_start_at `{:?}` is less than starts_at `{:?}`", floored_allocation_must_start_at, starts_at);

		let floored_allocation_must_end_at = floored_allocation_must_start_at.add_non_zero(floored_non_zero_size);

		(floored_allocation_must_start_at, floored_allocation_must_end_at)
	}

	#[inline(always)]
	pub(crate) fn non_null(&self) -> NonNull<Self>
	{
		(self as *const Self).non_null()
	}

	#[inline(always)]
	pub(crate) fn next_free_block_non_null(&self) -> NonNull<Self>
	{
		(self as *const Self).add_bytes_non_zero_u32(self.offset_from_start_of_this_block_to_next_block).non_null()
	}

	#[inline(always)]
	pub(crate) fn no_more_free_blocks(&self, end_of_all_blocks_non_null: NonNull<Self>) -> bool
	{
		let free_block_non_null = self.non_null();

		Self::debug_assert_within_free_blocks_memory(free_block_non_null, end_of_all_blocks_non_null);

		free_block_non_null == end_of_all_blocks_non_null
	}

	#[inline(always)]
	pub(crate) fn more_free_blocks(free_block_non_null: NonNull<Self>, end_of_all_blocks_non_null: NonNull<Self>) -> bool
	{
		Self::debug_assert_within_free_blocks_memory(free_block_non_null, end_of_all_blocks_non_null);

		free_block_non_null != end_of_all_blocks_non_null
	}

	#[inline(always)]
	pub(crate) fn starts_at(&self) -> MemoryAddress
	{
		self.non_null().cast::<u8>()
	}

	#[inline(always)]
	pub(crate) fn ends_at(&self) -> MemoryAddress
	{
		let size_of_this_block = self.size_of_this_block_as_usize();
		let starts_at = self.starts_at();

		starts_at.add(size_of_this_block)
	}

	#[inline(always)]
	fn next_contiguous_block<'a>(&self) -> &'a Self
	{
		self.next_contiguous_block_non_null().reference()
	}

	#[inline(always)]
	fn next_contiguous_block_non_null(&self) -> NonNull<Self>
	{
		self.ends_at().cast::<Self>()
	}

	#[inline(always)]
	pub(crate) fn used_entire_block(&self, floored_non_zero_size: NonZeroUsize) -> bool
	{
		Self::debug_assert_floored_non_zero_size(floored_non_zero_size);

		self.size_of_this_block == floored_non_zero_size.to_non_zero_u32()
	}

	#[inline(always)]
	pub(crate) fn initialize_front_block_and_coalesce_if_possible(&mut self, floored_allocation_must_start_at: MemoryAddress, next_block_non_null: NonNull<Self>, previous_free_block_raw: *mut Self)
	{
		let front_block = self;
		front_block.offset_from_start_of_this_block_to_next_block =  Self::difference_u32_non_zero(next_block_non_null, front_block.non_null());
		front_block.size_of_this_block = floored_allocation_must_start_at.difference_u32_non_zero(front_block.starts_at());
		Self::coalesce_previous_free_block_with_front_block_if_possible(previous_free_block_raw, front_block)
	}

	#[inline(always)]
	pub(crate) fn initialize_back_block(next_free_block_non_null: NonNull<Self>, free_block: &Self, floored_allocation_must_end_at: MemoryAddress, end_of_all_blocks_non_null: NonNull<Self>) -> NonNull<Self>
	{
		let back_block_non_null = floored_allocation_must_end_at.cast::<Self>();

		let back_block: &mut Self = back_block_non_null.mutable_reference();
		back_block.offset_from_start_of_this_block_to_next_block = Self::difference_u32_non_zero(next_free_block_non_null, back_block_non_null);
		back_block.size_of_this_block = Self::difference_u32_non_zero(free_block.next_contiguous_block_non_null(), back_block_non_null);
		back_block.coalesce(end_of_all_blocks_non_null);

		back_block_non_null
	}

	#[inline(always)]
	fn difference_u32_non_zero(later_block: NonNull<Self>, earlier_block: NonNull<Self>) -> NonZeroU32
	{
		later_block.cast::<u8>().difference_u32_non_zero(earlier_block.cast::<u8>())
	}

	#[inline(always)]
	pub(crate) fn coalesce(&mut self, end_of_all_blocks_non_null: NonNull<Self>)
	{
		while unlikely!(self.next_free_block_starts_contiguously_after_this())
		{
			if unlikely!(self.no_more_free_blocks(end_of_all_blocks_non_null))
			{
				return
			}

			let next_contiguous_free_block = self.next_contiguous_block();
			self.coalesce_with_next_contiguous_free_block(next_contiguous_free_block)
		}
	}

	#[inline(always)]
	pub(crate) fn coalesce_previous_free_block_with_front_block_if_possible(previous_free_block_raw: *mut Self, front_block: &Self)
	{
		let previous_free_block_exists = !previous_free_block_raw.is_null();
		if likely!(previous_free_block_exists)
		{
			let previous_free_block: &mut Self = previous_free_block_raw.non_null().mutable_reference();
			let front_block_is_contiguous_after_the_previous_free_block = previous_free_block.next_contiguous_block_non_null() == front_block.non_null();
			if unlikely!(front_block_is_contiguous_after_the_previous_free_block)
			{
				previous_free_block.coalesce_with_next_contiguous_free_block(front_block);
			}
		}
	}

	#[inline(always)]
	fn next_free_block_starts_contiguously_after_this(&self) -> bool
	{
		self.offset_from_start_of_this_block_to_next_block == self.size_of_this_block
	}

	#[inline(always)]
	fn coalesce_with_next_contiguous_free_block<'a>(&mut self, next_contiguous_free_block: &'a Self)
	{
		self.increment_offset_from_start_of_this_block_to_next_block(next_contiguous_free_block.offset_from_start_of_this_block_to_next_block);
		self.increment_size_of_this_block(next_contiguous_free_block.size_of_this_block);
	}

	#[inline(always)]
	fn increment_offset_from_start_of_this_block_to_next_block(&mut self, increment: NonZeroU32)
	{
		debug_assert!(self.offset_from_start_of_this_block_to_next_block.checked_add(increment).is_some(), "coalesced block will exceed memory address space");
		self.offset_from_start_of_this_block_to_next_block.add_assign(increment)
	}

	#[inline(always)]
	fn increment_size_of_this_block(&mut self, increment: NonZeroU32)
	{
		debug_assert!(self.size_of_this_block.checked_add(increment).is_some(), "coalesced block will exceed size");
		self.size_of_this_block.add_assign(increment)
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
	fn size_of_this_block_as_usize(&self) -> usize
	{
		let size_of_this_block_usize = self.size_of_this_block.get() as usize;

		debug_assert!(self.starts_at().checked_add(size_of_this_block_usize).is_some(), "block exceeds memory address space");

		size_of_this_block_usize
	}

	#[inline(always)]
	fn debug_assert_within_free_blocks_memory(free_block_non_null: NonNull<Self>, end_of_all_blocks_non_null: NonNull<Self>)
	{
		debug_assert!(end_of_all_blocks_non_null >= free_block_non_null, "Got beyond end of free blocks");
	}

	#[inline(always)]
	fn debug_assert_floored_non_zero_size(floored_non_zero_size: NonZeroUsize)
	{
		debug_assert!(floored_non_zero_size >= Self::MinimumAllocationSize, "non_zero_size `{}` is less than MinimumAllocationSize `{}`", floored_non_zero_size, Self::MinimumAllocationSize);
		debug_assert!(floored_non_zero_size <= Self::MaximumAllocationSize, "non_zero_size `{}` exceeds MaximumAllocationSize `{}`", floored_non_zero_size, Self::MaximumAllocationSize);
	}

	#[inline(always)]
	fn debug_assert_floored_non_zero_power_of_two_alignment(floored_non_zero_power_of_two_alignment: NonZeroUsize)
	{
		debug_assert!(floored_non_zero_power_of_two_alignment >= Self::MinimumAlignment, "floored_non_zero_power_of_two_alignment `{}` is less than MinimumAlignment `{}`", floored_non_zero_power_of_two_alignment, Self::MinimumAlignment);
		debug_assert!(floored_non_zero_power_of_two_alignment <= Self::MaximumAlignment, "floored_non_zero_power_of_two_alignment `{}` exceeds MaximumAlignment `{}`", floored_non_zero_power_of_two_alignment, Self::MaximumAlignment);
	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// An arena memory source.
#[derive(Debug)]
pub struct Arena<MS: MemorySource>
{
	next_available_slot_index: Cell<SlotIndex>,

	block_size: NonZeroUsize,
	#[cfg(debug_assertions)] number_of_blocks: NonZeroUsize,

	memory_source: MS,
	allocations_start_from: MemoryAddress,
	memory_source_size: NonZeroUsize,
}

impl<MS: MemorySource> Drop for Arena<MS>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.memory_source.release(self.memory_source_size, self.allocations_start_from)
	}
}

impl<MS: MemorySource> MemorySource for Arena<MS>
{
	#[inline(always)]
	fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocError>
	{
		debug_assert!(non_zero_size <= self.block_size);

		let next_available_slot_index = self.next_available_slot_index.get();

		if unlikely!(next_available_slot_index.is_fully_allocated())
		{
			return Err(AllocError)
		}

		let unallocated_block = self.unallocated_block(next_available_slot_index);
		self.next_available_slot_index.set(unallocated_block.next_available_slot_index());

		Ok(unallocated_block.to_memory_address())
	}

	#[inline(always)]
	fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress)
	{
		debug_assert!(non_zero_size <= self.block_size);

		let unallocated_block = UnallocatedBlock::from_memory_address(current_memory);
		unallocated_block.set_unoccupied_next_available_slot_index(self.next_available_slot_index.get());

		self.next_available_slot_index.set(self.slot_index_from_block(unallocated_block));
	}
}

impl<MS: MemorySource> Arena<MS>
{
	/// Create a new instance by memory size and block size.
	#[inline(always)]
	pub fn new_by_amount(memory_source: MS, block_size: NonZeroUsize, memory_source_size: NonZeroUsize, block_initializer: impl Fn(MemoryAddress, NonZeroUsize)) -> Result<Self, AllocError>
	{
		let number_of_blocks = ((memory_source_size.get() + (block_size.get() - 1)) / block_size.get()).non_zero();

		Self::new(memory_source, block_size, number_of_blocks, block_initializer)
	}

	/// Creates a new instance.
	///
	/// `block_size` must be at least 8 to be useful.
	/// `block_initializer` takes the address of a block and the size of a block; after it is called, the block will have the first 8 bytes (4 bytes on 32-bit platforms) overwritten with a slot index pointer.
	#[inline(always)]
	pub fn new(memory_source: MS, block_size: NonZeroUsize, number_of_blocks: NonZeroUsize, block_initializer: impl Fn(MemoryAddress, NonZeroUsize)) -> Result<Self, AllocError>
	{
		let memory_source_size = block_size.multiply(number_of_blocks);

		let allocations_start_from = memory_source.obtain(memory_source_size)?;

		Self::initialize_blocks_so_they_are_a_singly_linked_list(block_size, block_initializer, memory_source_size, allocations_start_from);

		Ok
		(
			Self
			{
				next_available_slot_index: Cell::default(),

				block_size,
				#[cfg(debug_assertions)] number_of_blocks,

				memory_source,
				allocations_start_from,
				memory_source_size,
			}
		)
	}

	#[inline(always)]
	fn initialize_blocks_so_they_are_a_singly_linked_list(block_size: NonZeroUsize, block_initializer: impl Fn(MemoryAddress, NonZeroUsize), memory_source_size: NonZeroUsize, allocations_start_from: MemoryAddress)
	{
		let mut slot_index = SlotIndex(1);
		let mut block_memory_address = allocations_start_from;
		let allocations_end_at = allocations_start_from.add_non_zero(memory_source_size);
		let allocations_end_at_less_one_block = allocations_end_at.subtract_non_zero(block_size);
		while block_memory_address != allocations_end_at_less_one_block
		{
			let unallocated_block = UnallocatedBlock::from_memory_address(block_memory_address);
			unallocated_block.initialize(block_size, &block_initializer, slot_index);

			slot_index.increment();
			block_memory_address.add_assign_non_zero(block_size)
		}
		UnallocatedBlock::from_memory_address(allocations_end_at_less_one_block).initialize(block_size, &block_initializer, SlotIndex::IsFullyAllocatedNextAvailableSlotIndexSentinel);
	}

	#[inline(always)]
	fn unallocated_block(&self, slot_index: SlotIndex) -> &UnallocatedBlock
	{
		UnallocatedBlock::from_memory_address(self.block_from_slot_index(slot_index))
	}

	#[inline(always)]
	fn block_from_slot_index(&self, slot_index: SlotIndex) -> MemoryAddress
	{
		debug_assert_ne!(slot_index, SlotIndex::IsFullyAllocatedNextAvailableSlotIndexSentinel, "Should never get IsFullyAllocatedNextAvailableSlotIndexSentinel for `slot_index`");

		debug_assert!(slot_index.0 < self.number_of_blocks.get(), "Arena index was out-of-range");

		self.allocations_start_from.add(self.block_size.get() * slot_index.0)
	}

	#[inline(always)]
	fn slot_index_from_block(&self, unallocated_block: &UnallocatedBlock) -> SlotIndex
	{
		SlotIndex(unallocated_block.to_memory_address().difference(self.allocations_start_from) / self.block_size.get())
	}
}

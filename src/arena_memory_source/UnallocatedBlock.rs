// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[repr(C)]
struct UnallocatedBlock
{
	next_available_slot_index: Cell<SlotIndex>,
	_remainder: Unsized,
}

impl UnallocatedBlock
{
	#[inline(always)]
	fn next_available_slot_index(&self) -> SlotIndex
	{
		self.next_available_slot_index.get()
	}

	#[inline(always)]
	fn set_unoccupied_next_available_slot_index(&self, slot_index: SlotIndex)
	{
		self.next_available_slot_index.set(slot_index)
	}

	#[inline(always)]
	fn from_memory_address<'a>(memory_address: MemoryAddress) -> &'a Self
	{
		unsafe { & * (memory_address.as_ptr() as *const Self) }
	}

	#[inline(always)]
	fn to_memory_address(&self) -> MemoryAddress
	{
		(self as *const Self as *const u8 as *mut u8).non_null()
	}
}

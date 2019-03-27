// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This allocator allocates memory-mapped data.
///
/// It is slow and uses system calls.
///
/// When dropped, any memory allocated with this allocator is ***NOT*** freed.
///
/// However, it is appropriate as a 'backing store' for other allocators.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryMapAllocator;

impl Default for MemoryMapAllocator
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
	}
}

impl Allocator for MemoryMapAllocator
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		const AssumedPageSize: usize = 4096;

		if unlikely!(non_zero_power_of_two_alignment.get() > AssumedPageSize)
		{
			return Err(AllocErr)
		}

		self.mmap_memory(non_zero_size.get())
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		Self::munmap_memory(current_memory, non_zero_size.get())
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		Self::mremap_memory(current_memory, non_zero_current_size.get(), non_zero_new_size.get())
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		Self::mremap_memory(current_memory, non_zero_current_size.get(), non_zero_new_size.get())
	}
}

impl MemoryMapAllocator
{
	/// `size` is rounded up to system page size.
	#[inline(always)]
	fn mmap_memory(&self, size: usize) -> Result<MemoryAddress, AllocErr>
	{
		let result = unsafe { mmap(null_mut(), size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, 0, 0) };
		if unlikely!(result == MAP_FAILED)
		{
			Err(AllocErr)
		}
		else
		{
			Ok(result.non_null().cast::<u8>())
		}
	}

	/// `size` is rounded up to system page size.
	#[inline(always)]
	fn mremap_memory(memory_address: MemoryAddress, old_size: usize, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		let result = unsafe { mremap(memory_address.as_ptr() as *mut _, old_size, new_size, MREMAP_MAYMOVE) };
		if unlikely!(result == MAP_FAILED)
		{
			Err(AllocErr)
		}
		else
		{
			Ok(result.non_null().cast::<u8>())
		}
	}

	/// `size` is rounded up to system page size.
	#[inline(always)]
	fn munmap_memory(memory_address: MemoryAddress, size: usize)
	{
		unsafe { munmap(memory_address.as_ptr() as *mut _, size) };
	}
}


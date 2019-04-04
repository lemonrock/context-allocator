// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This allocator allocates memory-mapped data local to NUMA nodes.
///
/// It is slow and uses system calls.
///
/// When dropped, any memory allocated with this allocator is ***NOT*** freed.
///
/// However, it is appropriate as a 'backing store' for other allocators.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumaMemoryMapAllocator
{
	memory_map_allocator: MemoryMapAllocator,

	mbind_mode: i32,

	mbind_nodemask: Option<usize>,

	mbind_maxnode: usize,

	mbind_flags: u32,
}

impl Default for NumaMemoryMapAllocator
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(MemoryMapAllocator::default(), NumaAllocationPolicy::default(), false)
	}
}

impl LargeAllocator for NumaMemoryMapAllocator
{
}

impl Allocator for NumaMemoryMapAllocator
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		let current_memory = self.memory_map_allocator.allocate(non_zero_size, non_zero_power_of_two_alignment)?;

		let nodemask = match self.mbind_nodemask
		{
			None => null(),
			Some(ref pointer) => pointer as *const usize,
		};

		let error_number = Self::mbind(current_memory.as_ptr() as *mut _, non_zero_size.get(), self.mbind_mode, nodemask, self.mbind_maxnode, self.mbind_flags);
		if likely!(error_number >= 0)
		{
			Ok(current_memory)
		}
		else if likely!(error_number < 0)
		{
			self.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory);
			Err(AllocErr)
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		self.memory_map_allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.memory_map_allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.memory_map_allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}
}

impl NumaMemoryMapAllocator
{
	/// Creates a new instance.
	///
	/// * `memory_map_allocator`: Underlying memory map allocator.
	/// * `allocation_policy`: NUMA node allocation policy.
	/// * `strict`: Force allocations to migrate to NUMA nodes specified in `allocation_policy` or fail to allocate.
	#[inline(always)]
	pub fn new(memory_map_allocator: MemoryMapAllocator, allocation_policy: NumaAllocationPolicy, strict: bool) -> Self
	{
		let (policy, (mode_flags, mbind_nodemask, mbind_maxnode)) = allocation_policy.values();
		let mbind_mode = policy | mode_flags;

		Self
		{
			memory_map_allocator,
			mbind_mode,
			mbind_nodemask,
			mbind_maxnode,
			mbind_flags: Self::mbind_flags(strict),
		}
	}

	#[inline(always)]
	fn mbind_flags(strict: bool) -> u32
	{
		if likely!(strict)
		{
			const MPOL_MF_STRICT: u32 = 1 << 0;
			const MPOL_MF_MOVE: u32 = 1 << 1;
			// Requires CAP_SYS_NICE.
			// const MPOL_MF_MOVE_ALL: i32 = 1<< 2;
			MPOL_MF_STRICT | MPOL_MF_MOVE
		}
		else
		{
			0
		}
	}

	/// Returns zero or positive for success and a negative error number for failure.
	#[inline(always)]
	fn mbind(start: *mut c_void, len: usize, mode: i32, nodemask: *const usize, maxnode: usize, flags: u32) -> isize
	{
		unsafe { Syscall::mbind.syscall6(start as isize, len as isize, mode as isize, nodemask as isize, maxnode as isize, flags as isize) }
	}
}


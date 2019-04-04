// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Represents settings for NUMA allocation.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumaSettings
{
	#[cfg(any(target_os = "android", target_os = "linux"))] mbind_mode: i32,

	#[cfg(any(target_os = "android", target_os = "linux"))] mbind_nodemask: Option<usize>,

	#[cfg(any(target_os = "android", target_os = "linux"))] mbind_maxnode: usize,

	#[cfg(any(target_os = "android", target_os = "linux"))] mbind_flags: u32,
}

#[cfg(any(target_os = "android", target_os = "linux"))]
impl NumaSettings
{
	/// Creates a new instance.
	///
	/// * `allocation_policy`: NUMA node allocation policy (ignored on operating systems other than Android and Linux).
	/// * `strict`: Force allocations to migrate to NUMA nodes specified in `allocation_policy` or fail to allocate (ignored on operating systems other than Android and Linux).
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn new(allocation_policy: NumaAllocationPolicy, strict: bool) -> Self
	{
		let (policy, (mode_flags, mbind_nodemask, mbind_maxnode)) = allocation_policy.values();
		let mbind_mode = policy | mode_flags;

		Self
		{
			mbind_mode,
			mbind_nodemask,
			mbind_maxnode,
			mbind_flags: Self::mbind_flags(strict),
		}
	}

	/// Creates a new instance.
	///
	/// * `allocation_policy`: NUMA node allocation policy (ignored on operating systems other than Android and Linux).
	/// * `strict`: Force allocations to migrate to NUMA nodes specified in `allocation_policy` or fail to allocate (ignored on operating systems other than Android and Linux).
	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	#[inline(always)]
	pub fn new(_allocation_policy: NumaAllocationPolicy, _strict: bool) -> Self
	{
		Self
	}

	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub(crate) fn post_allocate(&self, address: *mut c_void, size: usize) -> Result<(), ()>
	{
		let nodemask = match self.mbind_nodemask
		{
			None => null(),
			Some(ref pointer) => pointer as *const usize,
		};

		let error_number = Self::mbind(address, size, self.mbind_mode, nodemask, self.mbind_maxnode, self.mbind_flags);
		if likely!(error_number >= 0)
		{
			Ok(())
		}
		else if likely!(error_number < 0)
		{
			Err(())
		}
		else
		{
			unreachable!()
		}
	}

	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	#[inline(always)]
	pub(crate) fn post_allocate(&self, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		Ok(current_memory)
	}

	#[cfg(any(target_os = "android", target_os = "linux"))]
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
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	fn mbind(start: *mut c_void, len: usize, mode: i32, nodemask: *const usize, maxnode: usize, flags: u32) -> isize
	{
		unsafe { Syscall::mbind.syscall6(start as isize, len as isize, mode as isize, nodemask as isize, maxnode as isize, flags as isize) }
	}
}

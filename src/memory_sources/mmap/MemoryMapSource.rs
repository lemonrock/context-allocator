// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This NUMA-aware memory source allocates memory-mapped data, optionally using NUMA policy to allocate on a memory node closest to the current thread.
///
/// It is slow and uses system calls.
///
/// When dropped, any memory obtained with this allocator is ***NOT*** freed.
///
/// However, it is appropriate as a 'backing store' for other memory sources.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemoryMapSource
{
	/// Mapped memory settings.
	pub settings: MappedMemorySettings,

	/// Defaults.
	pub defaults: DefaultPageSizeAndHugePageSizes
}

impl MemorySource for MemoryMapSource
{
	#[inline(always)]
	fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		let mapped_memory = self.settings.anonymous_memory_map(unsafe { NonZeroU64::new_unchecked(non_zero_size.get() as u64) }, &self.defaults).map_err(|_: MemoryMapError| AllocErr)?;
		let memory_address: MemoryAddress = mapped_memory.virtual_address().into();
		forget(mapped_memory);
		Ok(memory_address)
	}

	#[inline(always)]
	fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress)
	{
		let result = unsafe { munmap(current_memory.as_ptr() as *mut c_void, non_zero_size.get()) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			panic!("munmap() returned an error of {}", errno())
		}
		else
		{
			panic!("munmap() failed with an unexpected exit code of {:?}", result)
		}
	}
}

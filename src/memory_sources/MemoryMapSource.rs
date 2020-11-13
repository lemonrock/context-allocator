// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This NUMA-aware memory source allocates memory-mapped data, optionally using NUMA policy to allocate on a memory node closest to the current thread.
///
/// It is slow and uses system calls.
///
/// When dropped, any memory obtained with this allocator is ***NOT*** freed.
///
/// However, it is appropriate as a 'backing store' for other memory sources.
#[derive(Debug)]
pub struct MemoryMapSource(MappedMemory);

impl MemorySource for MemoryMapSource
{
	#[inline(always)]
	fn size(&self) -> NonZeroUsize
	{
		let size = self.0.mapped_size_in_bytes();
		new_non_zero_usize(size)
	}
	
	#[inline(always)]
	fn allocations_start_from(&self) -> MemoryAddress
	{
		self.0.virtual_address().into()
	}
}

impl MemoryMapSource
{
	/// New instance.
	#[inline(always)]
	pub fn new(size: NonZeroU64, settings: MappedMemorySettings, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, MemoryMapError>
	{
		settings.anonymous_memory_map(size, defaults).map(|mapped_memory| Self(mapped_memory))
	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A memory source is a sort-of crude allocator that can obtain and release memory, from, say, the operating system, an arena or some fixed range.
///
/// It is thread-aware but not necessarily thread-safe.
pub trait MemorySource: Debug
{
	/// Obtain memory from the operating system, say.
	///
	/// Alignment will be whatever is appropriate, but is likely to be quite large.
	fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocErr>;

	/// Release memory to the operating system, say.
	///
	/// Alignment will be whatever is appropriate, but is likely to be quite large.
	fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress);
}

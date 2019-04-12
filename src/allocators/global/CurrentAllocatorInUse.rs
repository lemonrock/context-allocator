// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Records which allocator is currently in use for `Global` allocations.
///
/// This does not affect reallocations or deallocations in any way.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CurrentAllocatorInUse
{
	/// A coroutine local allocator.
	CoroutineLocal,

	/// A thread local allocator.
	ThreadLocal,

	/// A global allocator.
	Global,
}

// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Stack memory.
///
/// On x86-64, the stack needs to be 16 byte aligned with a minimum size of 64 bytes in order to store a `SavedContext`, hence the alignment of `64` (over `16`).
#[repr(C, align(64))]
pub struct CoroutineStackMemory<StackSize: Sized>
{
	sizing: StackSize
}

impl<StackSize: Sized> Debug for CoroutineStackMemory<StackSize>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "CoroutineStackMemory({})", size_of::<Self>())
	}
}

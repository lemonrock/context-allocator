// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait NonNullU8NodePointer: NonNullU8Ext
{
	#[inline(always)]
	fn node_pointer(self) -> NodePointer
	{
		NodePointer::from_memory_address(self.to_non_null_u8())
	}
}

impl NonNullU8NodePointer for NonNull<u8>
{
}

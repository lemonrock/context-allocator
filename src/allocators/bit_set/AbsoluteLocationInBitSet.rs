// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This is a mixed-radix representation.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct AbsoluteLocationInBitSet
{
	major: BitSetWordPointer,
	minor: NumberOfBits,
}

impl AbsoluteLocationInBitSet
{
	#[inline(always)]
	fn align_upwards_to_next_bit_set_word_pointer<R>(self, value_to_return_if_aligned: R, action_if_unaligned: impl FnOnce(&Self) -> R) -> (BitSetWordPointer, R)
	{
		if unlikely!(self.minor.is_zero())
		{
			(self.major, value_to_return_if_aligned)
		}
		else
		{
			let value_to_return = action_if_unaligned(&self);
			(self.major.increment(), value_to_return)
		}
	}
}

// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This is a mixed-radix representation.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RelativeLocationInBitSet
{
	major: NumberOfBitSetWords,
	minor: NumberOfBits,
}

impl RelativeLocationInBitSet
{
	#[inline(always)]
	fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		AbsoluteLocationInBitSet
		{
			major: inclusive_start_of_bitset.increment_in_bit_set_words(self.major),
			minor: self.minor,
		}
	}
}

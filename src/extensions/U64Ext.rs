// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


pub(crate) trait U64Ext: Sized + Copy + Ord + Debug
{
	#[inline(always)]
	fn round_down_to_power_of_two_exponent_usize(self, power_of_two_exponent: usize) -> u64
	{
		self.round_down_to_power_of_two_exponent(power_of_two_exponent as u64)
	}

	#[inline(always)]
	fn round_down_to_power_of_two_exponent(self, power_of_two_exponent: u64) -> u64
	{
		let value = self.to_u64();

		value & !((1 << power_of_two_exponent) - 1)
	}

	#[doc(hidden)]
	fn to_u64(self) -> u64;
}

impl U64Ext for u64
{
	#[inline(always)]
	fn to_u64(self) -> u64
	{
		self
	}
}

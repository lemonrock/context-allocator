// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Local allocator memory usage.
///
/// Only accurate when recorded.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalAllocatorMemoryUsage
{
	allocated: Cell<u64>,
	
	deallocated: Cell<u64>,
	
	growing_reallocated: Cell<u64>,
	
	shrinking_reallocated: Cell<u64>,
}

impl Default for LocalAllocatorMemoryUsage
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl Sub for LocalAllocatorMemoryUsage
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		Self
		{
			allocated: Cell::new(self.get_allocated() - rhs.get_allocated()),
			deallocated: Cell::new(self.get_deallocated() - rhs.get_deallocated()),
			growing_reallocated: Cell::new(self.get_growing_reallocated() - rhs.get_growing_reallocated()),
			shrinking_reallocated: Cell::new(self.get_shrinking_reallocated() - rhs.get_shrinking_reallocated())
		}
	}
}

impl LocalAllocatorMemoryUsage
{
	/// New instance.
	pub const fn new() -> Self
	{
		Self
		{
			allocated: Cell::new(0),
			
			deallocated: Cell::new(0),
			
			growing_reallocated: Cell::new(0),
			
			shrinking_reallocated: Cell::new(0),
		}
	}
	
	/// Memory usage.
	#[inline(always)]
	pub fn usage(&self) -> u64
	{
		(self.get_allocated() + self.get_growing_reallocated()) - (self.get_deallocated() + self.get_shrinking_reallocated())
	}
	
	#[inline(always)]
	fn allocated(&self, size: usize)
	{
		self.allocated.set(self.get_allocated() + size as u64);
	}
	
	#[inline(always)]
	fn deallocated(&self, size: NonZeroUsize)
	{
		self.deallocated.set(self.get_deallocated() + size.get() as u64);
	}
	
	#[inline(always)]
	fn growing_reallocated(&self, non_zero_current_size: NonZeroUsize, size: usize)
	{
		self.growing_reallocated.set(self.get_growing_reallocated() + ((size - non_zero_current_size.get()) as u64));
	}
	
	#[inline(always)]
	fn shrinking_reallocated(&self, non_zero_current_size: NonZeroUsize, size: usize)
	{
		self.shrinking_reallocated.set(self.get_shrinking_reallocated() + ((non_zero_current_size.get() - size) as u64));
	}
	
	#[inline(always)]
	fn get_allocated(&self) -> u64
	{
		self.allocated.get()
	}
	
	#[inline(always)]
	fn get_deallocated(&self) -> u64
	{
		self.deallocated.get()
	}
	
	#[inline(always)]
	fn get_growing_reallocated(&self) -> u64
	{
		self.growing_reallocated.get()
	}
	
	#[inline(always)]
	fn get_shrinking_reallocated(&self) -> u64
	{
		self.shrinking_reallocated.get()
	}
}

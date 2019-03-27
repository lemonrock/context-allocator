// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// NUMA nodes to allocate on.
///
/// If set to no nodes (the `Default::default()`) then memory is allocated on the local node if possible.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumaNodeBitSet
{
	bits: usize,

	/// Specifies physical node IDs.
	///
	/// Linux does not remap the nodemask when the thread moves to a different cpuset context, nor when the set of nodes allowed by the thread's current cpuset context changes.
	///
	/// (Not used if there are no nodes specified).
	pub static_nodes: bool,

	/// Specifies specifies node IDs that are relative to the set of node IDs allowed by the thread's current cpuset.
	///
	/// (Not used if there are no nodes specified).
	pub relative_nodes: bool,
}

impl NumaNodeBitSet
{
	/// Is this the empty set?
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.bits == 0
	}

	/// Add a NUMA node into the set.
	#[inline(always)]
	pub fn insert_numa_node(&mut self, zero_based_node_index: u8)
	{
		self.bits |= 1 << (zero_based_node_index as usize)
	}

	/// Remove a NUMA node from the set.
	#[inline(always)]
	pub fn remove_numa_node(&mut self, zero_based_node_index: u8)
	{
		self.bits &= !(1 << (zero_based_node_index as usize))
	}

	#[inline(always)]
	fn mask_and_size(&self) -> (i32, *const usize, usize)
	{
		if likely!(self.is_empty())
		{
			(0, null_mut(), 0)
		}
		else
		{
			let size = size_of::<usize>();

			let mut mode_flags = 0;
			if unlikely!(self.static_nodes)
			{
				const MPOL_F_STATIC_NODES: i32 = 1 << 15;
				mode_flags |= MPOL_F_STATIC_NODES
			}
			if unlikely!(self.relative_nodes)
			{
				const MPOL_F_RELATIVE_NODES: i32 = 1 << 14;
				mode_flags |= MPOL_F_RELATIVE_NODES
			}

			(mode_flags, &self.bits, size + 1)
		}
	}
}

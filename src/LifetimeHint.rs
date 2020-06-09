// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A hint for choosing a memory allocator.
#[derive(Debug)]
pub enum LifetimeHint
{
	/// Use this variant for contexts with short-lived lifetimes.
	///
	/// Very fast allocation and almost costless deallocation, at the expense of the strong likelihood of running out of memory.
	///
	/// Reallocation is very expensive when growing unless reallocating the most recently made allocation.
	ShortLived,

	/// Use this variant for contexts with slightly longer than short-lived lifetimes.
	///
	/// Slower allocation and deallocation but reallocation is less expensive than for `ShortLived`.
	MediumLived,

	/// Use this variant for contexts with long-lived lifetimes.
	LongLived,
}

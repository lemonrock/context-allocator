// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Deliberately structured like Layout to provide access to fields.
pub(crate) struct LayoutHack
{
	pub(crate) size_: usize,
	pub(crate) align_: NonZeroUsize,
}

impl LayoutHack
{
	#[inline(always)]
	pub(crate) fn access_private_fields(layout: Layout) -> Self
	{
		unsafe { transmute(layout) }
	}
}

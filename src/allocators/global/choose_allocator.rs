// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[macro_export]
macro_rules! choose_allocator
{
	($self: ident, $current_memory: ident, $callback: ident, $($argument: ident),*) =>
	{
		{
			if let Some(coroutine_local_allocator) = $self.coroutine_local_allocator()
			{
				if likely!(coroutine_local_allocator.contains($current_memory))
				{
					return coroutine_local_allocator.$callback($($argument, )*)
				}
			}

			if let Some(thread_local_allocator) = $self.thread_local_allocator()
			{
				if likely!(thread_local_allocator.contains($current_memory))
				{
					return thread_local_allocator.$callback($($argument, )*)
				}
			}

			$self.global_allocator().$callback($($argument, )*)
		}
	}
}

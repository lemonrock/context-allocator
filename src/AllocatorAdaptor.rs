// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts an `Allocator` to the `GlobalAlloc` and `Alloc` traits.
#[repr(transparent)]
#[derive(Debug)]
pub struct AllocatorAdaptor<'a, A: 'a + Allocator>(&'a A);

unsafe impl<'a, A: 'a + Allocator> GlobalAlloc for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return A::ZeroSizedAllocation.as_ptr()
		}

		let non_zero_size = NonZeroUsize::new_unchecked(zero_size);
		transmute(self.allocate(non_zero_size, layout.align_))
	}

	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
	{
		transmute(self.allocate_zeroed(layout))
	}

	#[inline(always)]
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

		if unlikely!(ptr == A::ZeroSizedAllocation.as_ptr())
		{
			return
		}

		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;
		debug_assert_ne!(zero_size, 0, "It should not be possible for a `layout.size_` to be zero if the `ptr` was the sentinel `Allocator::ZeroSizedAllocation`");
		let non_zero_size = NonZeroUsize::new_unchecked(zero_size);

		let current_memory = NonNull::new_unchecked(ptr);

		self.deallocate(non_zero_size,layout.align_, current_memory)
	}

	#[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

		transmute(self.reallocate(NonNull::new_unchecked(ptr), layout, new_size))
    }
}

unsafe impl<'a, A: 'a + Allocator> Alloc for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	unsafe fn alloc(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);
		if unlikely!(layout.size_ == 0)
		{
			return Ok(A::ZeroSizedAllocation)
		}
		let non_zero_size = NonZeroUsize::new_unchecked(layout.size_);
		self.allocate(non_zero_size, layout.align_)
	}

	#[inline(always)]
	unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		self.allocate_zeroed(layout)
	}

	#[inline(always)]
	unsafe fn dealloc(&mut self, ptr: MemoryAddress, layout: Layout)
	{
		if unlikely!(ptr == A::ZeroSizedAllocation)
		{
			return
		}

		let layout = LayoutHack::access_private_fields(layout);
		debug_assert_ne!(layout.size_, 0, "It should not be possible for a `layout.size_` to be zero if the `ptr` was the sentinel `Allocator::ZeroSizedAllocation`");

		let non_zero_size = NonZeroUsize::new_unchecked(layout.size_);
		self.deallocate(non_zero_size, layout.align_, ptr)
	}

	#[inline(always)]
	unsafe fn realloc(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		self.reallocate(ptr, layout, new_size)
	}

	#[inline(always)]
	unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);
		if unlikely!(layout.size_ == 0)
		{
			return Ok(Excess(A::ZeroSizedAllocation, 0))
		}
		let size = layout.size_;
		let non_zero_size = NonZeroUsize::new_unchecked(size);

		let result = self.allocate(non_zero_size, layout.align_);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer: *mut u8 = transmute(result);
		if unlikely!(pointer.is_null())
		{
			Err(AllocErr)
		}
		else
		{
			Ok(Excess(NonNull::new_unchecked(pointer), size))
		}
	}

	#[inline(always)]
	unsafe fn realloc_excess(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<Excess, AllocErr>
	{
		let result = self.reallocate(ptr, layout, new_size);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer: *mut u8 = transmute(result);
		if unlikely!(pointer.is_null())
		{
			Err(AllocErr)
		}
		else
		{
			Ok(Excess(NonNull::new_unchecked(pointer), new_size))
		}
	}

	#[inline(always)]
	unsafe fn grow_in_place(&mut self, _ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
	{
		let layout = LayoutHack::access_private_fields(layout);
		let size_ = layout.size_;
		debug_assert!(new_size >= size_, "new_size `{}` is less than layout.size_ `{}`", new_size, size_);
		Err(CannotReallocInPlace)
    }

	#[inline(always)]
	unsafe fn shrink_in_place(&mut self, _ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
	{
		let layout = LayoutHack::access_private_fields(layout);
		let size_ = layout.size_;
		debug_assert!(new_size <= size_, "layout.size_ `{}` is less than new_size `{}`", size_, new_size);
		Err(CannotReallocInPlace)
    }
}

impl<'a, A: 'a + Allocator> Allocator for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		self.0.allocate(non_zero_size, non_zero_power_of_two_alignment)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		self.0.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.0.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.0.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}
}

impl<'a, A: 'a + Allocator> AllocatorAdaptor<'a, A>
{
	#[doc(hidden)]
	#[inline(always)]
	fn allocate_zeroed(&self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return Ok(Self::ZeroSizedAllocation)
		}

		let non_zero_size = layout.size_.non_zero();
		let result = self.allocate(non_zero_size, layout.align_);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer = unsafe { transmute::<_, *mut u8>(result) };

		if likely!(!pointer.is_null())
		{
			unsafe { pointer.write_bytes(0x00, zero_size) };
		}

		unsafe { transmute(pointer) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn reallocate(&self, current_memory: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let current_size = layout.size_;

		if unlikely!(current_size == new_size)
		{
			return Ok(current_memory)
		}

		let non_zero_power_of_two_alignment = layout.align_;

		if likely!(new_size > current_size)
		{
			let non_zero_new_size = new_size.non_zero();

			if unlikely!(current_size == 0)
			{
				return self.allocate(non_zero_new_size, non_zero_power_of_two_alignment)
			}

			let non_zero_current_size = current_size.non_zero();
			self.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
		else
		{
			let non_zero_current_size = current_size.non_zero();

			if unlikely!(new_size == 0)
			{
				self.deallocate(non_zero_current_size, non_zero_power_of_two_alignment, current_memory);
				return Ok(Self::ZeroSizedAllocation)
			}

			let non_zero_new_size = new_size.non_zero();
			self.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
	}
}

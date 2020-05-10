// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A helper trait that brings together the core, common functionality required to implement the traits `GlobalAlloc` and `Alloc`.
pub trait Allocator: Debug + Sized
{
	/// The sentinel value used for a zero-sized allocation.
	const ZeroSizedAllocation: NonNull<u8> = non_null_pointer(usize::MAX as *mut u8);

	/// Allocate memory.
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocErr>;

	/// Deallocate (free) memory.
	///
	/// The parameter `memory` will never be the value `Self::ZeroSizedAllocation` and will always have been allocated by this `Allocator`.
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>);

	/// Reallocate memory by growing it.
	///
	/// `non_zero_new_size` will always be greater than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<(NonNull<u8>, usize), AllocErr>;

	/// Reallocate memory by shrinking it.
	///
	/// `non_zero_new_size` will always be less than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<(NonNull<u8>, usize), AllocErr>;

	/// Adapts to a `GlobalAlloc` and `Alloc`.
	#[inline(always)]
	fn adapt<'a>(&'a self) -> AllocatorAdaptor<'a, Self>
	{
		AllocatorAdaptor(self)
	}

	/// Adapts a reference to a `GlobalAlloc` and `Alloc` reference.
	#[inline(always)]
	fn adapt_reference<'a>(&'a self) -> &'a AllocatorAdaptor<'a, Self>
	{
		unsafe { transmute(self) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn allocate_zeroed(&self, layout: Layout) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return Ok((Self::ZeroSizedAllocation, 0))
		}

		let non_zero_size = layout.size_.non_zero();
		let (memory_address, actual_size) = self.allocate(non_zero_size, layout.align_)?;

		unsafe { memory_address.as_ptr().write_bytes(0x00, actual_size) };
		Ok((memory_address, actual_size))
	}

	#[doc(hidden)]
	#[inline(always)]
	fn reallocate(&self, current_memory: NonNull<u8>, layout: Layout, new_size: usize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let current_size = layout.size_;

		if unlikely!(current_size == new_size)
		{
			return Ok((current_memory, new_size))
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
				return Ok((Self::ZeroSizedAllocation, new_size))
			}

			let non_zero_new_size = new_size.non_zero();
			self.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_alloc(&self, layout: Layout) -> *mut u8
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return Self::ZeroSizedAllocation.as_ptr()
		}

		let non_zero_size = NonZeroUsize::new_unchecked(zero_size);
		match self.allocate(non_zero_size, layout.align_)
		{
			Ok((memory_address, _actual_size)) => memory_address.as_ptr(),
			Err(_) => null_mut(),
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_alloc_zeroed(&self, layout: Layout) -> *mut u8
	{
		match self.allocate_zeroed(layout)
		{
			Ok((memory_address, _actual_size)) => memory_address.as_ptr(),
			Err(_) => null_mut(),
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

		if unlikely!(ptr == Self::ZeroSizedAllocation.as_ptr())
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

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

		match self.reallocate(NonNull::new_unchecked(ptr), layout, new_size)
		{
			Ok((memory_address, _actual_size)) => memory_address.as_ptr(),
			Err(_) => null_mut(),
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	fn AllocRef_alloc(&self, layout: Layout) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);
		if unlikely!(layout.size_ == 0)
		{
			return Ok((Self::ZeroSizedAllocation, 0))
		}
		let non_zero_size = unsafe { NonZeroUsize::new_unchecked(layout.size_) };
		self.allocate(non_zero_size, layout.align_)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn AllocRef_alloc_zeroed(&self, layout: Layout) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		self.allocate_zeroed(layout)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn AllocRef_dealloc(&self, ptr: NonNull<u8>, layout: Layout)
	{
		if unlikely!(ptr == Self::ZeroSizedAllocation)
		{
			return
		}

		let layout = LayoutHack::access_private_fields(layout);
		debug_assert_ne!(layout.size_, 0, "It should not be possible for a `layout.size_` to be zero if the `ptr` was the sentinel `Allocator::ZeroSizedAllocation`");

		let non_zero_size = NonZeroUsize::new_unchecked(layout.size_);
		self.deallocate(non_zero_size, layout.align_, ptr)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn AllocRef_realloc(&self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		self.reallocate(ptr, layout, new_size)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn AllocRef_realloc_zeroed(&self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<(NonNull<u8>, usize), AllocErr>
	{
		let (memory_address, actual_size) = self.reallocate(ptr, layout, new_size)?;

		let layout = LayoutHack::access_private_fields(layout);
		let old_size = layout.size_;
		if actual_size > old_size
		{
			memory_address.as_ptr().add(old_size).write_bytes(0x00, actual_size - old_size);
		}
		Ok((memory_address, actual_size))
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn AllocRef_grow_in_place(&self, _ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<usize, CannotReallocInPlace>
	{
		let layout = LayoutHack::access_private_fields(layout);
		let size_ = layout.size_;
		debug_assert!(new_size >= size_, "new_size `{}` is less than layout.size_ `{}`", new_size, size_);
		Err(CannotReallocInPlace)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn AllocRef_grow_in_place_zeroed(&self, ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<usize, CannotReallocInPlace>
	{
		let actual_size = self.AllocRef_grow_in_place(ptr, layout, new_size)?;

		let layout = LayoutHack::access_private_fields(layout);
		let old_size = layout.size_;
		if actual_size > old_size
		{
			ptr.as_ptr().add(old_size).write_bytes(0x00, actual_size - old_size);
		}
		Ok(actual_size)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn AllocRef_shrink_in_place(&self, _ptr: NonNull<u8>, layout: Layout, new_size: usize) -> Result<usize, CannotReallocInPlace>
	{
		let layout = LayoutHack::access_private_fields(layout);
		let size_ = layout.size_;
		debug_assert!(new_size <= size_, "layout.size_ `{}` is less than new_size `{}`", size_, new_size);
		Err(CannotReallocInPlace)
	}
}

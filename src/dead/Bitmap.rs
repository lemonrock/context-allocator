// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


// bitmap-based allocator.
// Use a bitmap to identify free bytes; if we allocate in 16 byte chunks, 16 bytes can be represented as one bit (therefore a 16:1 ratio of memory is required).
// allocations of more than one page - in theory, we can use mremap.
pub(crate) struct Bitmap
{
	pointer: MemoryAddress,
	size_in_bytes: NonZeroUsize,
	minimum_allocation_unit_power_of_two: NonZeroUsize,
	shift_to_bit: usize,
}

impl Bitmap
{
	#[inline(always)]
	pub(crate) fn new(pointer: MemoryAddress, size_in_bytes: NonZeroUsize, minimum_allocation_unit_power_of_two: NonZeroUsize) -> Self
	{
		Self
			{
				pointer,
				size_in_bytes,
				minimum_allocation_unit_power_of_two,
				shift_to_bit: minimum_allocation_unit_power_of_two.get().trailing_zeros() as usize,
			}
	}

	#[inline(always)]
	pub(crate) fn is_64_byte_block_allocated(&self, allocations_start_from: MemoryAddress, block_pointer: MemoryAddress) -> bool
	{
		let starts_at_index_bytes = block_pointer.difference(allocations_start_from);

		let starts_at_bit_index = self.shift_to_bit_index(starts_at_index_bytes);

		self.get_bit_state(starts_at_bit_index)
	}

	// TODO: Linear search of bitmap to find smallest free block that will fit a potential allocation.

	// TODO: We are currently storing data BIG ENDIAN.
	// TODO: We need to adjust allocate() to store data Intel Little Endian if we want to use intrinsics.

	// Count the number of leading zero bits in unsigned 64-bit integer a, and return that count in dst. - _lzcnt_u64
	// Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst.  - _mm256_lzcnt_epi64 - requires AVX-512.

	// TODO: _bittest64() - can be effective for reg,reg or reg,imm but not reg,mem
	// TODO: intrinsic ?_BitScanForward64 (BSF instruction)
	// _mm_countbits_64 - bits set in a u64 - BUT - WE ARE STORING DATA BIG ENDIAN!!!

	// TODO: getting really cute; if reading 64 bytes (ie 64 * 8 bits, ie 64 * 8 * self.shift_to_bit) we can prefetch into a cache line.
	// TODO:we can also issue a prefetch write cache hint in allocate.

	// TODO: We can scan byte-by-byte if we prefetch into the cache.

	#[inline(always)]
	fn get_bit_state(&self, bit_index: usize) -> bool
	{
		let bit_mask_within_byte = Self::bit_mask_within_byte(bit_index);

		let byte = self.pointer_to_byte_at(bit_index).read::<u8>();

		// NOTE: This is a place where the `_bittest64()` intrinsic on x86-64 might be suitable.
		byte & bit_mask_within_byte != 0
	}

	#[inline(always)]
	pub(crate) fn allocate(&mut self, allocations_start_from: MemoryAddress, allocated_pointer: MemoryAddress, size: NonZeroUsize)
	{
		#[cfg(debug_assertions)]
			{
				let minimum_allocation_unit_power_of_two = self.minimum_allocation_unit_power_of_two.get();

				debug_assert_eq!(allocations_start_from.to_usize() % minimum_allocation_unit_power_of_two, 0, "allocations_start_from `{:?}` is not a multiple of self.minimum_allocation_unit `{:?}`", allocations_start_from, minimum_allocation_unit_power_of_two);

				debug_assert_eq!(allocated_pointer.to_usize() % minimum_allocation_unit_power_of_two, 0, "allocated_pointer `{:?}` is not a multiple of self.minimum_allocation_unit `{:?}`", allocated_pointer, minimum_allocation_unit_power_of_two);

				let ends_at_pointer = allocated_pointer.to_usize() + size.get();
				debug_assert_eq!(ends_at_pointer % minimum_allocation_unit_power_of_two, 0, "ends_at_pointer `{:?}` is not a multiple of self.minimum_allocation_unit `{:?}`", ends_at_pointer, minimum_allocation_unit_power_of_two);
			}

		let starts_at_index_bytes = allocated_pointer.difference(allocations_start_from);

		let starts_at_bit_index = self.shift_to_bit_index(starts_at_index_bytes);
		let rounded_up_starts_at_bit_index = Self::round_up_to_power_of_64(starts_at_bit_index);

		let ends_at_bit_index = self.shift_to_bit_index(starts_at_index_bytes + size.get());
		let rounded_down_ends_at_bit_index = Self::round_down_to_power_of_64(ends_at_bit_index);

		let mut current_index = self.pointer_to_byte_at(starts_at_bit_index);
		Self::set_bits_at_start_that_are_unaligned(&mut current_index, rounded_up_starts_at_bit_index - starts_at_bit_index);
		self.set_bits_in_middle_that_are_aligned(&mut current_index, rounded_down_ends_at_bit_index);
		Self::set_bits_at_end_that_are_unaligned(&mut current_index, ends_at_bit_index - rounded_down_ends_at_bit_index);
	}

	#[inline(always)]
	fn set_bits_at_start_that_are_unaligned(current_index: &mut MemoryAddress, bits_to_set: usize)
	{
		#[inline(always)]
		fn set_bits_and_advance(current_index: &mut MemoryAddress, bits_to_set: usize, size_of_all_ones_writes: usize)
		{
			#[inline(always)]
			const fn to_upper_bits_mask(bits_to_set: usize, size_of_all_ones_writes: usize) -> u8
			{
				let upper_bits = bits_to_set - size_of_all_ones_writes;

				(((1 << upper_bits) - 1) as u8) << (8 - upper_bits)
			}
			current_index.or_u8(to_upper_bits_mask(bits_to_set, size_of_all_ones_writes));
			current_index.add_assign(1)
		}

		match bits_to_set
			{
				0 =>
					{
						current_index.add_assign(size_of::<u64>())
					}

				1 ... 7 =>
					{
						current_index.add_assign(size_of::<u32>() + size_of::<u16>() + size_of::<u8>());
						set_bits_and_advance(current_index, bits_to_set, 0)
					}

				8 =>
					{
						current_index.add_assign(size_of::<u32>() + size_of::<u16>() + size_of::<u8>());
						current_index.write_and_advance(0xFFu8)
					}

				9 ... 15 =>
					{
						current_index.add_assign(size_of::<u32>() + size_of::<u16>());
						set_bits_and_advance(current_index, bits_to_set, size_of::<u8>());
						current_index.write_and_advance(0xFFu8)
					}

				16 =>
					{
						current_index.add_assign(size_of::<u32>() + size_of::<u16>());
						current_index.write_and_advance(0xFFFFu16)
					}

				17 ... 23 =>
					{
						current_index.add_assign(size_of::<u32>() + size_of::<u8>());
						set_bits_and_advance(current_index, bits_to_set, size_of::<u16>());
						current_index.write_and_advance(0xFFFFu16)
					}

				24 =>
					{
						current_index.add_assign(size_of::<u32>() + size_of::<u8>());
						current_index.write_and_advance(0xFFu8);
						current_index.write_and_advance(0xFFFFu16)
					}

				25 ... 31 =>
					{
						current_index.add_assign(size_of::<u32>());
						set_bits_and_advance(current_index, bits_to_set, size_of::<u16>() + size_of::<u8>());
						current_index.write_and_advance(0xFFu8);
						current_index.write_and_advance(0xFFFFu16)
					}

				32 =>
					{
						current_index.add_assign(size_of::<u32>());
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				33 ... 39 =>
					{
						current_index.add_assign(size_of::<u16>() + size_of::<u8>());
						set_bits_and_advance(current_index, bits_to_set, size_of::<u32>());
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				40 =>
					{
						current_index.add_assign(size_of::<u16>() + size_of::<u8>());
						current_index.write_and_advance(0xFFu8);
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				41 ... 47 =>
					{
						current_index.add_assign(size_of::<u16>());
						set_bits_and_advance(current_index, bits_to_set, size_of::<u8>() + size_of::<u32>());
						current_index.write_and_advance(0xFFu8);
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				48 =>
					{
						current_index.add_assign(size_of::<u16>());
						current_index.write_and_advance(0xFFFFu16);
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				49 ... 55 =>
					{
						current_index.add_assign(size_of::<u8>());
						set_bits_and_advance(current_index, bits_to_set, size_of::<u16>() + size_of::<u32>());
						current_index.write_and_advance(0xFFFFu16);
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				56 =>
					{
						current_index.add_assign(size_of::<u8>());
						current_index.write_and_advance(0xFFFFu16);
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				57 ... 63 =>
					{
						set_bits_and_advance(current_index, bits_to_set, size_of::<u8>() + size_of::<u16>() + size_of::<u32>());
						current_index.write_and_advance(0xFFu8);
						current_index.write_and_advance(0xFFFFu16);
						current_index.write_and_advance(0xFFFF_FFFFu32)
					}

				_ => unreachable!(),
			}
	}

	#[inline(always)]
	fn set_bits_in_middle_that_are_aligned(&self, current_index: &mut MemoryAddress, rounded_down_ends_at_bit_index: usize)
	{
		debug_assert_eq!(rounded_down_ends_at_bit_index % Self::power_of_two, 0, "rounded_down_ends_at_bit_index `{}` is not a power of `{}`", rounded_down_ends_at_bit_index, Self::power_of_two);

		let ends_at = self.pointer_to_byte_at(rounded_down_ends_at_bit_index);

		while current_index != &ends_at
			{
				current_index.write_and_advance(0xFFFF_FFFF_FFFF_FFFFu64);
			}
	}

	#[inline(always)]
	fn set_bits_at_end_that_are_unaligned(current_index: &mut MemoryAddress, bits_to_set: usize)
	{
		#[inline(always)]
		fn set_bits(current_index: &mut MemoryAddress, bits_to_set: usize, size_of_all_ones_writes: usize)
		{
			#[inline(always)]
			const fn to_lower_bits_mask(bits_to_set: usize, size_of_all_ones_writes: usize) -> u8
			{
				let lower_bits = bits_to_set - size_of_all_ones_writes;

				((1 << lower_bits) - 1) as u8
			}

			(*current_index).or_u8(to_lower_bits_mask(bits_to_set, size_of_all_ones_writes));
		}

		match bits_to_set
			{
				0 => (),

				1 ... 7 =>
					{
						set_bits(current_index, bits_to_set, 0)
					}

				8 =>
					{
						current_index.write(0xFFu8);
					}

				9 ... 15 =>
					{
						current_index.write_and_advance(0xFFu8);
						set_bits(current_index, bits_to_set, size_of::<u8>())
					}

				16 =>
					{
						current_index.write(0xFFFFu16);
					}

				17 ... 23 =>
					{
						current_index.write_and_advance(0xFFFFu16);
						set_bits(current_index, bits_to_set, size_of::<u16>())
					}

				24 =>
					{
						current_index.write_and_advance(0xFFFFu16);
						current_index.write(0xFFu8)
					}

				25 ... 31 =>
					{
						current_index.write_and_advance(0xFFFFu16);
						current_index.write_and_advance(0xFFu8);
						set_bits(current_index, bits_to_set, size_of::<u16>() + size_of::<u8>())
					}

				32 =>
					{
						current_index.write(0xFFFF_FFFFu32);
					}

				33 ... 39 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						set_bits(current_index, bits_to_set, size_of::<u32>())
					}

				40 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						current_index.write(0xFFu8)
					}

				41 ... 47 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						current_index.write_and_advance(0xFFu8);
						set_bits(current_index, bits_to_set, size_of::<u32>() + size_of::<u8>())
					}

				48 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						current_index.write(0xFFFFu16)
					}

				49 ... 55 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						current_index.write_and_advance(0xFFFFu16);
						set_bits(current_index, bits_to_set, size_of::<u32>() + size_of::<u16>())
					}

				56 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						current_index.write_and_advance(0xFFFFu16);
						current_index.write(0xFFu8)
					}

				57 ... 63 =>
					{
						current_index.write_and_advance(0xFFFF_FFFFu32);
						current_index.write_and_advance(0xFFFFu16);
						current_index.write_and_advance(0xFFu8);
						set_bits(current_index, bits_to_set, size_of::<u32>() + size_of::<u16>() + size_of::<u8>())
					}

				_ => unreachable!(),
			}
	}

	const power_of_two: usize = 64;

	const power_of_two_less_one: usize = Self::power_of_two - 1;

	#[inline(always)]
	const fn round_up_to_power_of_64(value: usize) -> usize
	{
		(value + Self::power_of_two_less_one) & !Self::power_of_two_less_one
	}

	#[inline(always)]
	const fn round_down_to_power_of_64(value: usize) -> usize
	{
		value & !Self::power_of_two_less_one
	}

	const BitsInU8Mask: usize = 0b0111;

	#[inline(always)]
	const fn byte_index(bit_index: usize) -> usize
	{
		bit_index & !Self::BitsInU8Mask
	}

	#[inline(always)]
	const fn bit_mask_within_byte(bit_index: usize) -> u8
	{
		(1 << (bit_index & Self::BitsInU8Mask)) as u8
	}

	#[inline(always)]
	fn pointer_to_byte_at(&self, bit_index: usize) -> MemoryAddress
	{
		let byte_index = Self::byte_index(bit_index);
		self.pointer.add(byte_index)
	}

	#[inline(always)]
	fn shift_to_bit_index(&self, byte_offset: usize) -> usize
	{
		byte_offset >> self.shift_to_bit
	}
}
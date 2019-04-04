// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Request that an allocation uses huge pages.
///
/// The allocation being requested has to be aligned to the size requested, viz it is best to only make allocations that use the configured huge page size.
///
/// On x86-64, this should be `2Mb` or `1Gb`.
///
/// Currently such requests assume that transparent huge pages are in effect.
///
/// On operating systems other than Android and Linux, huge page size has no effect.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum HugePageSize
{
	/// Regular, non-huge-page.
	None = 0,

	/// Equivalent to `MAP_HUGETLB`.
	#[cfg(any(target_os = "android", target_os = "linux"))] Default = MAP_HUGETLB | 0 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] Default = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_64KB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _64Kb = MAP_HUGETLB | 16 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _64Kb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_512KB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _512Kb = MAP_HUGETLB | 19 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _512Kb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_1Mb`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _1Mb = MAP_HUGETLB | 20 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _1Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_2MB`.
	///
	/// Suitable for x86-64.
	#[cfg(any(target_os = "android", target_os = "linux"))] _2Mb = MAP_HUGETLB | 21 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _2Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_8MB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _8Mb = MAP_HUGETLB | 23 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _8Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_16MB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _16Mb = MAP_HUGETLB | 24 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _16Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_32MB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _32Mb = MAP_HUGETLB | 25 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _32Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_256MB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _256Mb = MAP_HUGETLB | 28 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _256Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_512MB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _512Mb = MAP_HUGETLB | 29 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _512Mb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_1GB`.
	///
	/// Suitable for x86-64.
	#[cfg(any(target_os = "android", target_os = "linux"))] _1Gb = MAP_HUGETLB | 30 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _1Gb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_2GB`.
	///
	/// Suitable for ?
	#[cfg(any(target_os = "android", target_os = "linux"))] _2Gb = MAP_HUGETLB | 31 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _2Gb = 0,

	/// Equivalent to `MAP_HUGETLB | MAP_HUGE_16GB`.
	///
	/// Suitable for PowerPC.
	#[cfg(any(target_os = "android", target_os = "linux"))] _16Gb = MAP_HUGETLB | 34 << Self::MAP_HUGE_SHIFT,
	#[cfg(not(any(target_os = "android", target_os = "linux")))] _16Gb = 0,
}

impl Default for HugePageSize
{
	#[inline(always)]
	fn default() -> Self
	{
		HugePageSize::None
	}
}

impl HugePageSize
{
	#[cfg(any(target_os = "android", target_os = "linux"))] const MAP_HUGE_SHIFT: i32 = 26;
}

# context-allocator

This provides allocators suitable for a number of use cases.

All of these allocators implement the traits `std::alloc::GlobalAlloc` and `std::alloc::Allocator`, as we as a common base trait, `Allocator`.

The most useful is a global allocator which allows switching between thread, coroutine and global (and thuse lockable) memory allocators, using the macro `global_thread_and_coroutine_switchable_allocator()`.

Allocators provided include:-

* `BumpAllocator`, a never-freeing bump allocator with slight optimization for reallocating the last allocation.
* `BitSetAllocator`, an allocator that uses a bit set of free blocks; uses 64-bit chunks to optimize searches.
* `MultipleBinarySearchTreeAllocator`, an efficient allocator which minimizes fragmentation by using multiple red-black trees of free blocks which are aggresively defragmented.
* `ContextAllocator`, a choice of either `BumpAllocator`, `BitSetAllocator` or `MultipleBinarySearchTreeAllocator`.
* `MemoryMapAllocator`, a NUMA-aware mmap allocator with support for NUMA policies.
* `GlobalThreadAndCoroutineSwitchableAllocator`, suitable for replacing the global allocator and provides switchable allocators for global, thread local and context (coroutine) local needs; must b created using the macro `global_thread_and_coroutine_switchable_allocator`.

Allocators use a `MemorySource` to obtain and release memory.
Memory sources provided include:-

* `MemoryMapSource`, useful for thread-local allocators as it can obtain memory from NUMA-local memory.
* `ArenaMemorySource`, an arena of fixed blocks which is itself backed by a memory source; this is useful as a source for the `BumpAllocator` and `BitSetAllocator` when used for contexts.

Additionally a number of adaptors are provided:-

* `AllocatorAdaptor`, an adaptor of `Allocator` to `GlobalAlloc` and `Alloc`; use it by calling `Allocator.adapt()`
* `GlobalAllocToAllocatorAdaptor`, an adaptor of `GlobalAlloc` to `Allocator`, useful for assigning a global allocator to `GlobalThreadAndCoroutineSwitchableAllocator`.
* `AllocToAllocatorAdaptor`, an adaptor of `Alloc` to `Allocator`.

When using `GlobalThreadAndCoroutineSwitchableAllocator`, it is possible to save and restore the allocator state for the currently running context (coroutine).
It is also possible to create a lockless, fast thread-local allocator which make use of NUMA memory, unlike a conventional malloc.


## Future

* Investigate wrapping [Rampant Pixel's Memory Allocator](https://github.com/rampantpixels/rpmalloc).
* Investigate using DPDK's allocator.
* Investigate a B-tree backed allocator.
* Investigate a design that uses multiple doubly-linked 'free' lists of blocks; blocks can be variable in size but the free list is sorted
	* Iteration over a particular free-list range may encountered blocks too small, or blocks so large they can be split up.
	* This design is similar to that used by DPDK.
	* To make the allocator multi-threaded, DPDK takes a spin lock on a particular 'heap', which is a set of free lists.
* Investigate a fall-back over-size allocator for a thread-local allocator, which could use the `NumaMemoryMapSource` underneath.
* Investigate supporting over-size allocations in `MultipleBinarySearchTreeAllocator` by scanning the largest binary search tree for contiguous blocks.
* Investigate a persistent-memory backed allocator.
* Properly support excess allocations and Alloc's grow_in_place functions, but only if these are used by downstream collections.
* Investigate the use of the `BMI1` intrinsics `_blsi_u64` (extract lowest set bit), `_blsmsk_u64` and `_blsr_u64`.


## Licensing

The license for this project is MIT.

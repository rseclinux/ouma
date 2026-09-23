use scudo::GlobalScudoAllocator;

#[global_allocator]
static SCUDO_ALLOCATOR: GlobalScudoAllocator = GlobalScudoAllocator;

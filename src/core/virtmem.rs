// Virtual memory information.
struct Info {
    // Virtual memory page size.
    pub page_size: u32,
    // Virtual memory page granularity.
    pub page_granularity: u32,
}

// Returns virtual memory information, see `VirtMem::Info` for more details.
pub fn info() -> Info {
    // static VirtMem::Info vmInfo;
    // static std::atomic<uint32_t> vmInfoInitialized;
  
    // if (!vmInfoInitialized.load()) {
    //   VirtMem::Info localMemInfo;
    //   VirtMem_getInfo(localMemInfo);
  
    //   vmInfo = localMemInfo;
    //   vmInfoInitialized.store(1u);
    // }
  
    // return vmInfo;
    unimplemented!();
}
#[derive(Debug, Clone)]
pub enum KvmMemFlag {
  KvmMemLogDirtyPages = 0x1 << 0,
  KvmMemReadonly = 0x1 << 1,
}

impl KvmMemFlag {
  pub fn value(self) -> u32 {
    self as u32
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct KvmUserspaceMemoryRegion {
  slot: u32, // 0-15: slot id, 16-31: address space
  flags: u32,
  guest_phys_addr: u32, // [WARNING] lower 21bits must be identical with that of `userspace_addr`
  memory_size: u64,     // (bytes)
  userspace_addr: u64,
}

impl KvmUserspaceMemoryRegion {
  pub fn new(
    slot: u16,
    addr_space: u16,
    flags: Vec<KvmMemFlag>,
    guest_phys_addr: u32,
    memory_size: u64,
    userspace_addr: u64,
  ) -> Self {
    let slot = ((addr_space as u32) << 16) | slot as u32;
    let flags = flags
      .into_iter()
      .fold(0, |acc: u32, flag| (acc as u64 | flag as u64) as u32);
    Self {
      slot,
      flags,
      guest_phys_addr,
      memory_size,
      userspace_addr,
    }
  }
}

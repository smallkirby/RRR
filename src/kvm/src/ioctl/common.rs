use errno;
use libc::{open, O_CLOEXEC, O_RDWR};

use crate::{binding::structs::KvmUserspaceMemoryRegion, util::mmap_annon};

use super::{
  layout,
  misc::get_api_version,
  vm::{create_vm, set_tss_addr, set_user_memory_region},
  IoctlError,
};

const KVM_PATH: &str = "/dev/kvm\0";

pub struct Kvm {
  fd: i32,
  vmfd: Option<i32>,
}

impl Kvm {
  pub fn new() -> Result<Self, IoctlError> {
    let fd = unsafe { open(KVM_PATH.as_ptr() as *const i8, O_RDWR | O_CLOEXEC) };
    if fd < 0 {
      Err(IoctlError::KvmOpenFailed(errno::errno()))
    } else {
      Ok(Self { fd, vmfd: None })
    }
  }

  pub fn get_api_version(&self) -> Result<i32, IoctlError> {
    get_api_version(self.fd)
  }

  pub fn init_vm(&mut self) -> Result<(), IoctlError> {
    self.create_vm()?;
    self.set_tss_addr()?;
    self.set_user_memory_region()?;
    Ok(())
  }

  fn create_vm(&mut self) -> Result<(), IoctlError> {
    let vmfd = create_vm(self.fd)?;
    self.vmfd = Some(vmfd);
    Ok(())
  }

  fn set_tss_addr(&self) -> Result<(), IoctlError> {
    set_tss_addr(self.vmfd.unwrap(), layout::KVM_TSS_ADDR)?;
    Ok(())
  }

  fn set_user_memory_region(&self) -> Result<(), IoctlError> {
    let size = n_gib_bytes!(4);
    let mem = mmap_annon(size as usize).unwrap();
    let umemory_region = KvmUserspaceMemoryRegion::new(0, 0, vec![], 0, size as u64, mem as u64); // TODO
    set_user_memory_region(self.vmfd.unwrap(), &umemory_region)
  }
}

#[cfg(test)]
mod test {
  use super::super::misc::VALID_API_VERSION;
  use super::*;

  #[test]
  pub fn test_open_kvm() {
    Kvm::new().unwrap();
  }

  #[test]
  pub fn test_api_version() {
    let kvm = Kvm::new().unwrap();
    assert_eq!(kvm.get_api_version().unwrap(), VALID_API_VERSION);
  }
}

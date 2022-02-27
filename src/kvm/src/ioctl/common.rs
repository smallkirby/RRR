use errno;
use libc::{open, O_CLOEXEC, O_RDWR};

use super::{misc::get_api_version, IoctlError};

const KVM_PATH: &str = "/dev/kvm\0";

pub struct Kvm {
  fd: i32,
}

impl Kvm {
  pub fn new() -> Result<Self, IoctlError> {
    let fd = unsafe { open(KVM_PATH.as_ptr() as *const i8, O_RDWR | O_CLOEXEC) };
    if fd < 0 {
      Err(IoctlError::KvmOpenFailed(errno::errno()))
    } else {
      Ok(Self { fd })
    }
  }

  pub fn get_api_version(&self) -> Result<i32, IoctlError> {
    get_api_version(self.fd)
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

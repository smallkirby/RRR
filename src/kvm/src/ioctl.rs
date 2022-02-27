pub mod common;
pub mod misc;
pub mod vm;

use thiserror::Error;

use crate::binding::commands::KvmCommands;

/*** KVM ioctl error definition ***/

#[derive(Error, Debug)]
pub enum IoctlError {
  #[error("failed to open KVM device file.")]
  KvmOpenFailed(#[from] errno::Errno),
  #[error("ioctl({:?}) returned error: {:?}", ioctl_cmd, errno)]
  IoctlResultError {
    errno: errno::Errno,
    ioctl_cmd: KvmCommands,
  },
}

pub mod layout {
  // TSS region must be within first 4GB, and consumes 3 pages.
  pub const KVM_TSS_ADDR: u64 = (n_gib_bytes!(4) - n_kib_bytes!(4) * 3) as u64;
}

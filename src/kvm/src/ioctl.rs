pub mod common;
pub mod misc;

use thiserror::Error;

/*** KVM ioctl error definition ***/

#[derive(Error, Debug)]
pub enum IoctlError {
  #[error("failed to open KVM device file.")]
  KvmOpenFailed(#[from] errno::Errno),
  #[error("ioctl returned error.")]
  IoctlResultError { errno: errno::Errno },
}

/*** KVM ioctl commands definition ***/

const _IOC_NRSHIFT: u32 = 0;
const _IOC_NRBITS: u32 = 8;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
const _IOC_NONE: u32 = 0;
const KVMIO: u32 = 0xAE;

macro_rules! _IOC {
  ( $dir: expr, $typ: expr, $nr: expr, $size:expr ) => {
    (($dir << $crate::ioctl::_IOC_DIRSHIFT)
      | (($typ) << $crate::ioctl::_IOC_TYPESHIFT)
      | (($nr) << $crate::ioctl::_IOC_NRSHIFT)
      | (($size) << $crate::ioctl::_IOC_SIZESHIFT)) as u64
  };
}
macro_rules! _IO {
  ($typ: expr, $nr: expr) => {
    _IOC!($crate::ioctl::_IOC_NONE, $typ, $nr, 0)
  };
}
macro_rules! _KVMIO {
  ($nr: expr) => {
    _IO!($crate::ioctl::KVMIO, $nr)
  };
}

pub mod commands {
  pub const KVM_GET_API_VERSION: u64 = _KVMIO!(0);
}

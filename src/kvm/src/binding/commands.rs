use crate::binding::structs::KvmUserspaceMemoryRegion;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_NRBITS: u32 = 8;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const KVMIO: u32 = 0xAE;

macro_rules! _IOC {
  ( $dir: expr, $typ: expr, $nr: expr, $size:expr ) => {
    (($dir << $crate::binding::commands::_IOC_DIRSHIFT)
      | (($typ) << $crate::binding::commands::_IOC_TYPESHIFT)
      | (($nr) << $crate::binding::commands::_IOC_NRSHIFT)
      | (($size) << $crate::binding::commands::_IOC_SIZESHIFT) as u32) as isize
  };
}
macro_rules! _IOC_TYPECHECK {
  ($size: ty) => {
    ::std::mem::size_of::<$size>()
  };
}
macro_rules! _IO {
  ($typ: expr, $nr: expr) => {
    _IOC!($crate::binding::commands::_IOC_NONE, $typ, $nr, 0)
  };
}
macro_rules! _IOW {
  ($typ: expr, $nr: expr, $size: ty) => {
    _IOC!(
      $crate::binding::commands::_IOC_WRITE,
      $typ,
      $nr,
      _IOC_TYPECHECK!($size)
    )
  };
}
macro_rules! _KVMIO {
  ($nr: expr) => {
    _IO!($crate::binding::commands::KVMIO, $nr)
  };
  ($nr: expr, $struct: ty) => {
    _IOW!($crate::binding::commands::KVMIO, $nr, $struct)
  };
}

#[derive(Debug)]
pub enum KvmCommands {
  KvmGetApiVersion = _KVMIO!(0),
  KvmCreateVm = _KVMIO!(1),
  KvmSetUserMemoryRegion = _KVMIO!(0x46, KvmUserspaceMemoryRegion),
  KvmSetTssAddr = _KVMIO!(0x47),
}

#[allow(clippy::from_over_into)]
impl Into<u64> for KvmCommands {
  fn into(self) -> u64 {
    self as u64
  }
}

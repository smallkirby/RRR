use libc::c_void;

use super::IoctlError;
use crate::binding::structs::KvmUserspaceMemoryRegion;
use crate::ioctl::KvmCommands;

/*
  KVM_CREATE_VM

  Capability: basic
  Architectures: all
  Type: system ioctl
  Parameters: machine type identifier (KVM_VM_*)
  Returns: a VM fd that can be used to control the new virtual machine.
*/
pub fn create_vm(kvmfd: i32) -> Result<i32, IoctlError> {
  let result: i32 = unsafe { libc::ioctl(kvmfd, KvmCommands::KvmCreateVm.into(), 0) };
  if result < 0 {
    Err(IoctlError::IoctlResultError {
      errno: errno::errno(),
      ioctl_cmd: KvmCommands::KvmCreateVm,
    })
  } else {
    Ok(result)
  }
}

/*
  KVM_SET_TSS_ADDR

  Capability: KVM_CAP_SET_TSS_ADDR
  Architectures: x86
  Type: vm ioctl
  Parameters: unsigned long tss_address (in)
  Returns: 0 on success, -1 on error
*/
// set 3pages region's address for TSS(Task Switch Segment)
pub fn set_tss_addr(vmfd: i32, tss_addr: u64) -> Result<(), IoctlError> {
  let result: i32 = unsafe { libc::ioctl(vmfd, KvmCommands::KvmSetTssAddr.into(), tss_addr) };
  if result < 0 {
    Err(IoctlError::IoctlResultError {
      errno: errno::errno(),
      ioctl_cmd: KvmCommands::KvmSetTssAddr,
    })
  } else {
    Ok(())
  }
}

/*
  KVM_SET_USER_MEMORY_REGION

  Capability: KVM_CAP_USER_MEMORY
  Architectures: all
  Type: vm ioctl
  Parameters: struct kvm_userspace_memory_region (in)
  Returns: 0 on success, -1 on error
*/
pub fn set_user_memory_region(
  vmfd: i32,
  umemory_region: &KvmUserspaceMemoryRegion,
) -> Result<(), IoctlError> {
  let result: i32 = unsafe {
    libc::ioctl(
      vmfd,
      KvmCommands::KvmSetUserMemoryRegion.into(),
      umemory_region as *const KvmUserspaceMemoryRegion as *const c_void,
    )
  };
  if result < 0 {
    Err(IoctlError::IoctlResultError {
      errno: errno::errno(),
      ioctl_cmd: KvmCommands::KvmSetUserMemoryRegion,
    })
  } else {
    Ok(())
  }
}

use super::IoctlError;
use crate::binding::commands::KvmCommands;

pub const VALID_API_VERSION: i32 = 12;

/*
  KVM_GET_API_VERSION

  Capability: basic
  Architectures: all
  Type: system ioctl
  Parameters: none
  Returns: the constant KVM_API_VERSION (=12)
*/
pub fn get_api_version(kvmfd: i32) -> Result<i32, IoctlError> {
  let result: i32 = unsafe { libc::ioctl(kvmfd, KvmCommands::KvmGetApiVersion.into(), 0) };
  if result < 0 {
    Err(IoctlError::IoctlResultError {
      errno: errno::errno(),
      ioctl_cmd: KvmCommands::KvmGetApiVersion,
    })
  } else {
    Ok(result)
  }
}

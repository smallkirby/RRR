use super::commands::KVM_GET_API_VERSION;
use super::IoctlError;

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
  let result: i32 = unsafe { libc::ioctl(kvmfd, KVM_GET_API_VERSION, 0) };
  if result < 0 {
    Err(IoctlError::IoctlResultError {
      errno: errno::errno(),
    })
  } else {
    Ok(result)
  }
}

use libc::{
  self, c_void, MAP_ANONYMOUS, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE,
};
use std::ptr::null_mut;

pub fn mmap(addr: Option<*mut u8>, size: usize) -> Result<*mut u8, ()> {
  let ma = match addr {
    Some(addr) => unsafe {
      libc::mmap(
        addr as *mut c_void,
        size,
        PROT_WRITE | PROT_READ | PROT_EXEC,
        MAP_PRIVATE | MAP_FIXED | MAP_ANONYMOUS,
        0,
        0,
      )
    },
    None => unsafe {
      libc::mmap(
        null_mut(),
        size,
        PROT_WRITE | PROT_READ | PROT_EXEC,
        MAP_PRIVATE | MAP_ANONYMOUS,
        0,
        0,
      )
    },
  };
  if ma == MAP_FAILED {
    Err(())
  } else {
    Ok(ma as *mut u8)
  }
}

pub fn mmap_annon(size: usize) -> Result<*mut u8, ()> {
  mmap(None, size)
}

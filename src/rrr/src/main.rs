use kvm::ioctl::{common::Kvm, misc::VALID_API_VERSION};

use std::process::exit;

fn main() {
  let kvm = match Kvm::new() {
    Ok(kvm) => kvm,
    Err(e) => {
      eprintln!("{e}");
      exit(1);
    }
  };

  let api_version = match kvm.get_api_version() {
    Ok(version) => version,
    Err(e) => {
      eprintln!("{e}");
      exit(1);
    }
  };
  if api_version != VALID_API_VERSION {
    eprintln!("Invalid KVM API version returned: ${api_version}");
    exit(1);
  }

  println!("success");
}

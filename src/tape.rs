
pub mod tape {
  use nix::libc::{c_long, c_int, c_char};
  use nix::ioctl_read;
  use std::ffi::CString;

  const MTIOCGET_MAGIC: u8 = b'm';
  const MTIOCGET_TYPE_MODE: u8 = 2;

  #[derive(Debug, Default)]
  pub struct Mtget {
    mt_type: libc::c_long, // type of tape device
    mt_resid: c_long, // residual count -- number of bytes ignored/files/records not skipped

    mt_dsreg: c_long, // status register
    mt_gstat: c_long, // generic status
    mt_erreg: c_long, // error register

    // sometimes used fields
    mt_fileno: c_int, // number of current file on tape
    mt_blkno: c_int // current block number
  }

  // generates function like
  // pub unsafe fn mtiocget(fd: c_int, data: *mut mtget) -> Result<c_int>
  ioctl_read!(mtiocget, MTIOCGET_MAGIC, MTIOCGET_TYPE_MODE, Mtget);

  // #define	MTIOCGET	_IOR('m', 2, struct mtget)	
  pub fn status(dev: &str) -> i32 {
    let devstr = str_to_c_char(dev);
    let mut tape_status = Mtget::default();
    unsafe {
      let fd = libc::openat(libc::AT_FDCWD, devstr, libc::O_RDONLY);
      match mtiocget(fd, &mut tape_status) {
        Err(_why) => return 2,
        Ok(result) => result,
      };
    }

    println!("{:?}", tape_status);
    return 0;

  }

  fn str_to_c_char(input: &str) -> *const c_char {
    let value = CString::new(input).unwrap();
    value.as_ptr()
  }
}

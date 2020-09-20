pub mod tape {
  use nix::ioctl_read;
  use nix::libc::{c_char, c_int, c_long};
  use std::ffi::CString;

  const MTIOCGET_MAGIC: u8 = b'm';
  const MTIOCGET_TYPE_MODE: u8 = 2;

  #[repr(C)]
  #[derive(Debug, Default)]
  pub struct Mtget {
    mt_type: libc::c_long, // type of tape device
    mt_resid: c_long,      // residual count -- number of bytes ignored/files/records not skipped

    mt_dsreg: c_long, // status register
    mt_gstat: c_long, // generic status
    mt_erreg: c_long, // error register

    // sometimes used fields
    mt_fileno: c_int, // number of current file on tape
    mt_blkno: c_int,  // current block number
  }

  const MTISUNKNOWN: i64 = 0x01;
  const MTISQIC02: i64 = 0x02; /* Generic QIC-02 tape streamer */
  const MTISWT5150: i64 = 0x03; /* Wangtek 5150EQ; QIC-150; QIC-02 */
  const MTISARCHIVE5945L2: i64 = 0x04; /* Archive 5945L-2; QIC-24; QIC-02? */
  const MTISCMSJ500: i64 = 0x05; /* CMS Jumbo 500 (QIC-02?) */
  const MTISTDC3610: i64 = 0x06; /* Tandberg 6310; QIC-24 */
  const MTISARCHIVEVP60I: i64 = 0x07; /* Archive VP60i; QIC-02 */
  const MTISARCHIVE2150L: i64 = 0x08; /* Archive Viper 2150L */
  const MTISARCHIVE2060L: i64 = 0x09; /* Archive Viper 2060L */
  const MTISARCHIVESC499: i64 = 0x0A; /* Archive SC-499 QIC-36 controller */
  const MTISQIC02ALLFEATURES: i64 = 0x0F; /* Generic QIC-02 with all features */
  const MTISWT5099EEN24: i64 = 0x11; /* Wangtek 5099-een24; 60MB; QIC-24 */
  const MTISTEACMT2ST: i64 = 0x12; /* Teac MT-2ST 155mb drive; Teac DC-1 card (Wangtek type) */
  const MTISEVEREXFT40A: i64 = 0x32; /* Everex FT40A (QIC-40) */
  const MTISDDS1: i64 = 0x51; /* DDS device without partitions */
  const MTISDDS2: i64 = 0x52; /* DDS device with partitions */
  const MTISONSTREAMSC: i64 = 0x61; /* OnStream SCSI tape drives (SC-x0) and SCSI emulated (DI; DP; USB) */
  const MTISSCSI1: i64 = 0x71; /* Generic ANSI SCSI-1 tape unit */
  const MTISSCSI2: i64 = 0x72; /* Generic ANSI SCSI-2 tape unit */

  // generates function like
  // pub unsafe fn mtiocget(fd: c_int, data: *mut mtget) -> Result<c_int>
  ioctl_read!(mtiocget, MTIOCGET_MAGIC, MTIOCGET_TYPE_MODE, Mtget);

  // #define	MTIOCGET	_IOR('m', 2, struct mtget)
  pub fn status(dev: &str) -> i32 {
    let mut tape_status = Mtget::default();
    let devstr = CString::new(dev).unwrap();

    unsafe {
      let fd = libc::openat(libc::AT_FDCWD, devstr.as_ptr(), libc::O_RDONLY);

      if fd < 0 {
        println!("device not found: {} as c_char {:#?}", dev, devstr);
        return 2;
      }
      match mtiocget(fd, &mut tape_status) {
        Err(_why) => return 2,
        Ok(result) => result,
      };
    }

    // make sense of the data
    let tape_type = match tape_status.mt_type {
      MTISUNKNOWN => "Unknown",
      MTISQIC02 => "Generic QIC-02 tape streamer",
      MTISSCSI1 | MTISSCSI2 => "Generic ANSI SCSI tape unit",
      _ => "Still unknown",
    };

    // Mtget { mt_type: 114, mt_resid: 0, mt_dsreg: 1476657152,
    // mt_gstat: 16842752, mt_erreg: 0, mt_fileno: -1, mt_blkno: -1 }
    println!("{:?}", tape_status);
    println!("Tape type: {}", tape_type);
    return 0;
  }
}

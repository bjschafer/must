pub mod tape {
  use nix::{ioctl_read, ioctl_write_ptr};
  use nix::libc::{c_int, c_long, c_short};
  use std::ffi::CString;

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

  const MTRESET: c_short = 0; // reset drive
  const MTFSF: c_short = 1; // fastforward, position at fist record of next file
  const MTBSF: c_short = 2; // back space file, position before file mark
  const MTFSR: c_short = 3; // fastforward space record
  const MTBSR: c_short = 4; // back space record
  const MTWEOF: c_short = 5; // write end of file (or flush)
  const MTREW: c_short = 6; // rewind
  const MTTELL: c_short = 23; // tell block

  const MTIOCTOP_MAGIC: u8 = b'm';
  const MTIOCTOP_TYPE_MODE: u8 = 1;

  const MTIOCGET_MAGIC: u8 = b'm';
  const MTIOCGET_TYPE_MODE: u8 = 2;

  const MTIOCPOS_MAGIC: u8 = b'm';
  const MTIOCPOS_TYPE_MODE: u8 = 3;

  #[repr(C)]
  #[derive(Debug, Default)]
  pub struct Mtop {
    mt_op: c_short,
    mt_count: c_int,
  }

  #[repr(C)]
  #[derive(Debug, Default)]
  pub struct Mtget {
    mt_type: c_long, // type of tape device
    mt_resid: c_long,      // residual count -- number of bytes ignored/files/records not skipped

    mt_dsreg: c_long, // status register
    mt_gstat: c_long, // generic status
    mt_erreg: c_long, // error register

    // sometimes used fields
    mt_fileno: c_int, // number of current file on tape
    mt_blkno: c_int,  // current block number
  }

  #[repr(C)]
  #[derive(Debug, Default)]
  pub struct Mtpos {
    mt_blkno: c_long // current block number
  }

  struct FileDescriptor {
    fd: c_int,
  }

  impl Drop for FileDescriptor {
    fn drop (&mut self) {
      unsafe {
        if self.fd != -1 {
          libc::close(self.fd);
        }
      }
    }
  }


  // generates function like
  // pub unsafe fn mtiocget(fd: c_int, data: *mut mtget) -> Result<c_int>
  ioctl_write_ptr!(mtioctop, MTIOCTOP_MAGIC, MTIOCTOP_TYPE_MODE, Mtop);
  ioctl_read!(mtiocget, MTIOCGET_MAGIC, MTIOCGET_TYPE_MODE, Mtget);
  ioctl_read!(mtiocpos, MTIOCPOS_MAGIC, MTIOCPOS_TYPE_MODE, Mtpos);

  fn do_mtioctop(dev: &str, cmd: &mut Mtop) -> i32 {
    let fd = get_filedescriptor(dev).unwrap();

    unsafe {
      match mtioctop(fd.fd, cmd) {
        Err(_why) => -2,
        Ok(result) => result,
      }
    }
  }

  pub fn rewind(dev: &str) -> i32 {
    let mut tape_operation = Mtop {
      mt_op: MTREW,
      mt_count: 1,
    };

    do_mtioctop(dev, &mut tape_operation)
  }

  pub fn reset(dev: &str) -> i32 {
    let mut tape_operation = Mtop {
      mt_op: MTRESET,
      mt_count: 1
    };

    do_mtioctop(dev, &mut tape_operation)
  }

  /// `flush` calls `write_eof` with a zero count
  /// to ensure data is flushed to disk
  pub fn flush(dev: &str) -> i32 {
    write_eof(dev, 0)
  }

  /// `write_eof` has the peculiar property that, if given
  /// a zero count, it will ask the tape driver to flush
  /// all data to tape. Thus, it's often a good idea
  /// to call when wrapping up tape operations.
  pub fn write_eof(dev: &str, count: i32) -> i32 {
    let mut tape_operation = Mtop {
      mt_op: MTWEOF,
      mt_count: count,
    };

    do_mtioctop(dev, &mut tape_operation)
  }

  // From https://docs.oracle.com/cd/E19455-01/817-5430/6mksu57hg/index.html:
  //
  // When spacing forward over a record (either data or EOF), the tape head is
  // positioned in the tape gap between the record just skipped and the next record.
  //
  // When spacing forward over file marks (EOF records), the tape head is positioned
  // in the tape gap between the next EOF record and the record that follows it.
  //
  //
  // When spacing backward over a record (either data or EOF), the tape head is positioned
  // in the tape gap immediately preceding the tape record where the tape head is currently
  // positioned.

  // When spacing backward over file marks (EOF records), the tape head is
  // positioned in the tape gap preceding the EOF. Thus the next read would fetch the EOF.
  //
  //
  // Record skipping does not go past a file mark; file skipping does not go past the EOM
  //
  // After an MTFSR <huge number> command, the driver leaves the tape logically positioned
  // before the EOF. A related feature is that EOFs remain pending until the tape is closed.
  // For example, a program which first reads all the records of a file up to and including
  // the EOF and then performs an MTFSF command will leave the tape positioned just after
  // that same EOF, rather than skipping the next file.
  // 
  // The MTNBSF and MTFSF operations are inverses. Thus, an `MTFSF -1` is equivalent
  // to an `MTNBSF 1`. An `MTNBSF 0` is the same as `MTFSF 0`; both position the tape
  // device at the beginning of the current file.
  // 
  // MTBSF moves the tape backwards by file marks. The tape position will end on the
  // beginning of the tape side of the desired file mark. An `MTBSF 0` will position
  // the tape at the end of the current file, before the filemark.
  //
  // MTBSR and MTFSR operations perform much like space file operations, except that
  // they move by records instead of files. Variable-length I/O devices
  // (1/2” reel, for example) space actual records; fixed-length I/O devices space
  // physical records (blocks). 1/4” cartridge tape, for example, spaces 512 byte
  // physical records. The status ioctl residual count contains the number of files
  // or records not skipped.
  pub fn fastforward_filemark(dev: &str, count: i32) -> i32 {
    let mut tape_operation = Mtop {
      mt_op: MTFSF,
      mt_count: count
    };

    do_mtioctop(dev, &mut tape_operation)
  }

  pub fn fastforward_record(dev: &str, count: i32) -> i32 {
    let mut tape_operation = Mtop {
      mt_op: MTFSR,
      mt_count: count
    };

    do_mtioctop(dev, &mut tape_operation)
  }


  // #define	MTIOCGET	_IOR('m', 2, struct mtget)
  pub fn status(dev: &str) -> i32 {
    let mut tape_status = Mtget::default();
    let fd = get_filedescriptor(dev).unwrap();

    unsafe {
      match mtiocget(fd.fd, &mut tape_status) {
        Err(_why) => -2,
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

  pub fn get_position(dev: &str) -> i32 {
    let mut tape_position = Mtpos::default();
    let fd = get_filedescriptor(dev).unwrap();

    unsafe {
      match mtiocpos(fd.fd, &mut tape_position) {
        Err(_why) => {
          return 2
        },
        Ok(result) => result,
      };
    }

    println!("{:?}", tape_position);
    return 0;
  }

  fn get_filedescriptor(dev: &str) -> Result<FileDescriptor, nix::Error> {
    let devstr = CString::new(dev).unwrap();
    unsafe {
      match libc::openat(libc::AT_FDCWD, devstr.as_ptr(), libc::O_RDONLY) {
        result if result < 0 => Err(nix::Error::Sys(nix::errno::Errno::EIO)),
        result => Ok(FileDescriptor{ fd: result })
      }
    }
  }

}

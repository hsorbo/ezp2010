mod arguments;

pub mod ezp_common {

    #[derive(Debug, Clone)]
    pub enum ChipType {
        Spi,
        EE24,
        EE25,
        EE93,
    }
    impl ChipType {
        pub fn chip_to_u8(&self) -> u8 {
            return match &self {
                ChipType::Spi => 0x01,
                ChipType::EE24 => 0x02,
                ChipType::EE25 => 0x03,
                ChipType::EE93 => 0x04,
            };
        }
    }
}
pub mod ezp_commands {
    use crate::ezp_common::ChipType;
    use std::{error::Error, fmt};

    use byteorder::*;

    #[derive(Debug)]
    pub struct MyError {
        details: String,
    }

    impl MyError {
        pub fn new(msg: &str) -> MyError {
            MyError {
                details: msg.to_string(),
            }
        }
    }
    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for MyError {
        fn description(&self) -> &str {
            &self.details
        }
    }

    // enum Commands {
    //     Read = 0x110a,
    //     Write = 0x120c,
    //     Detect = 0x1500,
    //     Version = 0x1700,
    //     Serial = 0x1800,
    //     SelfTest = 0xf300,
    // }

    pub fn create_read_cmd(chip_type: &ChipType, size: u32, flags: u16, is5v: bool) -> Vec<u8> {
        let mut data = vec![0x11, 0x0a, chip_type.chip_to_u8(), 0x00, 0x00, 0x00, 0x00];
        data.write_u32::<NetworkEndian>(size).unwrap();
        data.write_u16::<NetworkEndian>(flags).unwrap();
        data.push(if is5v { 0x01 } else { 0x00 });
        data.push(0x00);
        return data;
    }

    pub fn process_read_cmd(resp: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let success: [u8; 3] = [0x11, 0x01, 0x00];
        if resp != success {
            return Err(Box::new(MyError::new("No")));
        }
        return Ok(());
    }

    pub fn create_write_cmd(
        chip_type: &ChipType,
        size: u32,
        flags: u16,
        write_flags: Option<u16>,
        is5v: bool,
    ) -> Vec<u8> {
        let mut data = vec![0x12, 0x0c, chip_type.chip_to_u8(), 0x00, 0x00, 0x00, 0x00];
        data.write_u32::<NetworkEndian>(size).unwrap();
        data.write_u16::<NetworkEndian>(write_flags.unwrap_or(0x01))
            .unwrap();
        data.write_u16::<NetworkEndian>(flags).unwrap();
        data.push(if is5v { 0x01 } else { 0x00 });
        data.push(0x00);
        return data;
    }

    pub fn process_write_cmd(resp: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        //println!("{}",hex::encode(&resp));
        let success: [u8; 3] = [0x12, 0x01, 0x01];
        if resp != success {
            return Err(Box::new(MyError::new("No")));
        }
        return Ok(());
    }

    pub fn create_detect_cmd(chip_type: &ChipType) -> Vec<u8> {
        return vec![0x15, 0x00, chip_type.chip_to_u8()];
    }

    pub fn process_detect_cmd(resp: &[u8]) -> Result<(u8,u8), Box<dyn std::error::Error>> {
        let found: [u8; 2] = [0x15, 0x02];
        if resp[..2] != found {
            return Err(Box::new(MyError::new("Detect chip error!")));
        }
        //chip_type: &ChipType
        return Ok((resp[2], resp[3]));
        //cmp     al, 0FFh, check if manufactor is != 0xff

        //ManufacturerID:%X\r\nDeviceID:%X
        //ManufacturerID: 1c DeviceID: 13
    }

    pub fn create_version_cmd() -> Vec<u8> {
        return vec![0x17, 0x00];
    }

    pub fn process_version(data: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        if data[..2] != [0x17, 0x1e] {
            return Err(Box::new(MyError::new("No")));
        }
        //core::slice::ascii::escape_ascii
        let end = data[2..].iter().position(|x| x == &0x00).unwrap_or(0);
        return Ok(String::from_utf8(data[2..end].to_vec())?);
    }

    pub fn create_serial_cmd() -> Vec<u8> {
        return vec![0x18, 0x00];
    }

    pub fn process_serial(data: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        if data[..2] != [0x18, 0x10] {
            return Err(Box::new(MyError::new("No")));
        }
        //TODO: Why 2x 0xff at end?
        return Ok(String::from_utf8(data[2..16].to_vec())?);
    }

    pub fn create_self_test_cmd() -> Vec<u8> {
        return vec![0xf3, 0x00];
    }
}

pub mod db {

    use crate::ezp_common;

    const ENTRY_SIZE: usize = 0x6C;
    const ENTRY_PAD: usize = 24;
    const DATA_SIZE: usize = 81100;
    const ENTRIES_START: usize = 0x64;
    const ENTRY_COUNT: usize = (DATA_SIZE - ENTRIES_START) / ENTRY_SIZE;
    const DATA: &[u8; DATA_SIZE] = include_bytes!("db.bin");

    #[derive(Debug)]
    #[repr(C)]
    struct RawChipDbEntry {
        chip_type: u32,
        product_name: [u8; 40],
        vendor_name: [u8; 20],
        _unknown1: u8,
        voltage: u8,
        size: u32,
        write_1: u32,
        write_2: u16,
        manufacturer_id: u8,
        device_id: u8,
        ee93_unk: u8,
        ee93_bits: u8,
    }

    #[derive(Debug, Clone)]
    pub struct ChipDbEntry {
        pub chip_type: ezp_common::ChipType,
        pub product_name: String,
        pub vendor_name: String,
        pub size: u32,
        pub voltage: u8,
        pub device_id: u8,
        pub manufacturer_id: u8,
        pub ee93_unk: u8,
        pub ee93_bits: u8,
        pub write_flag: Option<u16>,
    }
    impl ChipDbEntry {
        pub fn is5v(&self) -> bool {
            return self.voltage > 0x28;
        }
        pub fn flags(&self) -> u16 {
            return match self.chip_type {
                ezp_common::ChipType::Spi => 0x0300,
                ezp_common::ChipType::EE24 => {
                    // if self.ee24_unk == 0xfe { 0x0400 } } else { ... //not found in database
                    if self.size > 0x800 {
                        0x2400
                    } else {
                        0x1400
                    }
                }
                ezp_common::ChipType::EE25 => {
                    if self.size > 0x200 {
                        0x1100
                    } else {
                        0x0100
                    }
                }
                ezp_common::ChipType::EE93 => {
                    let hi = 0x10 * self.ee93_unk | 0x08;
                    let lo = if self.ee93_bits == 0x08 { 0x03 } else { 0x01 };
                    return ((hi as u16) << 8) | (lo as u16);
                }
            };
        }
    }
    fn parse_string(buf: &[u8]) -> Result<&str, Box<dyn std::error::Error>> {
        let kake = buf.iter().position(|&s| s == 0x00).ok_or("No terminator")?;
        return Ok(std::str::from_utf8(&buf[0..kake])?);
    }
    pub fn to_chiptype(t: u32) -> ezp_common::ChipType {
        return match t {
            0 => ezp_common::ChipType::Spi,
            1 => ezp_common::ChipType::EE24,
            2 => ezp_common::ChipType::EE25,
            3 => ezp_common::ChipType::EE93,
            _ => panic!("burn"),
        };
    }

    fn get(index: usize) -> Result<ChipDbEntry, Box<dyn std::error::Error>> {
        let offset = ENTRIES_START + (index * ENTRY_SIZE);
        let buf: &[u8] = &DATA[offset..offset + (ENTRY_SIZE - ENTRY_PAD)];
        let s: RawChipDbEntry = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
        let entry = ChipDbEntry {
            chip_type: to_chiptype(s.chip_type),
            size: s.size as u32,
            voltage: s.voltage,
            manufacturer_id: s.manufacturer_id,
            product_name: parse_string(&s.product_name)?.into(),
            vendor_name: parse_string(&s.vendor_name)?.into(),
            ee93_bits: s.ee93_bits,
            ee93_unk: s.ee93_unk,
            device_id: s.device_id,
            write_flag: match s.write_1 {
                0x00 => None,
                _ => Some(s.write_2),
            },
        };
        return Ok(entry);
    }
    pub fn getall() -> Vec<ChipDbEntry> {
        let mut entries: Vec<ChipDbEntry> = vec![];

        for n in 0..ENTRY_COUNT {
            let fooo = get(n).unwrap();
            entries.push(fooo);
        }
        return entries;
    }

    pub fn get_by_product_name(name: &str) -> Option<ChipDbEntry> {
        return getall()
            .iter()
            .find(|x| x.product_name == name)
            .map(|f| f.clone());
    }
}

pub mod programmer {
    use itertools::Itertools;
    use rusb::{
        ConfigDescriptor, DeviceHandle, EndpointDescriptor, GlobalContext, Interface,
        InterfaceDescriptor,
    };
    use std::time::Duration;

    pub trait Programmer {
        fn read(&self, buf: &mut [u8]) -> Result<usize, rusb::Error>;
        fn write(&self, buf: &[u8]) -> Result<usize, rusb::Error>;
    }

    pub struct UsbProgrammerContext {
        pub handle: DeviceHandle<GlobalContext>,
        pub config: ConfigDescriptor,
    }

    fn only_interface(c: &ConfigDescriptor) -> Interface {
        return c
            .interfaces()
            .exactly_one()
            .map_err(|_| "Interface not found")
            .unwrap();
    }

    impl UsbProgrammerContext {
        pub fn open() -> Result<UsbProgrammerContext, Box<dyn std::error::Error>> {
            let mut handle =
                rusb::open_device_with_vid_pid(0x10c4, 0xf5a0).ok_or("Programmer not found")?;
            let device = handle.device();
            let config = device.config_descriptor(0)?;

            handle.set_auto_detach_kernel_driver(true)?;
            handle.set_active_configuration(config.number())?;

            let iface = only_interface(&config);
            handle.claim_interface(iface.number())?;
            let k = UsbProgrammerContext { handle, config };
            return Ok(k);
        }
    }

    pub struct UsbProgrammer<'a> {
        handle: DeviceHandle<GlobalContext>,
        fin: EndpointDescriptor<'a>,
        fout: EndpointDescriptor<'a>,
    }
    impl UsbProgrammer<'_> {
        pub fn create_programmer<'a>(
            handle: DeviceHandle<GlobalContext>,
            ifdesc: &'a InterfaceDescriptor,
        ) -> UsbProgrammer<'a> {
            return UsbProgrammer {
                handle: handle,
                fout: ifdesc
                    .endpoint_descriptors()
                    .find(|x| x.direction() == rusb::Direction::Out)
                    .unwrap(),
                fin: ifdesc
                    .endpoint_descriptors()
                    .find(|x| x.direction() == rusb::Direction::In)
                    .unwrap(),
            };
        }
    }
    const TIMEOUT: Duration = core::time::Duration::from_millis(10000);

    impl Programmer for UsbProgrammer<'_> {
        fn write(&self, buf: &[u8]) -> Result<usize, rusb::Error> {
            return self.handle.write_bulk(self.fout.address(), buf, TIMEOUT);
        }
        fn read(&self, buf: &mut [u8]) -> Result<usize, rusb::Error> {
            return self.handle.read_bulk(self.fin.address(), buf, TIMEOUT);
        }
    }
}

pub mod programming {
    use crate::{
        db::ChipDbEntry,
        ezp_commands,
        ezp_common::ChipType,
        programmer::{Programmer, UsbProgrammer},
    };

    pub fn get_serial(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
        let mut data: [u8; 512] = [0x00; 512];
        let _ = p.write(&ezp_commands::create_serial_cmd());
        let read = p.read(&mut data)?;
        return Ok(ezp_commands::process_serial(&data[..read].to_vec())?);
    }

    pub fn get_version(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
        let mut data: [u8; 512] = [0x00; 512];
        let _ = p.write(&ezp_commands::create_version_cmd());
        let read = p.read(&mut data)?;
        return Ok(ezp_commands::process_version(&data[..read].to_vec())?);
    }

    pub fn self_test(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
        let mut data: [u8; 512] = [0x00; 512];
        let _ = p.write(&ezp_commands::create_self_test_cmd());
        let _ = p.read(&mut data)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        let read = p.read(&mut data)?;
        return Ok(String::from_utf8(data[..read].to_vec())?);
    }

    pub fn detect(p: &UsbProgrammer) -> Result<String, Box<dyn std::error::Error>> {
        let mut data: [u8; 5] = [0x00; 5];
        let cmd = &ezp_commands::create_detect_cmd(&ChipType::Spi);
        let _ = p.write(cmd);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = p.read(&mut data)?;
        let (man,dev) = ezp_commands::process_detect_cmd(&data)?;
        let all = crate::db::getall();
        let k = all.iter().find(|x| x.device_id == dev && x.manufacturer_id == man).ok_or("not d")?;
        return Ok(k.product_name.clone());
    }

    pub fn read(
        p: &UsbProgrammer,
        chip: &ChipDbEntry,
        writer: &mut dyn std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut data: [u8; 4096] = [0x00; 4096];
        let cmd =
            &ezp_commands::create_read_cmd(&chip.chip_type, chip.size, chip.flags(), chip.is5v());
        let _ = p.write(cmd);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let read = p.read(&mut data)?;
        ezp_commands::process_read_cmd(&data[..read])?;
        let mut total: usize = 0;

        loop {
            let read = p.read(&mut data)?;
            total += read;
            writer.write(&data[..read])?;
            if read < 4096 {
                break;
            }
        }
        if total != chip.size as usize {
            println!("Size mismatch got {} expected {}", total, chip.size);
        }
        return Ok(());
    }
    pub fn write(
        p: &UsbProgrammer,
        chip: &ChipDbEntry,
        reader: &mut dyn std::io::Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut data: [u8; 4096] = [0x00; 4096];
        let cmd = &&ezp_commands::create_write_cmd(
            &chip.chip_type,
            chip.size,
            chip.flags(),
            chip.write_flag,
            chip.is5v(),
        );
        let _ = p.write(cmd);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let read = p.read(&mut data)?;
        ezp_commands::process_write_cmd(&data[..read])?;
        let mut total: usize = 0;
        loop {
            let read = reader.read(&mut data)?;
            p.write(&data[..read])?;
            total += read;
            if total >= chip.size as usize {
                break;
            }
        }
        if total != chip.size as usize {
            println!("Size mismatch got {} expected {}", total, chip.size);
        }
        return Ok(());
    }

    pub fn erase(p: &UsbProgrammer) -> Result<(), Box<dyn std::error::Error>> {
        //todo: guard against wrong chip
        let cmd_start_erase: [u8; 3] = [0x14, 0x01, 0x01];
        let start_erase_success: [u8; 3] = [0x14, 0x01, 0x01];

        let cmd_erase: [u8; 3] = [0x13, 0x00, 0x01];
        let erase_ok: [u8; 3] = [0x13, 0x01, 0x01];
        let erase_done: [u8; 3] = [0x13, 0x01, 0x00];
        let mut response: [u8; 3] = [0x00, 0x00, 0x00];

        let _ = p.write(&cmd_start_erase);
        let _ = p.read(&mut response)?;
        if response != start_erase_success {
            return Err(Box::new(crate::ezp_commands::MyError::new(
                "Erase start fail",
            )));
        }
        loop {
            let _ = p.write(&cmd_erase);
            let _ = p.read(&mut response)?;
            if response == erase_done {
                return Ok(());
            }
            if response != erase_ok {
                return Err(Box::new(crate::ezp_commands::MyError::new("Erase fail")));
            }
        }
    }
}

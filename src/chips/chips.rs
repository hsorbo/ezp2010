// Automatically generated rust module for 'chips.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChipInfo<'a> {
    pub type_pb: mod_ChipInfo::RomType,
    pub device_name: Cow<'a, str>,
    pub manufacturer_name: Cow<'a, str>,
    pub voltage: u32,
    pub size: u32,
    pub write_1: bool,
    pub write_2: i32,
    pub manufacturer_id: u32,
    pub device_id: u32,
    pub ee93_unk: u32,
    pub ee93_bits: u32,
}

impl<'a> MessageRead<'a> for ChipInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.device_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.manufacturer_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.voltage = r.read_uint32(bytes)?,
                Ok(40) => msg.size = r.read_uint32(bytes)?,
                Ok(48) => msg.write_1 = r.read_bool(bytes)?,
                Ok(56) => msg.write_2 = r.read_int32(bytes)?,
                Ok(64) => msg.manufacturer_id = r.read_uint32(bytes)?,
                Ok(72) => msg.device_id = r.read_uint32(bytes)?,
                Ok(80) => msg.ee93_unk = r.read_uint32(bytes)?,
                Ok(88) => msg.ee93_bits = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChipInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == chips::mod_ChipInfo::RomType::Spi { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.device_name == "" { 0 } else { 1 + sizeof_len((&self.device_name).len()) }
        + if self.manufacturer_name == "" { 0 } else { 1 + sizeof_len((&self.manufacturer_name).len()) }
        + if self.voltage == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.voltage) as u64) }
        + if self.size == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + if self.write_1 == false { 0 } else { 1 + sizeof_varint(*(&self.write_1) as u64) }
        + if self.write_2 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.write_2) as u64) }
        + if self.manufacturer_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.manufacturer_id) as u64) }
        + if self.device_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.device_id) as u64) }
        + if self.ee93_unk == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.ee93_unk) as u64) }
        + if self.ee93_bits == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.ee93_bits) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != chips::mod_ChipInfo::RomType::Spi { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.device_name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.device_name))?; }
        if self.manufacturer_name != "" { w.write_with_tag(26, |w| w.write_string(&**&self.manufacturer_name))?; }
        if self.voltage != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.voltage))?; }
        if self.size != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.size))?; }
        if self.write_1 != false { w.write_with_tag(48, |w| w.write_bool(*&self.write_1))?; }
        if self.write_2 != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.write_2))?; }
        if self.manufacturer_id != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.manufacturer_id))?; }
        if self.device_id != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.device_id))?; }
        if self.ee93_unk != 0u32 { w.write_with_tag(80, |w| w.write_uint32(*&self.ee93_unk))?; }
        if self.ee93_bits != 0u32 { w.write_with_tag(88, |w| w.write_uint32(*&self.ee93_bits))?; }
        Ok(())
    }
}

pub mod mod_ChipInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RomType {
    Spi = 0,
    EE24 = 1,
    EE25 = 2,
    EE93 = 3,
}

impl Default for RomType {
    fn default() -> Self {
        RomType::Spi
    }
}

impl From<i32> for RomType {
    fn from(i: i32) -> Self {
        match i {
            0 => RomType::Spi,
            1 => RomType::EE24,
            2 => RomType::EE25,
            3 => RomType::EE93,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RomType {
    fn from(s: &'a str) -> Self {
        match s {
            "Spi" => RomType::Spi,
            "EE24" => RomType::EE24,
            "EE25" => RomType::EE25,
            "EE93" => RomType::EE93,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chips<'a> {
    pub chips: Vec<ChipInfo<'a>>,
}

impl<'a> MessageRead<'a> for Chips<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.chips.push(r.read_message::<ChipInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Chips<'a> {
    fn get_size(&self) -> usize {
        0
        + self.chips.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.chips { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}


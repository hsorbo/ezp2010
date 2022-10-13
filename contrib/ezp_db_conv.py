#!/usr/bin/env python3
"""ezp_db_conv.py: Converts the eeprom-database-file (DateBase.dat) 
from EZP2010.exe. to various formats."""
__author__      = "Håvard Sørbø"
__copyright__   = "Copyright 2022"
__license__     = "GPL2"

import struct
import json

def decode(b:bytes) -> bytes:
    data = bytearray()
    k = 1
    for x in range(0, len(b)):
        k = (k + (((k << 3) - k) * 0x190 + k) * 6) & 0x7fffffff
        data.append((b[x] - ((k >> 0x10) + k) - 0x5a) & 0xff)
    return bytes(data)

def parse_entry(entry: bytes):
    (type, device_name, manufacturer_name, unknown, voltage, size, write_1, write_2, manufacturer_id, device_id, ee93_unk, ee93_bits) = struct.unpack(
        'I 40s 20s c b 2x I I h B B B B 26x', entry)
    return {
        'type': type,
        'device_name': device_name.decode('ascii').replace('\0', ''),
        'manufacturer_name': manufacturer_name.decode('ascii').replace('\0', ''),
        #'unknown': unknown,
        'voltage': voltage,
        'size': size,
        'write_1': write_1,
        'write_2': write_2,
        'manufacturer_id': manufacturer_id,
        'device_id': device_id,
        'ee93_unk': ee93_unk,
        'ee93_bits': ee93_bits
    }

def read_db(filename: str):
    data = decode(open(filename, 'rb').read())
    entry_count = struct.unpack('H', data[0:2])[0]
    entry_offset = data[0x64:]
    while entry_count > 0:
        entry_data = entry_offset[:0x6C]
        yield entry_data
        entry_count -= 1
        entry_offset = entry_offset[0x6C:]

def create_json(entries):
    open('DateBase.json', 'w').write(json.dumps(entries, indent=2))

def create_markdown(entries):
    types = ['SPI', '24XX', '25XX','93XX']
    f = open('ROMS.md', 'w')
    f.write('# Supported ROMS\n\n')
    f.write('|Type|Device Name|Manufacturer Name|Voltage|Size|\n')
    f.write('|-|-|-|-|-|\n')
    for x in entries:
        f.write('|{}|{}|{}|{}|{}|\n'.format(types[x['type']], x['device_name'], x['manufacturer_name'], x['voltage'], x['size']))

if __name__ == "__main__":
    filename = 'DateBase.dat'
    entries = [parse_entry(x) for x in read_db(filename)]
    #create_json(entries)
    create_markdown(entries)


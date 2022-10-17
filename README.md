# ezp2010
Flashing roms using the ezp2010 programmer

Supported [ROMS](roms.md)

## Installation

### macOS

```sh
brew tap hsorbo/tap
brew install ezp2010
```

## Running
```
Read and write flash-roms using ezp2010

Usage: ezp <COMMAND>

Commands:
  read    Read from rom
  write   Write to rom
  info    Shows information about connected programmer
  erase   Erase (on supported chips)
  detect  Detect rom
  list    Shows available flash rom type
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

```
ezp info
Programmer: EZP2010 V2
S/N: 016059302-1211
Status: Hardware test ok!
```

```
ezp detect
EN25F80
```

```
ezp list | tail
XICOR      XC25020                  256B 	--type='XC25020' 
XICOR      XC25040                  512B 	--type='XC25040' 
XICOR      XC25080                  1kB  	--type='XC25080' 
XICOR      XC25128                  16kB 	--type='XC25128' 
XICOR      XC25160                  2kB  	--type='XC25160' 
XICOR      XC25256                  33kB 	--type='XC25256' 
XICOR      XC25320                  4kB  	--type='XC25320' 
XICOR      XC25512                  66kB 	--type='XC25512' 
XICOR      XC25640                  8kB  	--type='XC25640' 
XICOR      XC25640                  8kB  	--type='XC25640' 
```

```
time ezp read --type='EN25F80' foo.bin
Reading....
ezp read --type='EN25F80' foo.bin  0.01s user 0.05s system 1% cpu 3.343 total
```

* Requires

```sh
pacman -S gcc binutils
```

* Generate 60Hz EDID file for yoga 14s

```sh
edid-gen 2880 1800 --output 2880x1800.bin --timing-name "Linux 2.8K"
cat 2880x1800.bin|edid-decode
```

```
edid-decode (hex):

00 ff ff ff ff ff ff 00 31 d8 00 00 00 00 00 00
05 16 01 04 6d 4b 2e 78 ea 5e c0 a4 59 4a 98 25
20 50 54 00 00 00 49 00 01 01 01 01 01 01 01 01
01 01 01 01 01 01 a8 ac 40 30 b4 08 41 70 e0 38
36 10 ee d4 21 00 00 1e 00 00 00 ff 00 4c 69 6e
75 78 20 23 30 0a 20 20 20 20 00 00 00 fd 00 3b
3d 6e 70 2d 00 0a 20 20 20 20 20 20 00 00 00 fc
00 4c 69 6e 75 78 20 32 2e 38 4b 0a 20 20 00 19

----------------

Block 0, Base EDID:
  EDID Structure Version & Revision: 1.4
  Vendor & Product Identification:
    Manufacturer: LNX
    Model: 0
    Made in: week 5 of 2012
  Basic Display Parameters & Features:
    Analog display
    Input voltage level: 0.7/0.7 V
    Blank level equals black level
    Sync: Separate Composite Serration
    Maximum image size: 75 cm x 46 cm
    Gamma: 2.20
    DPMS levels: Standby Suspend Off
    RGB color display
    First detailed timing includes the native pixel format and preferred refresh rate
  Color Characteristics:
    Red  : 0.6416, 0.3486
    Green: 0.2919, 0.5957
    Blue : 0.1474, 0.1250
    White: 0.3125, 0.3281
  Established Timings I & II: none
  Standard Timings:
    GTF     :   832x520    60.000 Hz  16:10   32.340 kHz   34.151 MHz
  Detailed Timing Descriptors:
    DTD 1:  2880x1800   59.969 Hz   8:5   111.842 kHz  442.000 MHz (750 mm x 468 mm)
                 Hfront  224 Hsync 312 Hback 536 Hpol P
                 Vfront    3 Vsync   6 Vback  56 Vpol P
    Display Product Serial Number: 'Linux #0'
  Display Range Limits:
    Monitor ranges (GTF): 59-61 Hz V, 110-112 kHz H, max dotclock 450 MHz
    Display Product Name: 'Linux 2.8K'
Checksum: 0x19
```

* How to use EDID

See [Forcing modes and EDID](https://wiki.archlinux.org/title/kernel_mode_setting#Forcing_modes_and_EDID)

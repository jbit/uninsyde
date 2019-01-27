uninsyde
========
Extract chunks from Insyde H2O Iflash files.

Useful for getting plain ROM images out of windows firmware updates.

Installing
----------
Ensure you have a modern version of rust and cargo installed. (See https://rustup.rs)

```
~/uninsyde$ cargo install --path .
  Installing uninsyde v0.1.0 (/home/user/iflash_ex)
   Compiling uninsyde v0.1.0 (/home/user/iflash_ex)
    Finished release [optimized] target(s) in 1.35s
  Installing /home/user/.cargo/bin/uninsyde
```

ACER BIOS example
-----------------
```
~$ ls
BIOS_Acer_1.08_A_A.zip

~$ unzip BIOS_Acer_1.08_A_A.zip 
Archive:  BIOS_Acer_1.08_A_A.zip
   creating: BIOS_ACER_1.08_Windows/
  inflating: BIOS_ACER_1.08_Windows/Alien_BIOS_V1.08_EC_V0125_20180724.exe  

~$ cd BIOS_ACER_1.08_Windows/

~/BIOS_ACER_1.08_Windows$ 7za x Alien_BIOS_V1.08_EC_V0125_20180724.exe 

~/BIOS_ACER_1.08_Windows$ ls
Alien_BIOS_V1.08_EC_V0125_20180724.exe  H2OFFT.inf
BiosEc.fd                               H2OFFT-Wx64.exe
BiosImageProcx64.dll                    mfc90u.dll
Ding.wav                                Microsoft.VC90.CRT.manifest
FlsHook.exe                             Microsoft.VC90.MFC.manifest
FWUpdLcl.exe                            msvcp90.dll
H2OFFT64.sys                            msvcr90.dll
H2OFFT.cat                              platform.ini

~/BIOS_ACER_1.08_Windows$ uninsyde BiosEc.fd 
Reading  "BiosEc.fd"
offset=00005238 name=DRV_IMG file_size=9465320 data_size=9465320 (padding=0)
offset=000de9b0 name=BIOSIMG file_size=8388616 data_size=8388608 (padding=8)
offset=008de9d0 name=INI_IMG file_size=53096 data_size=53073 (padding=23)
offset=008eb950 name=EC_IMG_ file_size=131080 data_size=131072 (padding=8)
offset=0090b970 name=BIOSCER file_size=264 data_size=256 (padding=8)
offset=0090ba90 name=BIOSCR2 file_size=8 data_size=0 (padding=8)
Done

~/BIOS_ACER_1.08_Windows$ ls *.bin
BIOSCER.bin  BIOSCR2.bin  BIOSIMG.bin  DRV_IMG.bin  EC_IMG_.bin  INI_IMG.bin
```

# Apator WMBus reader

CC1101-based WMBus reader for Apator AT-WMBUS-16-2 water volume meters for Linux-based SBCs.
Uses `spidev` for communication with CC1101 module and `gpio-cdev` for RX FIFO-ready interrupt handling.

The repository contains:

* CC1101 hat designed for Orange Pi Zero 3 - KiCad board,
* Rust library for interfacing with CC1101 module,
* Rust application for receiving and decoding WMBus messages using CC1101 and publishing readings to MQTT broker.

## Board

<a href="./.docs/board-3d.jpg"><img src="./.docs/board-3d.jpg" height="200"/></a>
<a href="./.docs/board.jpg"><img src="./.docs/board.jpg" height="200"/></a>

## Resources

### CC1101
* https://github.com/fphammerle/python-cc1101 - Python interface for CC1101
* [Using CC1101 for WMBus reception (PDF)](https://www.ti.com/lit/an/swra234a/swra234a.pdf) - TI
* https://github.com/dsvensson/cc1101
* [CC1101 Datasheet](https://www.ti.com/lit/ds/symlink/cc1101.pdf)

### WMBus
* https://github.com/wmbusmeters/wmbusmeters - Decoding
* [WMBus frame formats (PDF)](https://www.ti.com/lit/an/swra522e/swra522e.pdf) - TI
* [WMBus frame formats (PDF)](https://www.ti.com/lit/an/swra234a/swra234a.pdf) - TI
* [WMBus frame formats (PDF)](https://www.st.com/resource/en/application_note/an4772-wmbus-2013-firmware-stack-overview-stmicroelectronics.pdf) - ST
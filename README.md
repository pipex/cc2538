[![Build Status](https://travis-ci.com/pipex/cc2538.svg?branch=master)](https://travis-ci.com/pipex/cc2538)

# cc2538

CC2538 Peripheral Access API generated using [svd2rust](https://github.com/rust-embedded/svd2rust). The svd was generated via [tixml2svd](https://github.com/dhoove/tixml2svd).

The API includes all peripherals for the CC2538SF53 (512kb flash, 32kb RAM) version of the SoC (see the [CC2538 device overview](http://www.ti.com/lit/ds/symlink/cc2538.pdf), page 6 for differences between different versions). The reason behind this is that I only have a [Zolertia Firefly](https://github.com/Zolertia/Resources/wiki/Firefly) device to test, which integrates that version of the SoC.

**This was generated automatically and has not been thoroughly tested. Use with caution.**

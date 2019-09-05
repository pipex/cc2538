# cc2538

CC2538 Peripheral Access API generated using [svd2rust](https://github.com/rust-embedded/svd2rust). The svd was generated via [tixml2svd](https://github.com/dhoove/tixml2svd).

The API includes all peripherals for the CC2538SF53 (512kb flash, 32kb RAM) version of the SoC (see the [CC2538 device overview](http://www.ti.com/lit/ds/symlink/cc2538.pdf), page 6 for differences between different versions). The reason behind this is that I only have a Zolertia Firefly device to test, which integrates that version of the SoC.

**This is highly experimental work in progress and has not been tested. Seriously don't use it yet**

## TODO

- [x] Generate API using svd2rust
- [ ] (Maybe) include the bootloaded backdoor as a memory section to flash firmware using [cc2538-bsl](https://github.com/JelmerT/cc2538-bsl).
- [ ] Develop some integration tests on real hardware
- [ ] Allow disabling crate features for other versions of the SoC.

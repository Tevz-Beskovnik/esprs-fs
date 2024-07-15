# This repo is to test the implementation of SPIFFS in epsrs

## Prerequisits

To successfully compile and run this program you will need to have the following things installed:

- python3,
- esptool.py,
- espflash cli
- esp-idf,
- esp-rs,
- rust toolchain

#### espflash

Can be instlled easily enought via cargo

#### esptool.py

Can be installed via `pip install esptool.py`

#### esp-rs

For the proper installation of eps-rs the tutorial on the [website](https://docs.esp-rs.org/book/) should be followed. I would recommend installing for both xtensa and RISC-V targets, but this project only supports RISC-V

### Development environment

I use vscode for most of my projects. I recommend having `rust-analyzer` installed. The only other thing to do before writting code is to add the following lines to your .vscode `settings.json` file:

```json
{
  "rust-analyzer.check.allTargets": false,
  "rust-analyzer.cargo.target": "riscv32imac-esp-espidf"
}
```

Please note though that this config only works for riscv targets (`riscv32imac-esp-espidf`)

## Running and flashing

To flash filesystem and new partitions to device run the `flash_spiffs.sh` script

```sh
./scripts/flash_spiffs.sh
```

To flash chip and monitor it run:

```sh
espflash flash target/riscv32imac-esp-espidf/release/esp-rs-fs-test --monitor
```

To only monitor run:

```sh
espflash monitor
```

## Things to keep in mind

When adding a spiffs storage partition it has to be added to the partition table with subtype `spiffs`, and the partition table on the device needs to get reflashed.

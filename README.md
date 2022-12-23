# BouffaloLab Rust Experiment
This repository is a Cargo workspace containing three lib crates and an example
binary crate.

## Embedded HAL and runtime (PoC)
The two crates `embedded-hal` and `embedded-rt` are two experimental crates 
that provides an alternative way of abstracting the embedded development in
Rust. This has nothing to do with the existing and published `embedded-hal`
crate. 

## BouffaloLab HAL
This is an HAL implementation for BouffaloLab chips, it depends on the 
workspace crate `embedded-hal`. It currently only support BL808 as it's my
main goal for now. It can be considered as a Rust port of the official 
[MCU SDK].

[MCU SDK]: https://github.com/bouffalolab/bl_mcu_sdk/

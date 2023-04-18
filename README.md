# BouffaloLab Rust Experiment
This repository is a Cargo workspace containing three lib crates and an example
binary crate.

## Embedded HAL utilities and runtime (PoC)
The crate `embedded-util` provides an alternative way of abstracting the embedded 
development in Rust. It provides utilities to generate peripherals and exclusive
access.

This crate completes the official `embedded-hal` crate by providing a new way of
defining Peripheral Access Crate, MMIO structures and bitfield registers.

## BouffaloLab HAL
This is an HAL implementation for BouffaloLab chips. It currently only support 
BL808 as it's my main goal for now. It can be considered as a Rust port of the 
official [MCU SDK].

## BouffaloLab Runtime
TODO

[MCU SDK]: https://github.com/bouffalolab/bouffalo_sdk

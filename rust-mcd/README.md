# Multi-Core-Debug library in rust

This library provides a rust type-safe API over Infineons MCD library. It
is far from complete, pull requests are welcome!

Due to the MCD library being only available in windows, this crate will only work
properly when compiled for windows.

This library is based on the demo provided by infineon, also included in this project
within [`mcd_demo_basic_120412`](mcd_demo_basic_120412).
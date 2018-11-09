## Goals

The aim of this repository is to create a working emulator for the target system that includes all the documented and undocumented behavior that's required to correctly run popular applications for that system.

The emulator should primarily be an end-user application, but we should keep in mind the possibility of providing it as a library for reuse as part of other systems.

Ideally, the core should be fully portable and idiomatic Rust.

## Contributors

Username|Contributions
:------:|:----------------
TChatzigiannakis|Architecture, instruction set (8-bit and 16-bit arithmetic, exchange, control)
Vassalware|Instruction set (load, call, return, jump)
raidenfreeman|Instruction set (8-bit arithmetic, bitwise)

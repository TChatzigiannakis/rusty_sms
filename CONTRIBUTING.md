## Goals

The aim of this repository is to create a working emulator for the target system that includes all the documented and undocumented behavior that's required to correctly run popular applications for that system.

The emulator should primarily be an end-user application, but we should keep in mind the possibility of providing it as a library for reuse as part of other systems.

Ideally, the core should be fully portable and idiomatic Rust.

## Contributions

* Every contributor must sign the CLA.
* To the extent that it is possible, every commit should build successfully in the hooked CI system.
* Contributions should try to follow the style rules of the project,  described below.

### Style

#### General
* Identifiers should use Rust conventions.
* Source files should be formatted according to `rustfmt` defaults.

### Instruction set
* The mapping between mnemonics and opcodes should be in ascending numerical order.
* The mapping between mnemonics and implementations should be in groups based on similar semantics.
* For the core part of a mnemonic, we use upper case for the first letter and lower case for the rest. For example, `ld` in the manual becomes `Ld` in our identifiers.  
* We do not use a comma separator for the arguments. For example, `add a, b` becomes `AddAB` in our identifiers.
* We use V (for "value") as a dereference indicator. For example, `(HL)` in the manual becomes `VHL` in our identifiers.
* We use X for memory reads at the location pointed by the PC. For example, `ld bc, **` becomes `LdBCXX` in our identifiers.
* The instruction implementations should be found in individual files that map to the instruction groups as described in the official Zilog Z80 User Manual.
* Extended instructions should be in their own folders.

## Contributors

Username|Contributions
:------:|:----------------
TChatzigiannakis|Architecture, instruction set (8-bit and 16-bit arithmetic, exchange, control)
Vassalware|Instruction set (load, call, return, jump)
raidenfreeman|Instruction set (8-bit arithmetic, bitwise)


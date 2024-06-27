# fortress

A Rust-based binary orchestrator (and eventual toolchain manager) for compiled Fortran binaries. Still in very WIP stage

The eventual hope is that this program turns into a general-purpose toolchain manager for modern fortran, considering how scattered the tooling ecosystem for the "dinosaur" language is atm :)

## motivation

I see great potential for calling highly optimized numerical routines written in Modern Fortran from Rust for orchestration purposes, i.e., to follow a series of computations or build a numerical data processing pipeline.

## todo

- Learn how to do CLI parsing better in Rust
- Learn error handling better in Rust to make `fortress` more Robust
- Check for fortran compilers in PATH in an OS-agnostic way
- Add support for fpm (fortran package manager)
- Add support for fortls (the fortran language server)


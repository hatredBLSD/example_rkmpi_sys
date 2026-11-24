# rkmpi-sys examples

Example applications for the **[rkmpi-sys](https://crates.io/crates/rkmpi-sys)**
crate — raw FFI bindings to the Rockchip **Rockit MPI SDK** for
**RV1103 / RV1106** SoCs (Luckfox Pico boards).

All examples run directly on the board.

## Examples

| Example | What it does |
|---|---|
| `tcp_mjpeg` | MJPEG over TCP streaming: VI → VENC (JPEG) → stream frames to a TCP client (open in a browser/player as MJPEG) |
| `raw_frame` | Capture a raw frame from VI, get the media buffer (`MB_BLK`) and dump pixels (e.g. NV12) |
| `venc` | MJPEG encoding |
| `rgn` | RGN init | 

## Target platform

* SoC: Rockchip **RV1103 / RV1106** (Cortex-A7, 32-bit ARM, hard-float, uClibc)
* Rust target: `armv7-unknown-linux-uclibceabihf` (nightly, `-Zbuild-std`)
* SDK: [luckfox-pico](https://github.com/LuckfoxTECH/luckfox-pico) with the
  `arm-rockchip830-linux-uclibcgnueabihf` toolchain

## Prerequisites

You need from the Luckfox SDK:

* headers: `media/rockit/rockit/mpi/sdk/include`
* libraries: `librockit.so`, `librockchip_mpp.so(.1)`, `librga.so`

Do **not** rename `librockchip_mpp.so.1` — it is the soname recorded inside
`librockit.so`:

```bash
ln -sf librockchip_mpp.so /path/to/sdk/lib/librockchip_mpp.so.1
```

## Build

```bash
export ROCKIT_LIB=/path/to/sdk/lib
export LD_LIBRARY_PATH=$ROCKIT_LIB

# .cargo/config.toml of this project:
#   [target.armv7-unknown-linux-uclibceabihf]
#   linker = "arm-rockchip830-linux-uclibcgnueabihf-gcc"

cargo +nightly build --release --target armv7-unknown-linux-uclibceabihf \
    -Zbuild-std=core,alloc,std,panic_abort
```

## Run on the board

Copy the binary and make sure the loader finds the SDK libraries
(on the stock Luckfox firmware they are already present):

## Notes

* Every `RK_MPI_*` call is `unsafe` — these examples are thin ports of the
  official C samples and keep the MPI call order: `RK_MPI_SYS_Init()` →
  module setup → `Bind` → loop → `UnBind` → `RK_MPI_SYS_Exit()`.
* Error codes returned by MPI functions are `i32`, `0` means success.
* `RK_BOOL` fields are enums in the bindings: use `RK_BOOL::RK_TRUE` /
  `RK_BOOL::RK_FALSE`, not `true`/`false`.

## License

Apache-2.0 (matching the upstream Rockchip headers).
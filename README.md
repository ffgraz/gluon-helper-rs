# gluon-helper-rs

Project to develop parts of existing infrastructure as rust lib/binaries in order to move away from C

## Structure

- `/src` -> `libgluonhelper` library (rust parts) and diagnostics tool
  - `/src/olsr`: `libolsrhelper`.c rewritten in rust, diagnostics tool
    - `respondd`: `respodd` module that uses `libolsrhelper`
    - lua: lua bindings for `libolsrhelper` as currently exist in `olsrd.c`
  - `/src/babel`: `libbabelhelper.c` rewritten in rust, diagnostics tool (planned)
    - `respondd`: `respodd` module that uses `libbabelhelper` (planned)
  - `/src/batman`: `libbatmanhelper.c` rewritten in rust, diagnostics tool (planned)
    - `/src/respondd`: `respodd` module that uses `libbatmanhelper` (planned)

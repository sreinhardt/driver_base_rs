### Driver Base
#### Spenser Reinhardt

This is an extremely minimal Rust kernel driver for x64 Windows. It's intended to be copied and directly altered for new projects, not included as a cargo dependency.

Most of the hard work was done previously by not-matthais[1] and Codentium[2], and all steps from [Codentium's - Windows Drivers in Rust Prerequisites](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-prerequisites/) should be followed prior to building.

### Modifying Your Build

Few modifications are needed to rename and configure to your own needs.

* Cargo.toml
  * package.name = Rename to your desired crate name.
* Makefile.toml
  * env.PRJ_NAME = Should be equal to set value in cargo.toml from above.
  * env.SVC_NAME = Name used in windbg and `sc start`
  * env.SVC_DISPLAY = More human readable name, used in Services, TaskMgr and similar.

### Cargo Make Options

To build and simplify deployment, cargo make options are:

* build-driver
  * Just builds this repository as `%package.name%.dll`.
* rename
  * Renames from `%package.name%.dll` to `%package.name%.sys`.
* sign
  * Generates a new, or uses existing, signing certificate to sign `%package.name%.sys`.
* install
  * Executes `sc create` command to create a new service for `%package.name%.sys`.
* start
  * Executes `sc start` command to start created service.
* stop
  * Executes `sc stop` command to stop started service.
* delete
  * Executes `sc delete` command to delete stopped service.

* 1 - https://not-matthias.github.io/kernel-driver-with-rust/
* 2 - https://codentium.com/guides/windows-dev/
* 3 - https://codentium.com/guides/windows-dev/windows-drivers-in-rust-prerequisites/

# Rust-QEMU-HelloWorld
 A quick demonstration of compiling something for an embedded target in Rust using QEMU.
 
 It uses GitHub User nrf-rs's rust Crate microbit to build for the nRF51822 microbit v1. Additionally, uses the crates probe-run and flip-link (for stack overflow protection). 

# How to run the binaries using QEMU:

1. Ensure that you have QEMU installed for your current platform (https://www.qemu.org/)
    -You will specifically be emulating an ARM platform, so make sure that this platform is installed.

2. Clone this repository

3. Open a terminal in the `/bin` directory and run the qemu-system-arm binary. You'll specify `-M microbit` as the machine as an argument, and load the ARM binary through specifying `loader,file=hello_world_nrf`.

For example, on my system running Windows, QEMU isn't added to path, so I'll open a terminal in the `/bin` directory and supply this to a PowerShell Terminal:

`."C:\Program Files\qemu\qemu-system-arm.exe" -M microbit -device loader,file=hello_world_nrf`

on other systems where you QEMU is installed, you might be able to get away with this:

`qemu-system-arm -M microbit -device loader,file=hello_world_nrf`

4. Qemu will then open up a monitor window. Under the 'View' tab, select `serial0` to see the "Hello World!" Message.

# How to compile from rust source:

1. Ensure that you have Rust installed, and updated to the newest version through rustup: ensure this is the case by running `rustup update` in a terminal window.

2. Add the compilation target to rust by running `rustup target add thumbv6m-none-eabi` in a terminal.

3. Add the probe-run and flip-link crates by running `cargo install probe-run flip-link` in a terminal.

4. Open a terminal window in /HelloWorldnrf and run `cargo build --features v1 --target thumbv6m-none-eabi` to compile the source for the Microbit v1. The compiled binary will be in `/HelloWorldnrf/target/thumbv6m-none-eabi/debug/hello_world_nrf`.



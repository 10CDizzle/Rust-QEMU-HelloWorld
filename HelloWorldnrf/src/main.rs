#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use microbit::hal::{clocks::Clocks, timer::Timer};
use rubble::beacon::Beacon;
use rubble::link::{ad_structure::AdStructure, CompanyId, MIN_PDU_BUF};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;

//Main entry point into program
fn main() {
}

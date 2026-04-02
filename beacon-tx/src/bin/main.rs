#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
use defmt::{ println };
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::esp_now::{ BROADCAST_ADDRESS };
use esp_radio::wifi::WifiMode;
use ::{ esp_backtrace as _, esp_println as _ };
extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

//Deep sleep
use esp_hal::rtc_cntl::{ Rtc };
use esp_hal::rtc_cntl::sleep::{ TimerWakeupSource };
//

//Custom
use shared::structs::BiomePacket;
use shared::enums::Biome;
use shared::utils;
//

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(
        peripherals.SW_INTERRUPT
    );
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) = esp_radio::wifi
        ::new(&radio_init, peripherals.WIFI, Default::default())
        .expect("Failed to initialize Wi-Fi controller");

    //wifi config
    let mut esp_now = _interfaces.esp_now;
    _ = _wifi_controller.set_mode(WifiMode::Sta);
    _ = _wifi_controller.start();
    _ = esp_now.set_channel(1);

    let biome = BiomePacket::new(Biome::Desert);

    let mut buf = [0u8; 4];
    let biome_packet = postcard::to_slice(&biome, &mut buf).unwrap(); //convert to bytes

    let waker = TimerWakeupSource::new(core::time::Duration::from_secs(5));
    let mut rtc = Rtc::new(peripherals.LPWR);
    //Sending packet then entering deep sleep
    println!("Sending Packet");
    let _info = esp_now.send(&BROADCAST_ADDRESS, biome_packet).unwrap();
    utils::blocking_delay(200);
    rtc.sleep_deep(&[&waker]); //Deep sleeps then reboots
    unreachable!();
}

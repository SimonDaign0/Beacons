#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

//use core::sync::atomic::{ AtomicU8 }; if multi core
mod fsm;
mod enums;
mod structs;

use defmt::{ println };
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::esp_now::{ EspNow };
use esp_radio::wifi::WifiMode;
use shared::enums::Biome;
use ::{ esp_backtrace as _, esp_println as _ };

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

//custom
use shared::structs::BiomePacket;
use shared::utils;
use fsm::StateMachine;
//

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320); //reclaimed mem from boot
    let timg0 = TimerGroup::new(_peripherals.TIMG0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(
        _peripherals.SW_INTERRUPT
    );
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) = esp_radio::wifi
        ::new(&radio_init, _peripherals.WIFI, Default::default())
        .expect("Failed to initialize Wi-Fi controller");

    //wifi config
    let esp_now = _interfaces.esp_now;
    _ = _wifi_controller.set_mode(WifiMode::Sta);
    _ = _wifi_controller.start();
    _ = esp_now.set_channel(1);
    // sm init
    let sm = StateMachine::init();
    //

    loop {
        let biome_recieved = poll_packets(&esp_now);
        gen_pokemon(biome_recieved, &sm);

        utils::blocking_delay(5000);
    }
}

fn poll_packets(esp_now: &EspNow<'_>) -> Option<Biome> {
    let packet = esp_now.receive()?;
    let bytes = packet.data();
    match postcard::from_bytes::<BiomePacket>(bytes) {
        Ok(biome_packet) =>
            match biome_packet.authenticate() {
                Ok(biome) => {
                    while let Some(_) = esp_now.receive() {} //discard remaining queue
                    return Some(biome);
                }
                Err(err) => {
                    println!("Error: {}", err);
                    return None;
                }
            }
        Err(_) => {
            println!("Failed to deserialize packet");
            None
        }
    }
}

fn gen_pokemon(biome_packet: Option<Biome>, sm: &StateMachine) {
    if let Some(biome) = biome_packet {
        let new_pokemon = sm.generate_pokemon(biome);
        println!("generated pokemon: {}", new_pokemon.kind);
    }
}

#![no_std]
#![no_main]

use core::time;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp2040_hal as hal;
use embedded_hal::adc::OneShot;
use hal::{adc::Adc, adc::AdcPin, Sio};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn main() -> ! {
    info!("Program start!"); // プログラム開始のログを出力
    let mut pac = hal::pac::Peripherals::take().unwrap(); // 周辺機器の取得

    let sio = Sio::new(pac.SIO); // SIOの初期化
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0, // IOバンク0
        pac.PADS_BANK0, // パッドバンク0
        sio.gpio_bank0, // GPIOバンク0
        &mut pac.RESETS, // リセット管理
    );
    info!("11");
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS); // ADCの初期化
    info!("12");
    let mut adc_pin_0 = AdcPin::new(pins.gpio26.into_floating_input()).unwrap(); // GPIO26をADC入力に設定

    let pin_adc_counts: u16 = adc.read(&mut adc_pin_0).unwrap(); // ADC値の読み取り
    info!("ADC Value: {}", pin_adc_counts); // ADC値のログ出力
    info!("Fin!");

    loop {
        // info!("test");
        // cortex_m::asm::wfi(); // 低電力モードで待機
        cortex_m::asm::delay(10);
    }
}
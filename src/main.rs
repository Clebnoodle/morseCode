#![no_std]
#![no_main]

// macro for start-up function
use cortex_m_rt::entry;

// halts the program on panic
use panic_halt as _;

// alias for the HAL crate
use rp2040_hal as hal;

// shorter alias for the Peripheral Access Crate
use hal::pac;

// traits
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use rp2040_hal::clocks::Clock;

// this linker will place this boot block at the start of the program image
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// Set external high-speed crystal based on the hardware. Raspberry Pi Pico is 12 MHz.
const XTAL_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let delay: &mut cortex_m::delay::Delay = &mut cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led_pin: &mut hal::gpio::Pin<hal::gpio::bank0::Gpio25, hal::gpio::Output<hal::gpio::PushPull>> = &mut pins.gpio25.into_push_pull_output();

    fn short(led_pin: &mut hal::gpio::Pin<hal::gpio::bank0::Gpio25, hal::gpio::Output<hal::gpio::PushPull>>, delay: &mut cortex_m::delay::Delay) {
        led_pin.set_high().unwrap();
        delay.delay_ms(200);
        led_pin.set_low().unwrap();
    }

    fn long(led_pin: &mut hal::gpio::Pin<hal::gpio::bank0::Gpio25, hal::gpio::Output<hal::gpio::PushPull>>, delay: &mut cortex_m::delay::Delay) {
        led_pin.set_high().unwrap();
        delay.delay_ms(600);
        led_pin.set_low().unwrap();
    }

    fn in_delay(delay: &mut cortex_m::delay::Delay) {
        delay.delay_ms(200);
    }

    fn between_delay(delay: &mut cortex_m::delay::Delay) {
        delay.delay_ms(1400);
    }

    loop {
        // c: l, s, l, s
        long(led_pin, delay);
        in_delay(delay);
        short(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        short(led_pin, delay);
        between_delay(delay);
        
        // s: s, s, s
        short(led_pin, delay);
        in_delay(delay);
        short(led_pin, delay);
        in_delay(delay);
        short(led_pin, delay);
        between_delay(delay);

        // e: s
        short(led_pin, delay);
        between_delay(delay);

        // 3: s, s, s, l, l
        short(led_pin, delay);
        in_delay(delay);
        short(led_pin, delay);
        in_delay(delay);
        short(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        between_delay(delay);

        // 1: s, l, l, l, l
        short(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        between_delay(delay);

        // 0: l, l, l, l, l
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        in_delay(delay);
        long(led_pin, delay);
        between_delay(delay);

        // extra delay between loop
        delay.delay_ms(3000);
    }
}
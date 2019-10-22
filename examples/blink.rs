#![no_std]
#![no_main]

extern crate panic_semihosting;

use rtfm_stm32f0xx::tim::tim2::{Duration, Instant};

use cortex_m_semihosting::hprintln;
use stm32f0xx_hal::{
    gpio::{gpioc, Output, PushPull},
    prelude::*,
    timers,
    time
};

#[rtfm::app(device = stm32f0::stm32f0x2, peripherals = true, monotonic = rtfm_stm32f0xx::tim::tim2::TIM2)]
const APP: () = {
    struct Resources {
        led: gpioc::PC7<Output<PushPull>>
    }

    #[init(schedule = [blink])]
    fn init(ctx: init::Context) -> init::LateResources {
        let mut p: stm32f0::stm32f0x2::Peripherals = ctx.device;
        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
        let _timer = timers::MonotonicTimer::tim2(p.TIM2, time::Hertz(1000), &mut rcc);
        let gpioc = p.GPIOC.split(&mut rcc);

        let led = cortex_m::interrupt::free(|cs| {
            gpioc.pc7.into_push_pull_output(cs)
        });

        ctx.schedule.blink(Instant::now() + Duration::from_ticks(1000)).unwrap();
        init::LateResources { led }
    }

    #[task(schedule = [blink], resources = [led])]
    fn blink(ctx: blink::Context) {
        hprintln!("tock").unwrap();
        ctx.resources.led.toggle().unwrap();
        ctx.schedule.blink(Instant::now() + Duration::from_ticks(1000)).unwrap();
    }

    extern "C" {
        fn ADC_COMP();
    }
};

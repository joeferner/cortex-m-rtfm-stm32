#![no_std]

/// Implements the rtfm::Monotonic trait using stm32f0xx TIMers
///
/// # Examples
///
/// Simple RTFM app:
///
/// ```
/// #[rtfm::app(..., monotonic = rtfm_stm32f0xx::tim::tim2::TIM2)]
/// const APP: () = {
///   #[init]
///   fn init(ctx: init::Context) {
///      initialize_tim2();
///      ctx.schedule.blink(Instant::now() + Duration::from_ticks(1000)).unwrap();
///   }
/// };
/// ```

pub mod tim;

//! TIM based monotonic

macro_rules! timers {
($($TIM:ident: ($tim:ident, $width:ident, $ptr:expr),)+) => {
    $(
        pub mod $tim {
            use core::{
                cmp::Ordering,
                convert::{Infallible, TryInto},
                fmt,
                marker::PhantomData,
                ops,
            };

            use rtfm::{Fraction, Monotonic};

            #[derive(Clone, Copy, Eq, PartialEq)]
            pub struct Instant {
                inner: $width,
                _not_send_or_sync: PhantomData<*mut ()>,
            }

            unsafe impl Sync for Instant {}

            unsafe impl Send for Instant {}

            impl Instant {
                /// Returns an instant corresponding to "now"
                pub fn now() -> Self {
                    unsafe {
                        let ptr = &*$ptr;
                        Instant {
                            inner: ptr.cnt.read().bits() as $width,
                            _not_send_or_sync: PhantomData,
                        }
                    }
                }

                /// Returns the amount of time elapsed since this instant was created.
                pub fn elapsed(&self) -> Duration {
                    Instant::now() - *self
                }

                /// Returns the amount of time elapsed from another instant to this one.
                pub fn duration_since(&self, earlier: Instant) -> Duration {
                    let diff = self.inner - earlier.inner;
                    Duration { inner: diff as u32 }
                }
            }

            impl fmt::Debug for Instant {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_tuple("Instant")
                        .field(&(self.inner as u32))
                        .finish()
                }
            }

            impl ops::AddAssign<Duration> for Instant {
                fn add_assign(&mut self, dur: Duration) {
                    self.inner = self.inner.wrapping_add(dur.inner as $width);
                }
            }

            impl ops::Add<Duration> for Instant {
                type Output = Self;

                fn add(mut self, dur: Duration) -> Self {
                    self += dur;
                    self
                }
            }

            impl ops::SubAssign<Duration> for Instant {
                fn sub_assign(&mut self, dur: Duration) {
                    self.inner = self.inner.wrapping_sub(dur.inner as $width);
                }
            }

            impl ops::Sub<Duration> for Instant {
                type Output = Self;

                fn sub(mut self, dur: Duration) -> Self {
                    self -= dur;
                    self
                }
            }

            impl ops::Sub<Instant> for Instant {
                type Output = Duration;

                fn sub(self, other: Instant) -> Duration {
                    self.duration_since(other)
                }
            }

            impl Ord for Instant {
                fn cmp(&self, rhs: &Self) -> Ordering {
                    self.inner.wrapping_sub(rhs.inner).cmp(&0)
                }
            }

            impl PartialOrd for Instant {
                fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                    Some(self.cmp(rhs))
                }
            }

            /// A `Duration` type to represent a span of time.
            #[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
            pub struct Duration {
                inner: u32,
            }

            impl Duration {
                /// Creates a new `Duration` from the specified number of ticks
                pub fn from_ticks(ticks: u32) -> Self {
                    Duration { inner: ticks }
                }

                /// Returns the total number of ticks contained by this `Duration`
                pub fn as_ticks(&self) -> u32 {
                    self.inner
                }
            }

            impl TryInto<u32> for Duration {
                type Error = Infallible;

                fn try_into(self) -> Result<u32, Infallible> {
                    Ok(self.as_ticks())
                }
            }

            impl ops::AddAssign for Duration {
                fn add_assign(&mut self, dur: Duration) {
                    self.inner += dur.inner;
                }
            }

            impl ops::Add<Duration> for Duration {
                type Output = Self;

                fn add(self, other: Self) -> Self {
                    Duration {
                        inner: self.inner + other.inner,
                    }
                }
            }

            impl ops::SubAssign for Duration {
                fn sub_assign(&mut self, rhs: Duration) {
                    self.inner -= rhs.inner;
                }
            }

            impl ops::Sub<Duration> for Duration {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self {
                    Duration {
                        inner: self.inner - rhs.inner,
                    }
                }
            }

            /// Adds the `ticks` method to the `u32` type
            pub trait U32Ext {
                /// Converts the `u32` value into clock ticks
                fn ticks(self) -> Duration;
            }

            impl U32Ext for u32 {
                fn ticks(self) -> Duration {
                    Duration { inner: self }
                }
            }

            /// Implementation of the `Monotonic` trait
            pub struct $TIM;

            impl Monotonic for $TIM {
                type Instant = Instant;

                fn ratio() -> Fraction {
                    Fraction {
                        numerator: 1,
                        denominator: 1,
                    }
                }

                unsafe fn reset() {
                    let ptr = &*$ptr;
                    ptr.cnt.reset();
                }

                fn now() -> Instant {
                    Instant::now()
                }

                fn zero() -> Instant {
                    Instant {
                        inner: 0,
                        _not_send_or_sync: PhantomData,
                    }
                }
            }
        }
    )+
}}

#[cfg(any(
feature = "stm32f030x8",
feature = "stm32f030xc",
feature = "stm32f070xb",
))]
timers! {
    TIM1: (tim1, u16, stm32f0::stm32f0x0::TIM1::ptr()),
    TIM3: (tim3, u16, stm32f0::stm32f0x0::TIM3::ptr()),
    TIM14: (tim14, u16, stm32f0::stm32f0x0::TIM14::ptr()),
    TIM16: (tim16, u16, stm32f0::stm32f0x0::TIM16::ptr()),
    TIM17: (tim17, u16, stm32f0::stm32f0x0::TIM17::ptr()),
}

#[cfg(any(
feature = "stm32f031",
feature = "stm32f051",
feature = "stm32f071",
feature = "stm32f091",
))]
timers! {
    TIM1: (tim1, u16, stm32f0::stm32f0x1::TIM1::ptr()),
    TIM3: (tim3, u16, stm32f0::stm32f0x1::TIM3::ptr()),
    TIM14: (tim14, u16, stm32f0::stm32f0x1::TIM14::ptr()),
    TIM16: (tim16, u16, stm32f0::stm32f0x1::TIM16::ptr()),
    TIM17: (tim17, u16, stm32f0::stm32f0x1::TIM17::ptr()),
}

#[cfg(any(
feature = "stm32f042",
feature = "stm32f072",
))]
timers! {
    TIM1: (tim1, u16, stm32f0::stm32f0x2::TIM1::ptr()),
    TIM3: (tim3, u16, stm32f0::stm32f0x2::TIM3::ptr()),
    TIM14: (tim14, u16, stm32f0::stm32f0x2::TIM14::ptr()),
    TIM16: (tim16, u16, stm32f0::stm32f0x2::TIM16::ptr()),
    TIM17: (tim17, u16, stm32f0::stm32f0x2::TIM17::ptr()),
}

#[cfg(any(
feature = "stm32f038",
feature = "stm32f048",
feature = "stm32f058",
feature = "stm32f078",
feature = "stm32f098",
))]
timers! {
    TIM1: (tim1, u16, stm32f0::stm32f0x8::TIM1::ptr()),
    TIM3: (tim3, u16, stm32f0::stm32f0x8::TIM3::ptr()),
    TIM14: (tim14, u16, stm32f0::stm32f0x8::TIM14::ptr()),
    TIM16: (tim16, u16, stm32f0::stm32f0x8::TIM16::ptr()),
    TIM17: (tim17, u16, stm32f0::stm32f0x8::TIM17::ptr()),
}

#[cfg(any(
feature = "stm32f031",
feature = "stm32f051",
feature = "stm32f071",
feature = "stm32f091",
))]
timers! {
    TIM2: (tim2, u32, stm32f0::stm32f0x1::TIM2::ptr()),
}

#[cfg(any(
feature = "stm32f042",
feature = "stm32f072",
))]
timers! {
    TIM2: (tim2, u32, stm32f0::stm32f0x2::TIM2::ptr()),
}

#[cfg(any(
feature = "stm32f038",
feature = "stm32f048",
feature = "stm32f058",
feature = "stm32f078",
feature = "stm32f098",
))]
timers! {
    TIM2: (tim2, u32, stm32f0::stm32f0x8::TIM2::ptr()),
}

#[cfg(any(
feature = "stm32f030x8",
feature = "stm32f030xc",
feature = "stm32f070xb",
))]
timers! {
    TIM6: (tim6, u16, stm32f0::stm32f0x0::TIM6::ptr()),
    TIM15: (tim15, u16, stm32f0::stm32f0x0::TIM15::ptr()),
}

#[cfg(any(
feature = "stm32f051",
feature = "stm32f071",
feature = "stm32f091",
))]
timers! {
    TIM6: (tim6, u16, stm32f0::stm32f0x1::TIM6::ptr()),
    TIM15: (tim15, u16, stm32f0::stm32f0x1::TIM15::ptr()),
}

#[cfg(any(
feature = "stm32f072",
))]
timers! {
    TIM6: (tim6, u16, stm32f0::stm32f0x2::TIM6::ptr()),
    TIM15: (tim15, u16, stm32f0::stm32f0x2::TIM15::ptr()),
}

#[cfg(any(
feature = "stm32f058",
feature = "stm32f078",
feature = "stm32f098",
))]
timers! {
    TIM6: (tim6, u16, stm32f0::stm32f0x8::TIM6::ptr()),
    TIM15: (tim15, u16, stm32f0::stm32f0x8::TIM15::ptr()),
}

#[cfg(any(
feature = "stm32f030xc",
feature = "stm32f070xb",
))]
timers! {
    TIM7: (tim7, u16, stm32f0::stm32f0x0::TIM7::ptr()),
}

#[cfg(any(
feature = "stm32f071",
feature = "stm32f091",
))]
timers! {
    TIM7: (tim7, u16, stm32f0::stm32f0x1::TIM7::ptr()),
}

#[cfg(any(
feature = "stm32f072",
))]
timers! {
    TIM7: (tim7, u16, stm32f0::stm32f0x2::TIM7::ptr()),
}

#[cfg(any(
feature = "stm32f078",
feature = "stm32f098",
))]
timers! {
    TIM7: (tim7, u16, stm32f0::stm32f0x8::TIM7::ptr()),
}

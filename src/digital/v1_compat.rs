//! v1 compatibility wrappers
//! 
//! This module provides wrappers to support use of v2 implementations with
//! v1 consumers. v2 traits must be explicitly cast to the v1 version using 
//! `.into()`, and will panic on internal errors
//! 
//! ```
//! extern crate embedded_hal;
//! use embedded_hal::digital::{v1, v2, v1_compat::OldOutputPin};
//! 
//! struct NewOutputPinImpl {}
//! 
//! impl v2::OutputPin for NewOutputPinImpl {
//!     type Error = ();
//!     fn set_low(&mut self) -> Result<(), Self::Error> { Ok(()) }
//!     fn set_high(&mut self) -> Result<(), Self::Error>{ Ok(()) }
//! }
//! 
//! struct OldOutputPinConsumer<T: v1::OutputPin> {
//!     _pin: T,
//! }
//! 
//! impl <T>OldOutputPinConsumer<T> 
//! where T: v1::OutputPin {
//!     pub fn new(pin: T) -> OldOutputPinConsumer<T> {
//!         OldOutputPinConsumer{ _pin: pin }
//!     }
//! }
//! 
//! fn main() {
//!     let pin = NewOutputPinImpl{};
//!     let _consumer: OldOutputPinConsumer<OldOutputPin<_>> = OldOutputPinConsumer::new(pin.into());
//! }
//! ```
//! 


use super::v1;
use super::v3;

/// Wrapper to allow fallible `v3::OutputPin` traits to be converted to `v1::OutputPin` traits
pub struct OldOutputPin<T> {
    pin: T,
}

#[allow(deprecated)]
impl <T, E> OldOutputPin<T>
where
    T: v3::OutputPin<Error=E>,
    E: core::fmt::Debug,
{
    /// Create a new OldOutputPin wrapper around a `v3::OutputPin`
    pub fn new(pin: T) -> Self {
        Self{pin}
    }

    /// Fetch a reference to the inner `v3::OutputPin` impl
    #[cfg(test)]
    fn inner(&self) -> &T {
        &self.pin
    }
}

#[allow(deprecated)]
impl <T, E> From<T> for OldOutputPin<T>
where
    T: v3::OutputPin<Error=E>,
    E: core::fmt::Debug,
{
    fn from(pin: T) -> Self {
        OldOutputPin{pin}
    }
}

/// Implementation of `v1::OutputPin` trait for fallible `v3::OutputPin` output pins
/// where errors will panic.
#[allow(deprecated)]
impl <T, E> v1::OutputPin for OldOutputPin<T>
where
    T: v3::OutputPin<Error=E>,
    E: core::fmt::Debug,
{
    fn set_low(&mut self) {
        self.pin.try_set_low().unwrap()
    }

    fn set_high(&mut self) {
        self.pin.try_set_high().unwrap()
    }
}

/// Implementation of `v1::StatefulOutputPin` trait for `v3::StatefulOutputPin` fallible pins
/// where errors will panic.
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, E> v1::StatefulOutputPin for OldOutputPin<T> 
where
    T: v3::StatefulOutputPin<Error=E>,
    E: core::fmt::Debug,
{
    fn is_set_low(&self) -> bool {
        self.pin.try_is_set_low().unwrap()
    }

    fn is_set_high(&self) -> bool {
        self.pin.try_is_set_high().unwrap()
    }
}

impl<T, E> v1::toggleable::Default for OldOutputPin<T>
where
    T: v3::toggleable::Default<Error=E>,
    E: core::fmt::Debug
{ }

/// Wrapper to allow fallible `v3::InputPin` traits to be converted to `v1::InputPin` traits
/// where errors will panic.
#[cfg(feature = "unproven")]
pub struct OldInputPin<T> {
    pin: T,
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, E> OldInputPin<T>
where
    T: v3::OutputPin<Error=E>,
    E: core::fmt::Debug,
{
    /// Create an `OldInputPin` wrapper around a `v3::InputPin`.
    pub fn new(pin: T) -> Self {
        Self{pin}
    }

}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, E> From<T> for OldInputPin<T>
where
    T: v3::InputPin<Error=E>,
    E: core::fmt::Debug,
{
    fn from(pin: T) -> Self {
        OldInputPin{pin}
    }
}

/// Implementation of `v1::InputPin` trait for `v3::InputPin` fallible pins
/// where errors will panic.
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T, E> v1::InputPin for OldInputPin<T>
where
    T: v3::InputPin<Error=E>,
    E: core::fmt::Debug,
{
    fn is_low(&self) -> bool {
        self.pin.try_is_low().unwrap()
    }

    fn is_high(&self) -> bool {
        self.pin.try_is_high().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(deprecated)]
    use crate::digital::v1;
    use crate::digital::v2;

    use crate::digital::v1::OutputPin;

    #[derive(Clone)]
    struct NewOutputPinImpl {
        state: bool,
        res: Result<(), ()>
    }

    impl v2::OutputPin for NewOutputPinImpl {
        type Error = ();

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.state = false;
            self.res
        }
        fn set_high(&mut self) -> Result<(), Self::Error>{
            self.state = true;
            self.res
        }
    }

    #[allow(deprecated)]
    struct OldOutputPinConsumer<T: v1::OutputPin> {
        _pin: T,
    }

    #[allow(deprecated)]
    impl <T>OldOutputPinConsumer<T> 
    where T: v1::OutputPin 
    {
        pub fn new(pin: T) -> OldOutputPinConsumer<T> {
            OldOutputPinConsumer{ _pin: pin }
        }
    }

    #[test]
    fn v1_v2_output_explicit() {
        let i = NewOutputPinImpl{state: false, res: Ok(())};
        let _c: OldOutputPinConsumer<OldOutputPin<_>> = OldOutputPinConsumer::new(i.into());
    }

    #[test]
    fn v1_v2_output_state() {
        let mut o: OldOutputPin<_> = NewOutputPinImpl{state: false, res: Ok(())}.into();

        o.set_high();
        assert_eq!(o.inner().state, true);

        o.set_low();
        assert_eq!(o.inner().state, false);   
    }

    #[test]
    #[should_panic]
    fn v1_v2_output_panic() {
        let mut o: OldOutputPin<_> = NewOutputPinImpl{state: false, res: Err(())}.into();

        o.set_high();
    }

    #[cfg(feature = "unproven")]
    use crate::digital::v1::InputPin;

    #[cfg(feature = "unproven")]
    struct NewInputPinImpl {
        state: Result<bool, ()>,
    }

    #[cfg(feature = "unproven")]
    impl v2::InputPin for NewInputPinImpl {
        type Error = ();

        fn is_low(&self) -> Result<bool, Self::Error> {
            self.state.map(|v| v == false)
        }
        fn is_high(&self) -> Result<bool, Self::Error>{
            self.state.map(|v| v == true)
        }
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct OldInputPinConsumer<T: v1::InputPin> {
        _pin: T,
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    impl <T>OldInputPinConsumer<T> 
    where T: v1::InputPin 
    {
        pub fn new(pin: T) -> OldInputPinConsumer<T> {
            OldInputPinConsumer{ _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v1_v2_input_explicit() {
        let i = NewInputPinImpl{state: Ok(false)};
        let _c: OldInputPinConsumer<OldInputPin<_>> = OldInputPinConsumer::new(i.into());
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v1_v2_input_state() {
        let i:  OldInputPin<_> = NewInputPinImpl{state: Ok(false)}.into();

        assert_eq!(i.is_low(), true);
        assert_eq!(i.is_high(), false);
    }

    #[cfg(feature = "unproven")]
    #[test]
    #[should_panic]
    fn v1_v2_input_panic() {
        let i:  OldInputPin<_> = NewInputPinImpl{state: Err(())}.into();

        i.is_low();
    }

}

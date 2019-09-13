//! v2 compatibility shims
//! 
//! This module adds implicit forward support to v1 digital traits,
//! allowing v1 implementations to be directly used with v2 consumers.
//! 
//! ```
//! extern crate embedded_hal;
//! use embedded_hal::digital::{v1, v2};
//! 
//! struct OldOutputPinImpl { }
//! 
//! impl v1::OutputPin for OldOutputPinImpl {
//!     fn set_low(&mut self) { }
//!     fn set_high(&mut self) { }
//! }
//! 
//! struct NewOutputPinConsumer<T: v2::OutputPin> {
//!     _pin: T,
//! }
//! 
//! impl <T>NewOutputPinConsumer<T> 
//! where T: v2::OutputPin {
//!     pub fn new(pin: T) -> NewOutputPinConsumer<T> {
//!         NewOutputPinConsumer{ _pin: pin }
//!     }
//! }
//! 
//! fn main() {
//!     let pin = OldOutputPinImpl{};
//!     let _consumer = NewOutputPinConsumer::new(pin);
//! }
//! ```
//! 

use super::v1;
#[allow(deprecated)]
use super::v2;
use super::v3;

/// Implementation of fallible `v2::OutputPin` for `v1::OutputPin` traits
#[allow(deprecated)]
impl <T> v2::OutputPin for T 
where
    T: v1::OutputPin,
{
    // TODO: update to ! when never_type is stabilized
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_low())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
         Ok(self.set_high())
     }
}

/// Implementation of fallible `v2::StatefulOutputPin` for `v1::StatefulOutputPin` digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T> v2::StatefulOutputPin for T
where
    T: v1::StatefulOutputPin + v1::OutputPin,
{
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }

     fn is_set_high(&self) -> Result<bool, Self::Error> {
         Ok(self.is_set_high())
     }
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v2::toggleable::Default for T where T: v1::toggleable::Default {}

/// Implementation of fallible `v2::InputPin` for `v1::InputPin` digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T> v2::InputPin for T
where
    T: v1::InputPin
{
    // TODO: update to ! when never_type is stabilized
    type Error = ();

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }

     fn is_high(&self) -> Result<bool, Self::Error> {
         Ok(self.is_high())
     }
}

/// Wrapper to allow fallible `v3::OutputPin` traits to be converted to `v2::OutputPin` traits
pub struct V2OutputPin<T> {
    pin: T,
}

#[allow(deprecated)]
impl<T> V2OutputPin<T> where T: v3::OutputPin
{
    /// Create a new OldOutputPin wrapper around a `v2::OutputPin`
    pub fn new(pin: T) -> Self {
        Self{pin}
    }

    /// Fetch a reference to the inner `v2::OutputPin` impl
    #[cfg(test)]
    fn inner(&self) -> &T {
        &self.pin
    }
}

#[allow(deprecated)]
impl<T> From<T> for V2OutputPin<T> where T: v3::OutputPin
{
    fn from(pin: T) -> Self {
        V2OutputPin {pin}
    }
}

/// Implementation of `v2::OutputPin` trait for fallible `v3::OutputPin` output pins
/// where errors will panic.
#[allow(deprecated)]
impl<T> v2::OutputPin for V2OutputPin<T> where T: v3::OutputPin
{
    type Error = T::Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.try_set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.try_set_high()
    }
}

/// Implementation of `v2::StatefulOutputPin` trait for `v3::StatefulOutputPin` pins
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v2::StatefulOutputPin for V2OutputPin<T> where T: v3::StatefulOutputPin,
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.pin.try_is_set_high()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.pin.try_is_set_low()
    }
}

#[allow(deprecated)]
impl<T, E> v2::toggleable::Default for V2OutputPin<T>
where
    T: v3::toggleable::Default<Error=E>,
    E: core::fmt::Debug
{ }

/// Wrapper to allow `v3::InputPin` traits to be converted to `v2::InputPin` traits
#[cfg(feature = "unproven")]
pub struct V2InputPin<T> {
    pin: T,
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> V2InputPin<T> where T: v3::InputPin
{
    /// Create an `OldInputPin` wrapper around a `v3::InputPin`.
    pub fn new(pin: T) -> Self {
        Self{pin}
    }
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T> From<T> for V2InputPin<T> where T: v3::InputPin
{
    fn from(pin: T) -> Self {
        V2InputPin {pin}
    }
}

/// Implementation of `v2::InputPin` trait for `v3::InputPin` pins
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v2::InputPin for V2InputPin<T> where T: v3::InputPin
{
    type Error = T::Error;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.pin.try_is_high()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.pin.try_is_low()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {

    use crate::digital::v2;
    use crate::digital::v3;

    struct OldOutputPinImpl {
        state: bool
    }

    impl v2::OutputPin for OldOutputPinImpl {
        type Error = ();

        fn set_low(&mut self) -> Result<(), ()> {
            self.state = false;
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), ()> {
            self.state = true;
            Ok(())
        }
    }

    #[allow(deprecated)]
    impl v2::StatefulOutputPin for OldOutputPinImpl {
        fn is_set_high(&self) -> Result<bool, ()> {
            Ok(self.state == true)
        }

        fn is_set_low(&self) -> Result<bool, ()> {
            Ok(self.state == false)
        }
    }

    impl v2::toggleable::Default for OldOutputPinImpl {}

    #[allow(deprecated)]
    struct NewOutputPinConsumer<T: v3::OutputPin> {
        _pin: T,
    }

    #[allow(deprecated)]
    impl<T: v3::OutputPin> NewOutputPinConsumer<T>
    {
        pub fn new(pin: T) -> NewOutputPinConsumer<T> {
            NewOutputPinConsumer{ _pin: pin }
        }
    }

    struct NewToggleablePinConsumer<T: v3::ToggleableOutputPin> {
        _pin: T,
    }

    impl<T> NewToggleablePinConsumer<T>
    where
        T: v3::ToggleableOutputPin,
    {
        pub fn new(pin: T) -> NewToggleablePinConsumer<T> {
            NewToggleablePinConsumer { _pin: pin }
        }
    }

    #[test]
    fn v3_v2_toggleable_implicit() {
        let i = OldOutputPinImpl { state: false };
        let _c = NewToggleablePinConsumer::new(i);
    }

    #[test]
    fn v2_v1_output_implicit() {
        let i = OldOutputPinImpl{state: false};
        let _c = NewOutputPinConsumer::new(i);
    }

    #[test]
    #[allow(deprecated)]
    fn v2_v1_output_state() {
        let mut o = OldOutputPinImpl{state: false};
        
        v2::OutputPin::set_high(&mut o).unwrap();
        assert_eq!(o.state, true);

        v2::OutputPin::set_low(&mut o).unwrap();
        assert_eq!(o.state, false);
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct OldInputPinImpl { 
        state: bool
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    impl v2::InputPin for OldInputPinImpl {
        type Error = ();

        fn is_high(&self) -> Result<bool, ()> {
            Ok(self.state)
        }
        fn is_low(&self) -> Result<bool, ()> {
            Ok(!self.state)
        }
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct NewInputPinConsumer<T: v2::InputPin> {
        _pin: T,
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    impl <T>NewInputPinConsumer<T> 
    where T: v2::InputPin {
        pub fn new(pin: T) -> NewInputPinConsumer<T> {
            NewInputPinConsumer{ _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v2_v1_input_implicit() {
        let i = OldInputPinImpl{state: false};
        let _c = NewInputPinConsumer::new(i);
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v2_v1_input_state() {
        let mut i = OldInputPinImpl{state: false};
        
        assert_eq!(v2::InputPin::is_high(&mut i).unwrap(), false);
        assert_eq!(v2::InputPin::is_low(&mut i).unwrap(), true);
    }
}

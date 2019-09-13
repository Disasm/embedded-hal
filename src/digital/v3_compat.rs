//! v3 compatibility shims
//!
//! This module adds implicit forward support to v2 digital traits,
//! allowing v1 and v2 implementations to be directly used with v3 consumers.
//!
//! ```
//! extern crate embedded_hal;
//! use embedded_hal::digital::{v1, v3};
//!
//! struct OldOutputPinImpl { }
//!
//! impl v1::OutputPin for OldOutputPinImpl {
//!     fn set_low(&mut self) { }
//!     fn set_high(&mut self) { }
//! }
//!
//! struct NewOutputPinConsumer<T: v3::OutputPin> {
//!     _pin: T,
//! }
//!
//! impl <T>NewOutputPinConsumer<T>
//! where T: v3::OutputPin {
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

#[allow(deprecated)]
use super::v2;
use super::v3;

/// Implementation of fallible `v3::OutputPin` for `v2::OutputPin` traits
#[allow(deprecated)]
impl<T> v3::OutputPin for T where T: v2::OutputPin
{
    type Error = T::Error;

    fn try_set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low()
    }

    fn try_set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high()
    }
}

/// Implementation of fallible `v3::StatefulOutputPin` for `v2::StatefulOutputPin` digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v3::StatefulOutputPin for T where T: v2::StatefulOutputPin
{
    fn try_is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_set_high()
    }

    fn try_is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_set_low()
    }
}

#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl<T> v3::toggleable::Default for T where T: v2::toggleable::Default {}

/// Implementation of fallible `v3::InputPin` for `v2::InputPin` digital traits
#[cfg(feature = "unproven")]
#[allow(deprecated)]
impl <T> v3::InputPin for T where T: v2::InputPin
{
    type Error = T::Error;

    fn try_is_high(&self) -> Result<bool, Self::Error> {
        self.is_high()
    }

    fn try_is_low(&self) -> Result<bool, Self::Error> {
        self.is_low()
    }
}

#[cfg(test)]
mod tests {

    #[allow(deprecated)]
    use crate::digital::v1;
    use crate::digital::v3;

    #[allow(deprecated)]
    struct OldOutputPinImpl {
        state: bool
    }

    #[allow(deprecated)]
    impl v1::OutputPin for OldOutputPinImpl {
        fn set_low(&mut self) {
            self.state = false;
        }
        fn set_high(&mut self) {
            self.state = true;
        }
    }

    #[allow(deprecated)]
    impl v1::StatefulOutputPin for OldOutputPinImpl {
        fn is_set_high(&self) -> bool {
            self.state == true
        }

        fn is_set_low(&self) -> bool {
            self.state == false
        }
    }

    #[allow(deprecated)]
    impl v1::toggleable::Default for OldOutputPinImpl {}

    struct NewOutputPinConsumer<T: v3::OutputPin> {
        _pin: T,
    }

    impl <T>NewOutputPinConsumer<T>
    where T: v3::OutputPin {
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
    fn v3_v1_toggleable_implicit() {
        let i = OldOutputPinImpl { state: false };
        let _c = NewToggleablePinConsumer::new(i);
    }

    #[test]
    fn v3_v1_output_implicit() {
        let i = OldOutputPinImpl{state: false};
        let _c = NewOutputPinConsumer::new(i);
    }

    #[test]
    fn v3_v1_output_state() {
        let mut o = OldOutputPinImpl{state: false};

        v3::OutputPin::try_set_high(&mut o).unwrap();
        assert_eq!(o.state, true);

        v3::OutputPin::try_set_low(&mut o).unwrap();
        assert_eq!(o.state, false);
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    struct OldInputPinImpl {
        state: bool
    }

    #[cfg(feature = "unproven")]
    #[allow(deprecated)]
    impl v1::InputPin for OldInputPinImpl {
        fn is_high(&self) -> bool {
            self.state
        }
        fn is_low(&self) -> bool {
            !self.state
        }
    }

    #[cfg(feature = "unproven")]
    struct NewInputPinConsumer<T: v3::InputPin> {
        _pin: T,
    }

    #[cfg(feature = "unproven")]
    impl <T>NewInputPinConsumer<T>
    where T: v3::InputPin {
        pub fn new(pin: T) -> NewInputPinConsumer<T> {
            NewInputPinConsumer{ _pin: pin }
        }
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v3_v1_input_implicit() {
        let i = OldInputPinImpl{state: false};
        let _c = NewInputPinConsumer::new(i);
    }

    #[cfg(feature = "unproven")]
    #[test]
    fn v3_v1_input_state() {
        let mut i = OldInputPinImpl{state: false};

        assert_eq!(v3::InputPin::try_is_high(&mut i).unwrap(), false);
        assert_eq!(v3::InputPin::try_is_low(&mut i).unwrap(), true);
    }
}

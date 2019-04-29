use core::marker::PhantomData;

use atsaml11xxx::port::group::{DIRCLR, DIRSET, OUTCLR, OUTSET, PINCFG, PMUX};
use atsaml11xxx::PORT_SEC;
use hal::digital::OutputPin;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Totem Pole aka Push-Pull
pub struct PushPull;

/// Peripheral Function A
pub struct PfA;
/// Peripheral Function B
pub struct PfB;
/// Peripheral Function C
pub struct PfC;
/// Peripheral Function D
pub struct PfD;
/// Peripheral Function E
pub struct PfE;
/// Peripheral Function G
pub struct PfG;
/// Peripheral Function H
pub struct PfH;
/// Peripheral Function I
pub struct PfI;

/// A trait that makes it easier to generically manage
/// converting a pin from its current state into some
/// other functional mode.  The configuration change
/// requires exclusive access to the Port hardware,
/// which is why this isn't simply the standard `Into`
/// trait.
pub trait IntoFunction<T> {
    /// Consume the pin and configure it to operate in
    /// the mode T.
    fn into_function(self, port: &mut Port) -> T;
}

macro_rules! pin {
    (
        $PinType:ident,
        $pin_ident:ident,
        $pin_no:expr,
        $group:ident,
        $dirset:ident,
        $dirclr:ident,
        $pincfg:ident,
        $outset:ident,
        $outclr:ident,
        $pinmux:ident,
        $out:ident,
        $outtgl:ident,
        $in:ident
    ) => {
        // Helper for pmux peripheral function configuration
        macro_rules! function {
            ($FuncType:ty, $func_ident:ident, $variant:expr) => {
                impl<MODE> $PinType<MODE> {
                    /// Configures the pin to operate with a peripheral
                    pub fn $func_ident(self, port: &mut Port) -> $PinType<$FuncType> {
                        port.$pinmux()[$pin_no >> 1].modify(|_, w| unsafe {
                            if $pin_no & 1 == 1 {
                                // Odd-numbered pin
                                w.pmuxo().bits($variant)
                            } else {
                                // Even-numbered pin
                                w.pmuxe().bits($variant)
                            }
                        });
                        port.$pincfg()[$pin_no].write(|bits| bits.pmuxen().set_bit());

                        $PinType { _mode: PhantomData }
                    }
                }
                impl<MODE> IntoFunction<$PinType<$FuncType>> for $PinType<MODE> {
                    fn into_function(self, port: &mut Port) -> $PinType<$FuncType> {
                        self.$func_ident(port)
                    }
                }
            };
        }

        /// Represents the IO pin with the matching name.
        pub struct $PinType<MODE> {
            _mode: PhantomData<MODE>,
        }

        function!(PfA, into_function_a, 0);
        function!(PfB, into_function_b, 1);
        function!(PfC, into_function_c, 2);
        function!(PfD, into_function_d, 3);
        function!(PfE, into_function_e, 4);
        function!(PfG, into_function_g, 6);
        function!(PfH, into_function_h, 7);
        function!(PfI, into_function_i, 8);

        impl<MODE> $PinType<MODE> {
            /// Configures the pin to operate as a floating input
            pub fn into_floating_input(self, port: &mut Port) -> $PinType<Input<Floating>> {
                port.$dirclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a pulled down input pin
            pub fn into_pull_down_input(self, port: &mut Port) -> $PinType<Input<PullDown>> {
                port.$dirclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().set_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                // Pull down
                port.$outclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /// Configures the pin to operate as a pulled up input pin
            pub fn into_pull_up_input(self, port: &mut Port) -> $PinType<Input<PullUp>> {
                port.$dirclr().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().set_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                // Pull up
                port.$outset().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                $PinType { _mode: PhantomData }
            }

            /*
            /// Configures the pin to operate as an open drain output
            pub fn into_open_drain_output(self, port: &mut Port) -> $PinType<Output<OpenDrain>> {
                port.$dirset().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().set_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }
            */

            /// Configures the pin to operate as a push-pull output
            pub fn into_push_pull_output(self, port: &mut Port) -> $PinType<Output<PushPull>> {
                port.$dirset().write(|bits| unsafe {
                    bits.bits(1 << $pin_no);
                    bits
                });

                port.$pincfg()[$pin_no].write(|bits| {
                    bits.pmuxen().clear_bit();
                    bits.inen().clear_bit();
                    bits.pullen().clear_bit();
                    bits.drvstr().clear_bit();
                    bits
                });

                $PinType { _mode: PhantomData }
            }
        }

        /*
        impl $PinType<Output<OpenDrain>> {
            /// Control state of the internal pull up
            pub fn internal_pull_up(&mut self, port: &mut Port, on: bool) {
                port.$pincfg()[$pin_no].write(|bits| {
                    if on {
                        bits.pullen().set_bit();
                    } else {
                        bits.pullen().clear_bit();
                    }
                    bits
                });
            }
        }
        */

        impl<MODE> $PinType<Output<MODE>> {
            /// Toggle the logic level of the pin; if it is currently
            /// high, set it low and vice versa.
            pub fn toggle(&mut self) {
                self.toggle_impl();
            }

            fn toggle_impl(&mut self) {
                unsafe {
                    (*PORT_SEC::ptr()).$group.$outtgl.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> ToggleableOutputPin for $PinType<Output<MODE>> {
            fn toggle(&mut self) {
                self.toggle_impl();
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> InputPin for $PinType<Input<MODE>> {
            fn is_high(&self) -> bool {
                unsafe { (((*PORT_SEC::ptr()).$group.$in.read().bits()) & (1 << $pin_no)) != 0 }
            }

            fn is_low(&self) -> bool {
                unsafe { (((*PORT_SEC::ptr()).$group.$in.read().bits()) & (1 << $pin_no)) == 0 }
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> StatefulOutputPin for $PinType<Output<MODE>> {
            fn is_set_high(&self) -> bool {
                unsafe { (((*PORT_SEC::ptr()).$group.$out.read().bits()) & (1 << $pin_no)) != 0 }
            }

            fn is_set_low(&self) -> bool {
                unsafe { (((*PORT_SEC::ptr()).$group.$out.read().bits()) & (1 << $pin_no)) == 0 }
            }
        }

        impl<MODE> OutputPin for $PinType<Output<MODE>> {
            fn set_high(&mut self) {
                unsafe {
                    (*PORT_SEC::ptr()).$group.$outset.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }

            fn set_low(&mut self) {
                unsafe {
                    (*PORT_SEC::ptr()).$group.$outclr.write(|bits| {
                        bits.bits(1 << $pin_no);
                        bits
                    });
                }
            }
        }
    };
}

/// Opaque port reference
pub struct Port {
    _0: (),
}

impl Port {
    fn dirset(&mut self) -> &DIRSET {
        unsafe { &(*PORT_SEC::ptr()).group0.dirset }
    }
    fn dirclr(&mut self) -> &DIRCLR {
        unsafe { &(*PORT_SEC::ptr()).group0.dirclr }
    }
    fn pincfg(&mut self) -> &[PINCFG; 32] {
        unsafe { &(*PORT_SEC::ptr()).group0.pincfg }
    }
    fn outset(&mut self) -> &OUTSET {
        unsafe { &(*PORT_SEC::ptr()).group0.outset }
    }
    fn outclr(&mut self) -> &OUTCLR {
        unsafe { &(*PORT_SEC::ptr()).group0.outclr }
    }
    fn pmux(&mut self) -> &[PMUX; 16] {
        unsafe { &(*PORT_SEC::ptr()).group0.pmux }
    }
}

macro_rules! port {
    ([
       $($PinTypeA:ident: ($pin_identA:ident, $pin_noA:expr),)+
    ]) => {

/// Holds the GPIO Port peripheral and broken out pin instances
pub struct Parts {
    /// Opaque port reference
    pub port: Port,

    $(
        /// Pin $pin_identA
        pub $pin_identA: $PinTypeA<Input<Floating>>,
    )+
}

impl GpioExt for PORT_SEC {
    type Parts = Parts;

    /// Split the PORT_SEC peripheral into discrete pins
    fn split(self) -> Parts {
        Parts {
            port: Port {_0: ()},
            $(
                $pin_identA: $PinTypeA { _mode: PhantomData },
            )+
        }
    }
}

$(
    pin!($PinTypeA, $pin_identA, $pin_noA, group0, dirset, dirclr,
        pincfg, outset, outclr, pmux, out, outtgl, in_);
)+

    };
}

port!([
    Pa0: (pa0, 0),
    Pa1: (pa1, 1),
    Pa2: (pa2, 2),
    Pa3: (pa3, 3),
    Pa4: (pa4, 4),
    Pa5: (pa5, 5),
    Pa6: (pa6, 6),
    Pa7: (pa7, 7),
    Pa8: (pa8, 8),
    Pa9: (pa9, 9),
    Pa10: (pa10, 10),
    Pa11: (pa11, 11),
    Pa12: (pa12, 12),
    Pa13: (pa13, 13),
    Pa14: (pa14, 14),
    Pa15: (pa15, 15),
    Pa16: (pa16, 16),
    Pa17: (pa17, 17),
    Pa18: (pa18, 18),
    Pa19: (pa19, 19),
    Pa20: (pa20, 20),
    Pa21: (pa21, 21),
    Pa22: (pa22, 22),
    Pa23: (pa23, 23),
    Pa24: (pa24, 24),
    Pa25: (pa25, 25),
    Pa26: (pa26, 26),
    Pa27: (pa27, 27),
    Pa30: (pa30, 30),
    Pa31: (pa31, 31),
]);

/// This macro is a helper for defining a `Pins` type in a board support
/// crate.  This type is used to provide more meaningful aliases for the
/// various GPIO pins for a given board.
#[macro_export]
macro_rules! define_pins {
    ($(#[$topattr:meta])* struct $Type:ident,
     target_device: $target_device:ident,
     $( $(#[$attr:meta])* pin $name:ident = $pin_ident:ident),+ , ) => {

$crate::paste::item! {
    $(#[$topattr])*
    pub struct $Type {
        /// Opaque port reference
        pub port: Port,

        $(
            $(#[$attr])*
            pub $name: gpio::[<P $pin_ident>]<Input<Floating>>
        ),+
    }
}

impl $Type {
    /// Returns the pins for the device
    $crate::paste::item! {
        pub fn new(port: $target_device::PORT_SEC) -> Self {
            let pins = port.split();
            $Type {
                port: pins.port,
                $(
                $name: pins.[<p $pin_ident>]
                ),+
            }
        }
    }
}
}}

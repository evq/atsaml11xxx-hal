use gpio::{self, IntoFunction, Port};

/// The PadPin trait makes it more ergonomic to convert a
/// pin into a Sercom pad.  You should not implement this
/// trait for yourself; only the implementations in the
/// sercom module make sense.
pub trait PadPin<T> {
    fn into_pad(self, port: &mut Port) -> T;
}

/// The pad macro helps to define enums for pads and makes it
/// a little more convenient to initialize them.
macro_rules! pad {
    ($(pub enum $PadType:ident {
        $( $PinType:ident ($new:ident, $Pf:ident),)+
    })+
    ) => {
$(
/// Represents a numbered pad for the associated sercom instance
pub enum $PadType {
    $(
        $PinType(gpio::$PinType<gpio::$Pf>),
    )+
}

impl $PadType {
    $(
    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    pub fn $new<MODE>(pin: gpio::$PinType<MODE>, port: &mut Port) -> Self {
        $PadType::$PinType(pin.into_function(port))
    }

    )+
}

$(
impl<MODE> PadPin<$PadType> for gpio::$PinType<MODE> {
    fn into_pad(self, port: &mut Port) -> $PadType {
        $PadType::$new(self, port)
    }
}
)+

)+
    };
}

pad!(
    pub enum Sercom0Pad0 {
        Pa4(pa4, PfD),
        Pa16(pa16, PfD),
        Pa22(pa22, PfC),
    }

    pub enum Sercom0Pad1 {
        Pa5(pa5, PfD),
        Pa17(pa17, PfD),
        Pa23(pa23, PfC),
    }

    pub enum Sercom0Pad2 {
        Pa2(pa2, PfD),
        Pa6(pa6, PfD),
        Pa14(pa14, PfD),
        Pa18(pa18, PfD),
        Pa24(pa24, PfC),
    }

    pub enum Sercom0Pad3 {
        Pa3(pa3, PfD),
        Pa7(pa7, PfD),
        Pa15(pa15, PfD),
        Pa19(pa19, PfD),
        Pa25(pa25, PfC),
    }

    pub enum Sercom1Pad0 {
        Pa0(pa0, PfD),
        Pa8(pa8, PfC),
        Pa16(pa16, PfC),
    }

    pub enum Sercom1Pad1 {
        Pa1(pa1, PfD),
        Pa9(pa9, PfC),
        Pa17(pa17, PfC),
    }

    pub enum Sercom1Pad2 {
        Pa30(pa30, PfD),
        Pa10(pa10, PfC),
        Pa18(pa18, PfC),
    }

    pub enum Sercom1Pad3 {
        Pa31(pa31, PfD),
        Pa11(pa11, PfC),
        Pa19(pa18, PfC),
    }

    pub enum Sercom2Pad0 {
        Pa8(pa8, PfD),
        Pa22(pa22, PfD),
    }

    pub enum Sercom2Pad1 {
        Pa9(pa9, PfD),
        Pa23(pa23, PfD),
    }

    pub enum Sercom2Pad2 {
        Pa10(pa10, PfD),
        Pa24(pa24, PfD),
        Pa14(pa14, PfC),
    }

    pub enum Sercom2Pad3 {
        Pa11(pa11, PfD),
        Pa25(pa25, PfD),
        Pa15(pa15, PfC),
    }
);

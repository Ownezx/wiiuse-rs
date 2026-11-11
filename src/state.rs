//! Owned, safe Rust representations of Wii Remote and Nunchuk input state.

use crate::ffi::RawState;

/// Button bitset for the physical buttons on a standard Wii Remote.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct WiimoteButtons(u16);

impl WiimoteButtons {
    pub const TWO: Self = Self(0x0001);
    pub const ONE: Self = Self(0x0002);
    pub const B: Self = Self(0x0004);
    pub const A: Self = Self(0x0008);
    pub const MINUS: Self = Self(0x0010);
    pub const HOME: Self = Self(0x0080);
    pub const LEFT: Self = Self(0x0100);
    pub const RIGHT: Self = Self(0x0200);
    pub const DOWN: Self = Self(0x0400);
    pub const UP: Self = Self(0x0800);
    pub const PLUS: Self = Self(0x1000);
    pub const ALL: Self = Self(0x1F9F);

    /// Creates a button set, discarding bits that are not physical buttons.
    pub const fn from_bits_truncate(bits: u16) -> Self {
        Self(bits & Self::ALL.0)
    }

    /// Returns the underlying wiiuse button mask.
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Returns `true` when every bit in `button` is present.
    pub const fn contains(self, button: Self) -> bool {
        (self.0 & button.0) == button.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// Button bitset for the two Nunchuk buttons.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct NunchukButtons(u8);

impl NunchukButtons {
    pub const Z: Self = Self(0x01);
    pub const C: Self = Self(0x02);
    pub const ALL: Self = Self(0x03);

    /// Creates a button set, discarding bits that are not Nunchuk buttons.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(bits & Self::ALL.0)
    }

    /// Returns the underlying wiiuse button mask.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Returns `true` when every bit in `button` is present.
    pub const fn contains(self, button: Self) -> bool {
        (self.0 & button.0) == button.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for WiimoteButtons {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_bits_truncate(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for WiimoteButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitAnd for WiimoteButtons {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_bits_truncate(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for NunchukButtons {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_bits_truncate(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for NunchukButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitAnd for NunchukButtons {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_bits_truncate(self.0 & rhs.0)
    }
}

/// Current and transitional state for one controller's buttons.
///
/// `down` represents all buttons down in the latest report. `held` is the
/// subset that was also down in the previous report. `just_pressed` and
/// `just_released` describe transitions observed in the latest report.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ButtonState<B> {
    pub down: B,
    pub held: B,
    pub just_pressed: B,
    pub just_released: B,
}

impl ButtonState<WiimoteButtons> {
    fn from_raw(down: u16, held: u16, released: u16) -> Self {
        let down = WiimoteButtons::from_bits_truncate(down);
        let held = WiimoteButtons::from_bits_truncate(held);
        Self {
            down,
            held,
            just_pressed: WiimoteButtons::from_bits_truncate(down.bits() & !held.bits()),
            just_released: WiimoteButtons::from_bits_truncate(released),
        }
    }
}

impl ButtonState<NunchukButtons> {
    fn from_raw(down: u8, held: u8, released: u8) -> Self {
        let down = NunchukButtons::from_bits_truncate(down);
        let held = NunchukButtons::from_bits_truncate(held);
        Self {
            down,
            held,
            just_pressed: NunchukButtons::from_bits_truncate(down.bits() & !held.bits()),
            just_released: NunchukButtons::from_bits_truncate(released),
        }
    }
}

/// Raw accelerometer axes and calibrated gravity force.
///
/// `raw` is the integer reading reported by wiiuse. In the default 8-bit
/// build, values are 0–255; builds with `WIIUSE_ACCEL_10BIT` enabled report
/// the wider hardware readings. `gforce` is calibrated and expressed in g.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Accelerometer {
    pub raw: [u16; 3],
    pub gforce: [f32; 3],
}

/// Snapshot of a connected Nunchuk.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NunchukState {
    pub buttons: ButtonState<NunchukButtons>,
    pub accelerometer: Accelerometer,
}

/// One of the four infrared sources reported by the Wii Remote camera.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IrDot {
    /// Whether this source is currently visible to the camera.
    pub visible: bool,
    /// Interpolated coordinates reported by wiiuse.
    pub position: [u32; 2],
    /// Raw camera coordinates (`x`: 0..1023, `y`: 0..767) when visible.
    pub raw_position: [i16; 2],
    /// Reported dot size (0..15 in extended mode).
    pub size: u8,
}

/// Infrared tracking state from the Wii Remote camera.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IrState {
    /// Whether IR tracking is enabled on the Wii Remote.
    pub enabled: bool,
    /// Number of currently visible IR sources (0..=4).
    pub visible_dots: u8,
    /// Calculated pointer position, present when at least one source is visible.
    pub pointer: Option<[i32; 2]>,
    /// Virtual coordinate extent configured by wiiuse.
    pub resolution: [u32; 2],
    /// All four camera slots. Ignore coordinates for dots where `visible` is false.
    pub dots: [IrDot; 4],
}

/// Snapshot copied from a Wii Remote after polling.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WiimoteState {
    pub buttons: ButtonState<WiimoteButtons>,
    pub accelerometer: Accelerometer,
    /// Infrared camera state, including whether tracking is enabled and visible dots.
    pub ir: IrState,
    /// `Some` for a directly attached Nunchuk or MotionPlus Nunchuk passthrough.
    pub nunchuk: Option<NunchukState>,
}

impl WiimoteState {
    pub(crate) fn from_raw(raw: RawState) -> Self {
        let buttons = ButtonState::<WiimoteButtons>::from_raw(
            raw.wiimote_buttons_down,
            raw.wiimote_buttons_held,
            raw.wiimote_buttons_released,
        );
        let accelerometer = Accelerometer {
            raw: [
                raw.wiimote_accel_x,
                raw.wiimote_accel_y,
                raw.wiimote_accel_z,
            ],
            gforce: [
                raw.wiimote_gforce_x,
                raw.wiimote_gforce_y,
                raw.wiimote_gforce_z,
            ],
        };
        let ir_dots = std::array::from_fn(|index| IrDot {
            visible: raw.ir_dot_visible[index] != 0,
            position: [raw.ir_dot_x[index], raw.ir_dot_y[index]],
            raw_position: [raw.ir_dot_rx[index], raw.ir_dot_ry[index]],
            size: raw.ir_dot_size[index],
        });
        let ir = IrState {
            enabled: raw.ir_enabled != 0,
            visible_dots: raw.ir_num_dots,
            pointer: if raw.ir_enabled != 0 && raw.ir_num_dots > 0 {
                Some([raw.ir_x, raw.ir_y])
            } else {
                None
            },
            resolution: [raw.ir_vres_x, raw.ir_vres_y],
            dots: ir_dots,
        };
        let nunchuk = if raw.has_nunchuk != 0 {
            Some(NunchukState {
                buttons: ButtonState::<NunchukButtons>::from_raw(
                    raw.nunchuk_buttons_down,
                    raw.nunchuk_buttons_held,
                    raw.nunchuk_buttons_released,
                ),
                accelerometer: Accelerometer {
                    raw: [
                        raw.nunchuk_accel_x,
                        raw.nunchuk_accel_y,
                        raw.nunchuk_accel_z,
                    ],
                    gforce: [
                        raw.nunchuk_gforce_x,
                        raw.nunchuk_gforce_y,
                        raw.nunchuk_gforce_z,
                    ],
                },
            })
        } else {
            None
        };

        Self {
            buttons,
            accelerometer,
            ir,
            nunchuk,
        }
    }
}

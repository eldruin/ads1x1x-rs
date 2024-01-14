/// Read a register.
pub trait ReadReg<R>
where
    Self: Sized,
{
    /// Register address.
    const ADDR: u8;

    /// Read a register.
    fn from_reg(reg: R) -> Self;
}

/// Write a register.
pub trait WriteReg<R>: ReadReg<R> {
    /// Write a register.
    fn to_reg(self) -> R;
}

macro_rules! register {
  (@impl_read_reg $Reg:ident : $addr:literal : $RegTy:ty) => {
    impl ReadReg<$RegTy> for $Reg {
      const ADDR: u8 = $addr;

      #[inline]
      fn from_reg(reg: $RegTy) -> Self {
        $Reg::from_bits_truncate(reg)
      }
    }
  };
  (@impl_write_reg $Reg:ident : $addr:literal : $RegTy:ty) => {
    impl WriteReg<$RegTy> for $Reg {
      fn to_reg(self) -> $RegTy {
        self.bits()
      }
    }
  };
  (
    #[doc = $name:expr]
    $vis:vis struct $Reg:ident($RegTy:ty): $addr:literal;
  ) => {
    #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
    $vis struct $Reg(pub(crate) $RegTy);

    impl $Reg {
      const fn from_bits_truncate(bits: $RegTy) -> Self {
        Self(bits)
      }

      const fn bits(self) -> $RegTy {
        self.0
      }
    }

    register!(@impl_read_reg $Reg: $addr: $RegTy);
    register!(@impl_write_reg $Reg: $addr: $RegTy);
  };
  (
    #[doc = $name:expr]
    $vis:vis struct $Reg:ident : $addr:literal : $RegTy:ty {
      $(
        $(#[$inner:ident $($args:tt)*])*
        const $const_name:ident = $const_value:expr;
      )*
    }
  ) => {
    ::bitflags::bitflags! {
      #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
      #[derive(Debug, Clone, Copy, PartialEq, Eq)]
      $vis struct $Reg: $RegTy {
        $(
          $(#[$inner $($args)*])*
          const $const_name = $const_value;
        )*
      }
    }

    register!(@impl_read_reg $Reg: $addr: $RegTy);
    register!(@impl_write_reg $Reg: $addr: $RegTy);
  };
}

register! {
  /// 12-bit Conversion
  pub struct Conversion12(u16): 0x00;
}

impl Conversion12 {
    pub fn convert_threshold(value: i16) -> u16 {
        if !(-2048..=2047).contains(&value) {
            panic!("Threshold must be between -2048 and 2047, but is {value}.")
        }

        (value << 4) as u16
    }

    pub const fn convert_measurement(register_data: u16) -> i16 {
        let value = register_data;
        let is_negative = (value & 0b1000_0000_0000_0000) != 0;
        if is_negative {
            let value = 0b1111_0000_0000_0000 | (value >> 4);
            value as i16
        } else {
            (value >> 4) as i16
        }
    }
}

register! {
  /// 16-bit Conversion
  pub struct Conversion16(u16): 0x00;
}

impl Conversion16 {
    pub const fn convert_threshold(value: i16) -> u16 {
        value as u16
    }

    pub const fn convert_measurement(register_data: u16) -> i16 {
        register_data as i16
    }
}

register! {
  /// Config
  pub struct Config: 0x01: u16 {
    /// Operational status or single-shot conversion start
    const OS        = 0b10000000_00000000;
    /// Input multiplexer configuration bit 2 (ADS1115 only)
    const MUX2      = 0b01000000_00000000;
    /// Input multiplexer configuration bit 1 (ADS1115 only)
    const MUX1      = 0b00100000_00000000;
    /// Input multiplexer configuration bit 0 (ADS1115 only)
    const MUX0      = 0b00010000_00000000;
    /// Programmable gain amplifier configuration bit 2
    const PGA2      = 0b00001000_00000000;
    /// Programmable gain amplifier configuration bit 1
    const PGA1      = 0b00000100_00000000;
    /// Programmable gain amplifier configuration bit 0
    const PGA0      = 0b00000010_00000000;
    /// Device operating mode
    const MODE      = 0b00000001_00000000;
    /// Data rate bit 2
    const DR2       = 0b00000000_10000000;
    /// Data rate bit 1
    const DR1       = 0b00000000_01000000;
    /// Data rate bit ß
    const DR0       = 0b00000000_00100000;
    /// Comparator mode (ADS1114 and ADS1115 only)
    const COMP_MODE = 0b00000000_00010000;
    /// Comparator polarity (ADS1114 and ADS1115 only)
    const COMP_POL  = 0b00000000_00001000;
    /// Latching comparator (ADS1114 and ADS1115 only)
    const COMP_LAT  = 0b00000000_00000100;
    /// Comparator queue and disable bit 1 (ADS1114 and ADS1115 only)
    const COMP_QUE1 = 0b00000000_00000010;
    /// Comparator queue and disable bit 0 (ADS1114 and ADS1115 only)
    const COMP_QUE0 = 0b00000000_00000001;

    /// Input multiplexer configuration (ADS1115 only)
    const MUX = Self::MUX2.bits() | Self::MUX1.bits() | Self::MUX0.bits();
    /// Programmable gain amplifier configuration
    const PGA = Self::PGA2.bits() | Self::PGA1.bits() | Self::PGA0.bits();
    /// Data rate
    const DR = Self::DR2.bits() | Self::DR1.bits() | Self::DR0.bits();
    /// Comparator queue and disable (ADS1114 and ADS1115 only)
    const COMP_QUE = Self::COMP_QUE1.bits() | Self::COMP_QUE0.bits();
  }
}

impl Default for Config {
    fn default() -> Self {
        Self::OS
            .union(Self::PGA1)
            .union(Self::MODE)
            .union(Self::DR2)
            .union(Self::COMP_QUE1)
            .union(Self::COMP_QUE0)
    }
}

register! {
  /// Lo_thresh
  pub struct LoThresh(u16): 0x02;
}

register! {
  /// Hi_thresh
  pub struct HiThresh(u16): 0x03;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_measurement_12bit() {
        assert_eq!(0, Conversion12::convert_measurement(0));
        assert_eq!(2047, Conversion12::convert_measurement(0x7FFF));
        assert_eq!(-2048, Conversion12::convert_measurement(0x8000));
        assert_eq!(-1, Conversion12::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_measurement_16bit() {
        assert_eq!(0, Conversion16::convert_measurement(0));
        assert_eq!(32767, Conversion16::convert_measurement(0x7FFF));
        assert_eq!(-32768, Conversion16::convert_measurement(0x8000));
        assert_eq!(-1, Conversion16::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_threshold_12bit() {
        assert_eq!(0, Conversion12::convert_threshold(0));
        assert_eq!(0x7FF0, Conversion12::convert_threshold(2047));
        assert_eq!(0x8000, Conversion12::convert_threshold(-2048));
        assert_eq!(0xFFF0, Conversion12::convert_threshold(-1));
    }

    #[test]
    #[should_panic]
    fn convert_threshold_12bit_invalid_low() {
        Conversion12::convert_threshold(-2049);
    }

    #[test]
    #[should_panic]
    fn convert_threshold_12bit_invalid_high() {
        Conversion12::convert_threshold(2048);
    }

    #[test]
    fn convert_threshold_16bit() {
        assert_eq!(0x7FFF, Conversion16::convert_threshold(32767));
        assert_eq!(0x8000, Conversion16::convert_threshold(-32768));
    }
}

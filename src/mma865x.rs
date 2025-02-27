//! MLX90614-specific functions

use embedded_hal::i2c::I2c;

use crate::{ic, mode, register_access::BitFlags, Config, Mma8x5x};
use core::marker::PhantomData;

impl<I2C: I2c> Mma8x5x<I2C, ic::Mma8652, mode::Standby> {
    /// Create new instance of the MMA8652 device.
    pub fn new_mma8652(i2c: I2C) -> Self {
        Mma8x5x {
            i2c,
            address: 0x1D,
            xyz_data_cfg: Config::default(),
            ctrl_reg1: Config::default(),
            ctrl_reg2: Config::default(),
            ctrl_reg3: Config::default(),
            pl_cfg: Config {
                bits: BitFlags::DBCNTM,
            },
            _ic: PhantomData,
            _mode: PhantomData,
        }
    }
}

impl<I2C: I2c> Mma8x5x<I2C, ic::Mma8653, mode::Standby> {
    /// Create new instance of the MMA8653 device.
    pub fn new_mma8653(i2c: I2C) -> Self {
        Mma8x5x {
            i2c,
            address: 0x1D,
            xyz_data_cfg: Config::default(),
            ctrl_reg1: Config::default(),
            ctrl_reg2: Config::default(),
            ctrl_reg3: Config::default(),
            pl_cfg: Config {
                bits: BitFlags::DBCNTM,
            },
            _ic: PhantomData,
            _mode: PhantomData,
        }
    }
}

//! Driver for IMX477 camera.
//! 
//! Adapted from https://github.com/raspberrypi/linux/blob/19a1b03529363945fbbb4b9160fe8645809a9dce/drivers/media/i2c/imx477.c

use crate::i2c::{I2cDev, I2cAddr};
use crate::gpio::{Pin, Output};
use crate::clock::Clocks;
use crate::time::wait;

/// I²C address of the sensor.
const I2C_ADDR: I2cAddr = I2cAddr::new(0x1A);

const REG_CHIP_ID: u16      = 0x0016;

const REG_FRAME_LENGTH: u16 = 0x0340;
const MAX_FRAME_LENGTH: u16 = 0xFFDC;

const REG_LINE_LENGTH: u16  = 0x0342;
const MAX_LINE_LENGTH: u16  = 0xFFF0;

const REG_EXPOSURE: u16     = 0x0202;
const EXPOSURE_OFFSET: u16  = 22;
const DEFAULT_EXPOSURE: u16 = 0x640;
const MIN_EXPOSURE: u16     = 4;
const MAX_EXPOSURE: u16     = MAX_LINE_LENGTH - EXPOSURE_OFFSET;

const REG_ANALOG_GAIN: u16  = 0x0204;
const MIN_ANALOG_GAIN: u16  = 0;
const MAX_ANALOG_GAIN: u16  = 978;

/// Required clock frequency.
const XCLK_FREQ: u32 = 24_000_000;
const DEFAULT_LINK_FREQ: u32 = 450_000_000;

const PIXEL_RATE: u32 = 840_000_000;

/// Minimum delay before power up is considered ready (8 ms).
const POWER_ON_DELAY: u64 = 8_000_000;


/// Driver structure for Sony IMX477 sensor.
pub struct Imx477<const NUM: u8, D: I2cDev> {
    /// The I²C bus for communicating with the sensor.
    i2c: D,
    /// The reset pin, hardware connected to the reset pin of the sensor.
    reset: Pin<NUM, Output>,
    /// Sensor mode.
    mode: &'static Mode,

    long_exp_shift: u8,
}

impl<const NUM: u8, D: I2cDev> Imx477<NUM, D> {

    pub fn probe(
        mut i2c: D, 
        mut reset: Pin<NUM, Output>, 
        clocks: &Clocks
    ) -> Self {

        // Power on...
        reset.set_high();
        wait(POWER_ON_DELAY);

        // Identify module...
        let value = read_reg(&mut i2c, REG_CHIP_ID, RegLen::Size2);
        assert_eq!(value, 0x0477, "incoherent chip id for IMX477");

        Self {
            i2c,
            reset,
            mode: &MODE_4056_3040,
            long_exp_shift: 0,
        }

    }

    fn read_reg(&mut self, reg: u16, len: RegLen) -> u32 {
        read_reg(&mut self.i2c, reg, len)
    }

    fn write_reg(&mut self, reg: u16, len: RegLen, val: u32) {
        write_reg(&mut self.i2c, reg, len, val)
    }

    /// Set analog gain (0..978).
    pub fn set_analog_gain(&mut self, gain: u16) {
        debug_assert!(gain <= 978);
        self.write_reg(REG_ANALOG_GAIN, RegLen::Size2, gain as _)
    }

    pub fn set_exposure(&mut self, exposure: u16) {

    }

}


#[derive(Debug, Clone, Copy)]
enum RegLen {
    Size1 = 1,
    Size2 = 2,
    Size3 = 3,
    Size4 = 4,
}

fn read_reg(i2c: &mut impl I2cDev, reg: u16, len: RegLen) -> u32 {
    let mut data = [0; 4];
    i2c.write(I2C_ADDR, None, &[(reg >> 8) as u8, reg as u8]);
    i2c.read(I2C_ADDR, None, &mut data[(4 - len as usize)..]);
    u32::from_be_bytes(data)
}

fn write_reg(i2c: &mut impl I2cDev, reg: u16, len: RegLen, val: u32) {
    let mut data = [0; 6];
    data[0..2].copy_from_slice(&reg.to_be_bytes());
    data[2..6].copy_from_slice(&(val << 8 * (4 - len as u32)).to_be_bytes());
    i2c.write(I2C_ADDR, None, &data[..(2 + len as usize)]);
}


/// Definition of an I²C register address and value on IMX477.
struct RegVal(u16, u8);

/// Describe IMX477 sensor mode.
struct Mode {
    /// Frame width.
    width: u16,
    /// Frame height.
    height: u16,
    /// H-timing in pixels.
    line_length_pix: u16,
    /// Framerate ofc... 
    framerate: u16,
    /// Default registers values.
    regs: &'static [RegVal],
}

/// Definition of all registers common to all display modes.
const MODE_COMMON_REGS: [RegVal; 309] = [
    RegVal(0x0136, 0x18),
    RegVal(0x0137, 0x00),
    RegVal(0x0138, 0x01),
    RegVal(0xe000, 0x00),
    RegVal(0xe07a, 0x01),
    RegVal(0x0808, 0x02),
    RegVal(0x4ae9, 0x18),
    RegVal(0x4aea, 0x08),
    RegVal(0xf61c, 0x04),
    RegVal(0xf61e, 0x04),
    RegVal(0x4ae9, 0x21),
    RegVal(0x4aea, 0x80),
    RegVal(0x38a8, 0x1f),
    RegVal(0x38a9, 0xff),
    RegVal(0x38aa, 0x1f),
    RegVal(0x38ab, 0xff),
    RegVal(0x55d4, 0x00),
    RegVal(0x55d5, 0x00),
    RegVal(0x55d6, 0x07),
    RegVal(0x55d7, 0xff),
    RegVal(0x55e8, 0x07),
    RegVal(0x55e9, 0xff),
    RegVal(0x55ea, 0x00),
    RegVal(0x55eb, 0x00),
    RegVal(0x574c, 0x07),
    RegVal(0x574d, 0xff),
    RegVal(0x574e, 0x00),
    RegVal(0x574f, 0x00),
    RegVal(0x5754, 0x00),
    RegVal(0x5755, 0x00),
    RegVal(0x5756, 0x07),
    RegVal(0x5757, 0xff),
    RegVal(0x5973, 0x04),
    RegVal(0x5974, 0x01),
    RegVal(0x5d13, 0xc3),
    RegVal(0x5d14, 0x58),
    RegVal(0x5d15, 0xa3),
    RegVal(0x5d16, 0x1d),
    RegVal(0x5d17, 0x65),
    RegVal(0x5d18, 0x8c),
    RegVal(0x5d1a, 0x06),
    RegVal(0x5d1b, 0xa9),
    RegVal(0x5d1c, 0x45),
    RegVal(0x5d1d, 0x3a),
    RegVal(0x5d1e, 0xab),
    RegVal(0x5d1f, 0x15),
    RegVal(0x5d21, 0x0e),
    RegVal(0x5d22, 0x52),
    RegVal(0x5d23, 0xaa),
    RegVal(0x5d24, 0x7d),
    RegVal(0x5d25, 0x57),
    RegVal(0x5d26, 0xa8),
    RegVal(0x5d37, 0x5a),
    RegVal(0x5d38, 0x5a),
    RegVal(0x5d77, 0x7f),
    RegVal(0x7b75, 0x0e),
    RegVal(0x7b76, 0x0b),
    RegVal(0x7b77, 0x08),
    RegVal(0x7b78, 0x0a),
    RegVal(0x7b79, 0x47),
    RegVal(0x7b7c, 0x00),
    RegVal(0x7b7d, 0x00),
    RegVal(0x8d1f, 0x00),
    RegVal(0x8d27, 0x00),
    RegVal(0x9004, 0x03),
    RegVal(0x9200, 0x50),
    RegVal(0x9201, 0x6c),
    RegVal(0x9202, 0x71),
    RegVal(0x9203, 0x00),
    RegVal(0x9204, 0x71),
    RegVal(0x9205, 0x01),
    RegVal(0x9371, 0x6a),
    RegVal(0x9373, 0x6a),
    RegVal(0x9375, 0x64),
    RegVal(0x991a, 0x00),
    RegVal(0x996b, 0x8c),
    RegVal(0x996c, 0x64),
    RegVal(0x996d, 0x50),
    RegVal(0x9a4c, 0x0d),
    RegVal(0x9a4d, 0x0d),
    RegVal(0xa001, 0x0a),
    RegVal(0xa003, 0x0a),
    RegVal(0xa005, 0x0a),
    RegVal(0xa006, 0x01),
    RegVal(0xa007, 0xc0),
    RegVal(0xa009, 0xc0),
    RegVal(0x3d8a, 0x01),
    RegVal(0x4421, 0x04),
    RegVal(0x7b3b, 0x01),
    RegVal(0x7b4c, 0x00),
    RegVal(0x9905, 0x00),
    RegVal(0x9907, 0x00),
    RegVal(0x9909, 0x00),
    RegVal(0x990b, 0x00),
    RegVal(0x9944, 0x3c),
    RegVal(0x9947, 0x3c),
    RegVal(0x994a, 0x8c),
    RegVal(0x994b, 0x50),
    RegVal(0x994c, 0x1b),
    RegVal(0x994d, 0x8c),
    RegVal(0x994e, 0x50),
    RegVal(0x994f, 0x1b),
    RegVal(0x9950, 0x8c),
    RegVal(0x9951, 0x1b),
    RegVal(0x9952, 0x0a),
    RegVal(0x9953, 0x8c),
    RegVal(0x9954, 0x1b),
    RegVal(0x9955, 0x0a),
    RegVal(0x9a13, 0x04),
    RegVal(0x9a14, 0x04),
    RegVal(0x9a19, 0x00),
    RegVal(0x9a1c, 0x04),
    RegVal(0x9a1d, 0x04),
    RegVal(0x9a26, 0x05),
    RegVal(0x9a27, 0x05),
    RegVal(0x9a2c, 0x01),
    RegVal(0x9a2d, 0x03),
    RegVal(0x9a2f, 0x05),
    RegVal(0x9a30, 0x05),
    RegVal(0x9a41, 0x00),
    RegVal(0x9a46, 0x00),
    RegVal(0x9a47, 0x00),
    RegVal(0x9c17, 0x35),
    RegVal(0x9c1d, 0x31),
    RegVal(0x9c29, 0x50),
    RegVal(0x9c3b, 0x2f),
    RegVal(0x9c41, 0x6b),
    RegVal(0x9c47, 0x2d),
    RegVal(0x9c4d, 0x40),
    RegVal(0x9c6b, 0x00),
    RegVal(0x9c71, 0xc8),
    RegVal(0x9c73, 0x32),
    RegVal(0x9c75, 0x04),
    RegVal(0x9c7d, 0x2d),
    RegVal(0x9c83, 0x40),
    RegVal(0x9c94, 0x3f),
    RegVal(0x9c95, 0x3f),
    RegVal(0x9c96, 0x3f),
    RegVal(0x9c97, 0x00),
    RegVal(0x9c98, 0x00),
    RegVal(0x9c99, 0x00),
    RegVal(0x9c9a, 0x3f),
    RegVal(0x9c9b, 0x3f),
    RegVal(0x9c9c, 0x3f),
    RegVal(0x9ca0, 0x0f),
    RegVal(0x9ca1, 0x0f),
    RegVal(0x9ca2, 0x0f),
    RegVal(0x9ca3, 0x00),
    RegVal(0x9ca4, 0x00),
    RegVal(0x9ca5, 0x00),
    RegVal(0x9ca6, 0x1e),
    RegVal(0x9ca7, 0x1e),
    RegVal(0x9ca8, 0x1e),
    RegVal(0x9ca9, 0x00),
    RegVal(0x9caa, 0x00),
    RegVal(0x9cab, 0x00),
    RegVal(0x9cac, 0x09),
    RegVal(0x9cad, 0x09),
    RegVal(0x9cae, 0x09),
    RegVal(0x9cbd, 0x50),
    RegVal(0x9cbf, 0x50),
    RegVal(0x9cc1, 0x50),
    RegVal(0x9cc3, 0x40),
    RegVal(0x9cc5, 0x40),
    RegVal(0x9cc7, 0x40),
    RegVal(0x9cc9, 0x0a),
    RegVal(0x9ccb, 0x0a),
    RegVal(0x9ccd, 0x0a),
    RegVal(0x9d17, 0x35),
    RegVal(0x9d1d, 0x31),
    RegVal(0x9d29, 0x50),
    RegVal(0x9d3b, 0x2f),
    RegVal(0x9d41, 0x6b),
    RegVal(0x9d47, 0x42),
    RegVal(0x9d4d, 0x5a),
    RegVal(0x9d6b, 0x00),
    RegVal(0x9d71, 0xc8),
    RegVal(0x9d73, 0x32),
    RegVal(0x9d75, 0x04),
    RegVal(0x9d7d, 0x42),
    RegVal(0x9d83, 0x5a),
    RegVal(0x9d94, 0x3f),
    RegVal(0x9d95, 0x3f),
    RegVal(0x9d96, 0x3f),
    RegVal(0x9d97, 0x00),
    RegVal(0x9d98, 0x00),
    RegVal(0x9d99, 0x00),
    RegVal(0x9d9a, 0x3f),
    RegVal(0x9d9b, 0x3f),
    RegVal(0x9d9c, 0x3f),
    RegVal(0x9d9d, 0x1f),
    RegVal(0x9d9e, 0x1f),
    RegVal(0x9d9f, 0x1f),
    RegVal(0x9da0, 0x0f),
    RegVal(0x9da1, 0x0f),
    RegVal(0x9da2, 0x0f),
    RegVal(0x9da3, 0x00),
    RegVal(0x9da4, 0x00),
    RegVal(0x9da5, 0x00),
    RegVal(0x9da6, 0x1e),
    RegVal(0x9da7, 0x1e),
    RegVal(0x9da8, 0x1e),
    RegVal(0x9da9, 0x00),
    RegVal(0x9daa, 0x00),
    RegVal(0x9dab, 0x00),
    RegVal(0x9dac, 0x09),
    RegVal(0x9dad, 0x09),
    RegVal(0x9dae, 0x09),
    RegVal(0x9dc9, 0x0a),
    RegVal(0x9dcb, 0x0a),
    RegVal(0x9dcd, 0x0a),
    RegVal(0x9e17, 0x35),
    RegVal(0x9e1d, 0x31),
    RegVal(0x9e29, 0x50),
    RegVal(0x9e3b, 0x2f),
    RegVal(0x9e41, 0x6b),
    RegVal(0x9e47, 0x2d),
    RegVal(0x9e4d, 0x40),
    RegVal(0x9e6b, 0x00),
    RegVal(0x9e71, 0xc8),
    RegVal(0x9e73, 0x32),
    RegVal(0x9e75, 0x04),
    RegVal(0x9e94, 0x0f),
    RegVal(0x9e95, 0x0f),
    RegVal(0x9e96, 0x0f),
    RegVal(0x9e97, 0x00),
    RegVal(0x9e98, 0x00),
    RegVal(0x9e99, 0x00),
    RegVal(0x9ea0, 0x0f),
    RegVal(0x9ea1, 0x0f),
    RegVal(0x9ea2, 0x0f),
    RegVal(0x9ea3, 0x00),
    RegVal(0x9ea4, 0x00),
    RegVal(0x9ea5, 0x00),
    RegVal(0x9ea6, 0x3f),
    RegVal(0x9ea7, 0x3f),
    RegVal(0x9ea8, 0x3f),
    RegVal(0x9ea9, 0x00),
    RegVal(0x9eaa, 0x00),
    RegVal(0x9eab, 0x00),
    RegVal(0x9eac, 0x09),
    RegVal(0x9ead, 0x09),
    RegVal(0x9eae, 0x09),
    RegVal(0x9ec9, 0x0a),
    RegVal(0x9ecb, 0x0a),
    RegVal(0x9ecd, 0x0a),
    RegVal(0x9f17, 0x35),
    RegVal(0x9f1d, 0x31),
    RegVal(0x9f29, 0x50),
    RegVal(0x9f3b, 0x2f),
    RegVal(0x9f41, 0x6b),
    RegVal(0x9f47, 0x42),
    RegVal(0x9f4d, 0x5a),
    RegVal(0x9f6b, 0x00),
    RegVal(0x9f71, 0xc8),
    RegVal(0x9f73, 0x32),
    RegVal(0x9f75, 0x04),
    RegVal(0x9f94, 0x0f),
    RegVal(0x9f95, 0x0f),
    RegVal(0x9f96, 0x0f),
    RegVal(0x9f97, 0x00),
    RegVal(0x9f98, 0x00),
    RegVal(0x9f99, 0x00),
    RegVal(0x9f9a, 0x2f),
    RegVal(0x9f9b, 0x2f),
    RegVal(0x9f9c, 0x2f),
    RegVal(0x9f9d, 0x00),
    RegVal(0x9f9e, 0x00),
    RegVal(0x9f9f, 0x00),
    RegVal(0x9fa0, 0x0f),
    RegVal(0x9fa1, 0x0f),
    RegVal(0x9fa2, 0x0f),
    RegVal(0x9fa3, 0x00),
    RegVal(0x9fa4, 0x00),
    RegVal(0x9fa5, 0x00),
    RegVal(0x9fa6, 0x1e),
    RegVal(0x9fa7, 0x1e),
    RegVal(0x9fa8, 0x1e),
    RegVal(0x9fa9, 0x00),
    RegVal(0x9faa, 0x00),
    RegVal(0x9fab, 0x00),
    RegVal(0x9fac, 0x09),
    RegVal(0x9fad, 0x09),
    RegVal(0x9fae, 0x09),
    RegVal(0x9fc9, 0x0a),
    RegVal(0x9fcb, 0x0a),
    RegVal(0x9fcd, 0x0a),
    RegVal(0xa14b, 0xff),
    RegVal(0xa151, 0x0c),
    RegVal(0xa153, 0x50),
    RegVal(0xa155, 0x02),
    RegVal(0xa157, 0x00),
    RegVal(0xa1ad, 0xff),
    RegVal(0xa1b3, 0x0c),
    RegVal(0xa1b5, 0x50),
    RegVal(0xa1b9, 0x00),
    RegVal(0xa24b, 0xff),
    RegVal(0xa257, 0x00),
    RegVal(0xa2ad, 0xff),
    RegVal(0xa2b9, 0x00),
    RegVal(0xb21f, 0x04),
    RegVal(0xb35c, 0x00),
    RegVal(0xb35e, 0x08),
    RegVal(0x0112, 0x0c),
    RegVal(0x0113, 0x0c),
    RegVal(0x0114, 0x01),
    RegVal(0x0350, 0x00),
    RegVal(0xbcf1, 0x02),
    RegVal(0x3ff9, 0x01),
];

/// Mode: 12 Mpix 10 fps.
const MODE_4056_3040: Mode = Mode {
    width: 4056,
    height: 3040,
    line_length_pix: 0x5DC0,
    framerate: 10,
    regs: &[
        RegVal(0x0342, 0x5d),
        RegVal(0x0343, 0xc0),
        RegVal(0x0344, 0x00),
        RegVal(0x0345, 0x00),
        RegVal(0x0346, 0x00),
        RegVal(0x0347, 0x00),
        RegVal(0x0348, 0x0f),
        RegVal(0x0349, 0xd7),
        RegVal(0x034a, 0x0b),
        RegVal(0x034b, 0xdf),
        RegVal(0x00e3, 0x00),
        RegVal(0x00e4, 0x00),
        RegVal(0x00fc, 0x0a),
        RegVal(0x00fd, 0x0a),
        RegVal(0x00fe, 0x0a),
        RegVal(0x00ff, 0x0a),
        RegVal(0x0220, 0x00),
        RegVal(0x0221, 0x11),
        RegVal(0x0381, 0x01),
        RegVal(0x0383, 0x01),
        RegVal(0x0385, 0x01),
        RegVal(0x0387, 0x01),
        RegVal(0x0900, 0x00),
        RegVal(0x0901, 0x11),
        RegVal(0x0902, 0x02),
        RegVal(0x3140, 0x02),
        RegVal(0x3c00, 0x00),
        RegVal(0x3c01, 0x03),
        RegVal(0x3c02, 0xa2),
        RegVal(0x3f0d, 0x01),
        RegVal(0x5748, 0x07),
        RegVal(0x5749, 0xff),
        RegVal(0x574a, 0x00),
        RegVal(0x574b, 0x00),
        RegVal(0x7b75, 0x0a),
        RegVal(0x7b76, 0x0c),
        RegVal(0x7b77, 0x07),
        RegVal(0x7b78, 0x06),
        RegVal(0x7b79, 0x3c),
        RegVal(0x7b53, 0x01),
        RegVal(0x9369, 0x5a),
        RegVal(0x936b, 0x55),
        RegVal(0x936d, 0x28),
        RegVal(0x9304, 0x00),
        RegVal(0x9305, 0x00),
        RegVal(0x9e9a, 0x2f),
        RegVal(0x9e9b, 0x2f),
        RegVal(0x9e9c, 0x2f),
        RegVal(0x9e9d, 0x00),
        RegVal(0x9e9e, 0x00),
        RegVal(0x9e9f, 0x00),
        RegVal(0xa2a9, 0x60),
        RegVal(0xa2b7, 0x00),
        RegVal(0x0401, 0x00),
        RegVal(0x0404, 0x00),
        RegVal(0x0405, 0x10),
        RegVal(0x0408, 0x00),
        RegVal(0x0409, 0x00),
        RegVal(0x040a, 0x00),
        RegVal(0x040b, 0x00),
        RegVal(0x040c, 0x0f),
        RegVal(0x040d, 0xd8),
        RegVal(0x040e, 0x0b),
        RegVal(0x040f, 0xe0),
        RegVal(0x034c, 0x0f),
        RegVal(0x034d, 0xd8),
        RegVal(0x034e, 0x0b),
        RegVal(0x034f, 0xe0),
        RegVal(0x0301, 0x05),
        RegVal(0x0303, 0x02),
        RegVal(0x0305, 0x04),
        RegVal(0x0306, 0x01),
        RegVal(0x0307, 0x5e),
        RegVal(0x0309, 0x0c),
        RegVal(0x030b, 0x02),
        RegVal(0x030d, 0x02),
        RegVal(0x030e, 0x00),
        RegVal(0x030f, 0x96),
        RegVal(0x0310, 0x01),
        RegVal(0x0820, 0x07),
        RegVal(0x0821, 0x08),
        RegVal(0x0822, 0x00),
        RegVal(0x0823, 0x00),
        RegVal(0x080a, 0x00),
        RegVal(0x080b, 0x7f),
        RegVal(0x080c, 0x00),
        RegVal(0x080d, 0x4f),
        RegVal(0x080e, 0x00),
        RegVal(0x080f, 0x77),
        RegVal(0x0810, 0x00),
        RegVal(0x0811, 0x5f),
        RegVal(0x0812, 0x00),
        RegVal(0x0813, 0x57),
        RegVal(0x0814, 0x00),
        RegVal(0x0815, 0x4f),
        RegVal(0x0816, 0x01),
        RegVal(0x0817, 0x27),
        RegVal(0x0818, 0x00),
        RegVal(0x0819, 0x3f),
        RegVal(0xe04c, 0x00),
        RegVal(0xe04d, 0x7f),
        RegVal(0xe04e, 0x00),
        RegVal(0xe04f, 0x1f),
        RegVal(0x3e20, 0x01),
        RegVal(0x3e37, 0x00),
        RegVal(0x3f50, 0x00),
        RegVal(0x3f56, 0x02),
        RegVal(0x3f57, 0xae),
    ],
};

/// Mode: 2x2 binned 40 fps.
const MODE_2028_1520: Mode = Mode {
    width: 2028,
    height: 1520,
    line_length_pix: 0x31C4,
    framerate: 40,
    regs: &[
        RegVal(0x0342, 0x31),
        RegVal(0x0343, 0xc4),
        RegVal(0x0344, 0x00),
        RegVal(0x0345, 0x00),
        RegVal(0x0346, 0x00),
        RegVal(0x0347, 0x00),
        RegVal(0x0348, 0x0f),
        RegVal(0x0349, 0xd7),
        RegVal(0x034a, 0x0b),
        RegVal(0x034b, 0xdf),
        RegVal(0x0220, 0x00),
        RegVal(0x0221, 0x11),
        RegVal(0x0381, 0x01),
        RegVal(0x0383, 0x01),
        RegVal(0x0385, 0x01),
        RegVal(0x0387, 0x01),
        RegVal(0x0900, 0x01),
        RegVal(0x0901, 0x12),
        RegVal(0x0902, 0x02),
        RegVal(0x3140, 0x02),
        RegVal(0x3c00, 0x00),
        RegVal(0x3c01, 0x03),
        RegVal(0x3c02, 0xa2),
        RegVal(0x3f0d, 0x01),
        RegVal(0x5748, 0x07),
        RegVal(0x5749, 0xff),
        RegVal(0x574a, 0x00),
        RegVal(0x574b, 0x00),
        RegVal(0x7b53, 0x01),
        RegVal(0x9369, 0x73),
        RegVal(0x936b, 0x64),
        RegVal(0x936d, 0x5f),
        RegVal(0x9304, 0x00),
        RegVal(0x9305, 0x00),
        RegVal(0x9e9a, 0x2f),
        RegVal(0x9e9b, 0x2f),
        RegVal(0x9e9c, 0x2f),
        RegVal(0x9e9d, 0x00),
        RegVal(0x9e9e, 0x00),
        RegVal(0x9e9f, 0x00),
        RegVal(0xa2a9, 0x60),
        RegVal(0xa2b7, 0x00),
        RegVal(0x0401, 0x01),
        RegVal(0x0404, 0x00),
        RegVal(0x0405, 0x20),
        RegVal(0x0408, 0x00),
        RegVal(0x0409, 0x00),
        RegVal(0x040a, 0x00),
        RegVal(0x040b, 0x00),
        RegVal(0x040c, 0x0f),
        RegVal(0x040d, 0xd8),
        RegVal(0x040e, 0x0b),
        RegVal(0x040f, 0xe0),
        RegVal(0x034c, 0x07),
        RegVal(0x034d, 0xec),
        RegVal(0x034e, 0x05),
        RegVal(0x034f, 0xf0),
        RegVal(0x0301, 0x05),
        RegVal(0x0303, 0x02),
        RegVal(0x0305, 0x04),
        RegVal(0x0306, 0x01),
        RegVal(0x0307, 0x5e),
        RegVal(0x0309, 0x0c),
        RegVal(0x030b, 0x02),
        RegVal(0x030d, 0x02),
        RegVal(0x030e, 0x00),
        RegVal(0x030f, 0x96),
        RegVal(0x0310, 0x01),
        RegVal(0x0820, 0x07),
        RegVal(0x0821, 0x08),
        RegVal(0x0822, 0x00),
        RegVal(0x0823, 0x00),
        RegVal(0x080a, 0x00),
        RegVal(0x080b, 0x7f),
        RegVal(0x080c, 0x00),
        RegVal(0x080d, 0x4f),
        RegVal(0x080e, 0x00),
        RegVal(0x080f, 0x77),
        RegVal(0x0810, 0x00),
        RegVal(0x0811, 0x5f),
        RegVal(0x0812, 0x00),
        RegVal(0x0813, 0x57),
        RegVal(0x0814, 0x00),
        RegVal(0x0815, 0x4f),
        RegVal(0x0816, 0x01),
        RegVal(0x0817, 0x27),
        RegVal(0x0818, 0x00),
        RegVal(0x0819, 0x3f),
        RegVal(0xe04c, 0x00),
        RegVal(0xe04d, 0x7f),
        RegVal(0xe04e, 0x00),
        RegVal(0xe04f, 0x1f),
        RegVal(0x3e20, 0x01),
        RegVal(0x3e37, 0x00),
        RegVal(0x3f50, 0x00),
        RegVal(0x3f56, 0x01),
        RegVal(0x3f57, 0x6c),
    ],
};

/// Mode: 1080p copped 50fps.
const MODE_2028_1080: Mode = Mode {
    width: 2028,
    height: 1080,
    line_length_pix: 0x31C4,
    framerate: 50,
    regs: &[
        RegVal(0x0342, 0x31),
        RegVal(0x0343, 0xc4),
        RegVal(0x0344, 0x00),
        RegVal(0x0345, 0x00),
        RegVal(0x0346, 0x01),
        RegVal(0x0347, 0xb8),
        RegVal(0x0348, 0x0f),
        RegVal(0x0349, 0xd7),
        RegVal(0x034a, 0x0a),
        RegVal(0x034b, 0x27),
        RegVal(0x0220, 0x00),
        RegVal(0x0221, 0x11),
        RegVal(0x0381, 0x01),
        RegVal(0x0383, 0x01),
        RegVal(0x0385, 0x01),
        RegVal(0x0387, 0x01),
        RegVal(0x0900, 0x01),
        RegVal(0x0901, 0x12),
        RegVal(0x0902, 0x02),
        RegVal(0x3140, 0x02),
        RegVal(0x3c00, 0x00),
        RegVal(0x3c01, 0x03),
        RegVal(0x3c02, 0xa2),
        RegVal(0x3f0d, 0x01),
        RegVal(0x5748, 0x07),
        RegVal(0x5749, 0xff),
        RegVal(0x574a, 0x00),
        RegVal(0x574b, 0x00),
        RegVal(0x7b53, 0x01),
        RegVal(0x9369, 0x73),
        RegVal(0x936b, 0x64),
        RegVal(0x936d, 0x5f),
        RegVal(0x9304, 0x00),
        RegVal(0x9305, 0x00),
        RegVal(0x9e9a, 0x2f),
        RegVal(0x9e9b, 0x2f),
        RegVal(0x9e9c, 0x2f),
        RegVal(0x9e9d, 0x00),
        RegVal(0x9e9e, 0x00),
        RegVal(0x9e9f, 0x00),
        RegVal(0xa2a9, 0x60),
        RegVal(0xa2b7, 0x00),
        RegVal(0x0401, 0x01),
        RegVal(0x0404, 0x00),
        RegVal(0x0405, 0x20),
        RegVal(0x0408, 0x00),
        RegVal(0x0409, 0x00),
        RegVal(0x040a, 0x00),
        RegVal(0x040b, 0x00),
        RegVal(0x040c, 0x0f),
        RegVal(0x040d, 0xd8),
        RegVal(0x040e, 0x04),
        RegVal(0x040f, 0x38),
        RegVal(0x034c, 0x07),
        RegVal(0x034d, 0xec),
        RegVal(0x034e, 0x04),
        RegVal(0x034f, 0x38),
        RegVal(0x0301, 0x05),
        RegVal(0x0303, 0x02),
        RegVal(0x0305, 0x04),
        RegVal(0x0306, 0x01),
        RegVal(0x0307, 0x5e),
        RegVal(0x0309, 0x0c),
        RegVal(0x030b, 0x02),
        RegVal(0x030d, 0x02),
        RegVal(0x030e, 0x00),
        RegVal(0x030f, 0x96),
        RegVal(0x0310, 0x01),
        RegVal(0x0820, 0x07),
        RegVal(0x0821, 0x08),
        RegVal(0x0822, 0x00),
        RegVal(0x0823, 0x00),
        RegVal(0x080a, 0x00),
        RegVal(0x080b, 0x7f),
        RegVal(0x080c, 0x00),
        RegVal(0x080d, 0x4f),
        RegVal(0x080e, 0x00),
        RegVal(0x080f, 0x77),
        RegVal(0x0810, 0x00),
        RegVal(0x0811, 0x5f),
        RegVal(0x0812, 0x00),
        RegVal(0x0813, 0x57),
        RegVal(0x0814, 0x00),
        RegVal(0x0815, 0x4f),
        RegVal(0x0816, 0x01),
        RegVal(0x0817, 0x27),
        RegVal(0x0818, 0x00),
        RegVal(0x0819, 0x3f),
        RegVal(0xe04c, 0x00),
        RegVal(0xe04d, 0x7f),
        RegVal(0xe04e, 0x00),
        RegVal(0xe04f, 0x1f),
        RegVal(0x3e20, 0x01),
        RegVal(0x3e37, 0x00),
        RegVal(0x3f50, 0x00),
        RegVal(0x3f56, 0x01),
        RegVal(0x3f57, 0x6c),
    ],
};

/// Mode: 2x2 binned and cropped 120fps.
const MODE_1332_990: Mode = Mode {
    width: 1332,
    height: 990,
    line_length_pix: 6664,
    framerate: 120,
    regs: &[
        RegVal(0x420b, 0x01),
        RegVal(0x990c, 0x00),
        RegVal(0x990d, 0x08),
        RegVal(0x9956, 0x8c),
        RegVal(0x9957, 0x64),
        RegVal(0x9958, 0x50),
        RegVal(0x9a48, 0x06),
        RegVal(0x9a49, 0x06),
        RegVal(0x9a4a, 0x06),
        RegVal(0x9a4b, 0x06),
        RegVal(0x9a4c, 0x06),
        RegVal(0x9a4d, 0x06),
        RegVal(0x0112, 0x0a),
        RegVal(0x0113, 0x0a),
        RegVal(0x0114, 0x01),
        RegVal(0x0342, 0x1a),
        RegVal(0x0343, 0x08),
        RegVal(0x0340, 0x04),
        RegVal(0x0341, 0x1a),
        RegVal(0x0344, 0x00),
        RegVal(0x0345, 0x00),
        RegVal(0x0346, 0x02),
        RegVal(0x0347, 0x10),
        RegVal(0x0348, 0x0f),
        RegVal(0x0349, 0xd7),
        RegVal(0x034a, 0x09),
        RegVal(0x034b, 0xcf),
        RegVal(0x00e3, 0x00),
        RegVal(0x00e4, 0x00),
        RegVal(0x00fc, 0x0a),
        RegVal(0x00fd, 0x0a),
        RegVal(0x00fe, 0x0a),
        RegVal(0x00ff, 0x0a),
        RegVal(0xe013, 0x00),
        RegVal(0x0220, 0x00),
        RegVal(0x0221, 0x11),
        RegVal(0x0381, 0x01),
        RegVal(0x0383, 0x01),
        RegVal(0x0385, 0x01),
        RegVal(0x0387, 0x01),
        RegVal(0x0900, 0x01),
        RegVal(0x0901, 0x22),
        RegVal(0x0902, 0x02),
        RegVal(0x3140, 0x02),
        RegVal(0x3c00, 0x00),
        RegVal(0x3c01, 0x01),
        RegVal(0x3c02, 0x9c),
        RegVal(0x3f0d, 0x00),
        RegVal(0x5748, 0x00),
        RegVal(0x5749, 0x00),
        RegVal(0x574a, 0x00),
        RegVal(0x574b, 0xa4),
        RegVal(0x7b75, 0x0e),
        RegVal(0x7b76, 0x09),
        RegVal(0x7b77, 0x08),
        RegVal(0x7b78, 0x06),
        RegVal(0x7b79, 0x34),
        RegVal(0x7b53, 0x00),
        RegVal(0x9369, 0x73),
        RegVal(0x936b, 0x64),
        RegVal(0x936d, 0x5f),
        RegVal(0x9304, 0x03),
        RegVal(0x9305, 0x80),
        RegVal(0x9e9a, 0x2f),
        RegVal(0x9e9b, 0x2f),
        RegVal(0x9e9c, 0x2f),
        RegVal(0x9e9d, 0x00),
        RegVal(0x9e9e, 0x00),
        RegVal(0x9e9f, 0x00),
        RegVal(0xa2a9, 0x27),
        RegVal(0xa2b7, 0x03),
        RegVal(0x0401, 0x00),
        RegVal(0x0404, 0x00),
        RegVal(0x0405, 0x10),
        RegVal(0x0408, 0x01),
        RegVal(0x0409, 0x5c),
        RegVal(0x040a, 0x00),
        RegVal(0x040b, 0x00),
        RegVal(0x040c, 0x05),
        RegVal(0x040d, 0x34),
        RegVal(0x040e, 0x03),
        RegVal(0x040f, 0xde),
        RegVal(0x034c, 0x05),
        RegVal(0x034d, 0x34),
        RegVal(0x034e, 0x03),
        RegVal(0x034f, 0xde),
        RegVal(0x0301, 0x05),
        RegVal(0x0303, 0x02),
        RegVal(0x0305, 0x02),
        RegVal(0x0306, 0x00),
        RegVal(0x0307, 0xaf),
        RegVal(0x0309, 0x0a),
        RegVal(0x030b, 0x02),
        RegVal(0x030d, 0x02),
        RegVal(0x030e, 0x00),
        RegVal(0x030f, 0x96),
        RegVal(0x0310, 0x01),
        RegVal(0x0820, 0x07),
        RegVal(0x0821, 0x08),
        RegVal(0x0822, 0x00),
        RegVal(0x0823, 0x00),
        RegVal(0x080a, 0x00),
        RegVal(0x080b, 0x7f),
        RegVal(0x080c, 0x00),
        RegVal(0x080d, 0x4f),
        RegVal(0x080e, 0x00),
        RegVal(0x080f, 0x77),
        RegVal(0x0810, 0x00),
        RegVal(0x0811, 0x5f),
        RegVal(0x0812, 0x00),
        RegVal(0x0813, 0x57),
        RegVal(0x0814, 0x00),
        RegVal(0x0815, 0x4f),
        RegVal(0x0816, 0x01),
        RegVal(0x0817, 0x27),
        RegVal(0x0818, 0x00),
        RegVal(0x0819, 0x3f),
        RegVal(0xe04c, 0x00),
        RegVal(0xe04d, 0x5f),
        RegVal(0xe04e, 0x00),
        RegVal(0xe04f, 0x1f),
        RegVal(0x3e20, 0x01),
        RegVal(0x3e37, 0x00),
        RegVal(0x3f50, 0x00),
        RegVal(0x3f56, 0x00),
        RegVal(0x3f57, 0xbf),
    ],
};

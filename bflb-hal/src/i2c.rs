//! Base I2C peripheral.

use core::ptr::addr_of;

use crate::arch::bl808::{I2c as I2cRegs, I2C0, I2C1, I2C2, I2C3};
use crate::arch::bl808::i2c;

// use crate::dma::{DmaSrcEndpoint, DmaDstEndpoint, DmaEndpointConfig, 
//     DmaPeripheral, DmaDataWidth, DmaBurstSize, DmaIncrement};
use crate::gpio::{Pin, PinPull, PinDrive, PinFunction, Alternate};
use crate::sealed::Sealed;
use crate::clock;


/// A generic I²C port trait, used to abstract complexity of the underlying 
/// type if you don't care, providing ways to read and write slave registers.
pub trait I2cDev {

    /// Blocking read of an I²C slave on the bus.
    fn read(&mut self, slave_addr: I2cAddr, sub_addr: Option<I2cSubAddr>, data: &mut [u8]) -> bool;
    
    /// Blocking write to an I²C slave on the bus.
    fn write(&mut self, slave_addr: I2cAddr, sub_addr: Option<I2cSubAddr>, data: &[u8]) -> bool;

}


/// Base structure for accessing I²C controller of a specific port.
pub struct I2cAccess<const PORT: u8>(pub(crate) ());

impl<const PORT: u8> I2cAccess<PORT> {

    /// Initialize the I²C port and return an initialized port to run transactions on. 
    /// Note that only **odd** pin numbers are accepted as SCL signal, and only **even**
    /// pin numbers are accepted as SDA signal. Both numbers must be between 0 and 41 
    /// included.
    pub fn init<const SCL_PIN: u8, const SDA_PIN: u8>(
        scl: impl Into<Pin<SCL_PIN, Alternate>>,
        sda: impl Into<Pin<SDA_PIN, Alternate>>,
        config: &I2cConfig,
    ) -> I2c<PORT, Pin<SCL_PIN, Alternate>, Pin<SDA_PIN, Alternate>> {

        // Valid SCL pins are ODD in range 0..=41
        // Valid SDA pins are EVEN in range 0..=41

        if SCL_PIN > 41 || SCL_PIN % 2 != 0 {
            panic!("invalid i2c scl pin {SCL_PIN}")
        } else if SDA_PIN > 41 || SDA_PIN % 2 != 1 {
            panic!("invalid i2c sda pin {SDA_PIN}")
        }

        let func = match PORT {
            0 => PinFunction::I2c0,
            1 => PinFunction::I2c1,
            2 => PinFunction::I2c2,
            3 => PinFunction::I2c3,
            _ => panic!("invalid i2c port {PORT}")
        };

        let hw_freq = match PORT {
            0 | 1 => clock::i2c::get_mcu_i2c_freq(),
            2 => clock::i2c::get_mm_i2c0_freq(),
            3 => clock::i2c::get_mm_i2c1_freq(),
            _ => unreachable!()
        };

        let mut scl = scl.into();
        let mut sda = sda.into();

        scl.modify_config(|config| {
            config.set_function(func);
            config.set_smt(true);
            config.set_drive(PinDrive::Drive0);
            config.set_pull(PinPull::Up);
        });

        sda.modify_config(|config| {
            config.set_function(func);
            config.set_smt(true);
            config.set_drive(PinDrive::Drive0);
            config.set_pull(PinPull::Up);
        });

        // hw_freq / freq / 4
        let phase = hw_freq / (config.frequency * 4);
        let tmp;

        if config.frequency <= 100_000 {
            tmp = phase / 4;
        } else {
            tmp = phase / 2;
        }

        // Setup registers
        let regs = get_registers::<PORT>();
        
        regs.config().modify(|reg| {
            reg.m_en().clear();
            reg.scl_sync_en().set(config.scl_sync as _);
            reg.deg_en().set((config.de_glitch_count != 0) as _);
            if config.de_glitch_count != 0 {
                reg.deg_cnt().set(config.de_glitch_count as u32 - 1);
            }
        });

        regs.int_sts().modify(|reg| {
            reg.end_mask().fill();
            reg.txf_mask().fill();
            reg.rxf_mask().fill();
            reg.nak_mask().fill();
            reg.arb_mask().fill();
            reg.fer_mask().fill();
        });

        let mut prd_start_stop = i2c::I2cPrd::default();
        prd_start_stop.phase_0().set(phase - tmp);
        prd_start_stop.phase_1().set(phase + tmp);
        prd_start_stop.phase_2().set(phase);
        prd_start_stop.phase_3().set(phase);
        regs.prd_start().set(prd_start_stop);
        regs.prd_stop().set(prd_start_stop);

        regs.prd_data().set_with(|reg| {
            reg.phase_0().set(phase - tmp);
            reg.phase_1().set(phase + tmp);
            reg.phase_2().set(phase + tmp);
            reg.phase_3().set(phase - tmp);
        });
        
        I2c { scl, sda }

    }

}

/// Internal marker trait to denote a valid I²C pin.
pub trait I2cPin: Sealed { }
impl<const NUM: u8> I2cPin for Pin<NUM, Alternate> {}

/// An initialized access to an I²C controller, used for read/write 
pub struct I2c<const PORT: u8, Scl: I2cPin, Sda: I2cPin> {
    /// The bus SCL pin (clock).
    scl: Scl,
    /// The bus SDA pin (data).
    sda: Sda,
}

impl<const PORT: u8, Scl: I2cPin, Sda: I2cPin> I2c<PORT, Scl, Sda> {

    /// Downgrade and close this I²C port, returning its original components.
    pub fn downgrade(self) -> (I2cAccess<PORT>, Scl, Sda) {
        // Self is dropped at the end, this effectively disable the controller.        
        unsafe { (I2cAccess(()), addr_of!(self.scl).read(), addr_of!(self.sda).read()) }
    }

    /// Internal function used to setup transfer for I²C read or write
    /// transaction.
    #[inline(never)]
    fn init_transfer(slave_addr: I2cAddr, sub_addr: Option<I2cSubAddr>, dir: I2cDirection, data_len: usize) {

        let regs = get_registers::<PORT>();

        // Setup address, sub address length and various data info.
        regs.config().modify(|reg| {
            
            if let Some(sub_addr) = &sub_addr {
                reg.sub_addr_en().fill();
                reg.sub_addr_bc().set(match sub_addr {
                    I2cSubAddr::Size1(_) => 0,
                    I2cSubAddr::Size2(_) => 1,
                    I2cSubAddr::Size3(_) => 2,
                    I2cSubAddr::Size4(_) => 3,
                });
            } else {
                reg.sub_addr_en().clear();
            }

            reg.slv_10b_addr_en().set(slave_addr.wide() as _);
            reg.slv_addr().set(slave_addr.addr() as _);

            reg.pkt_len().set((data_len - 1) as _);
            
            match dir {
                I2cDirection::Read => reg.pkt_dir().fill(),
                I2cDirection::Write => reg.pkt_dir().clear(),
            }

        });

        // Setup sub-address.
        if let Some(sub_addr) = &sub_addr {
            // Ignore the size, just write the whole number and rely 
            // on 'sub_addr_bc' initialized above.
            regs.sub_addr().set(i2c::I2cSubAddr::new(match *sub_addr {
                I2cSubAddr::Size1(v) => v,
                I2cSubAddr::Size2(v) => v,
                I2cSubAddr::Size3(v) => v,
                I2cSubAddr::Size4(v) => v,
            }));
        }

    }

    /// Internal function to enable this I²C port.
    fn enable() {
        get_registers::<PORT>().config().modify(|reg| reg.m_en().fill());
    }

    /// Internal function disable this I²C port, this also clears FIFO queues and
    /// clear interrupts.
    fn disable() {
        let regs = get_registers::<PORT>();
        regs.config().modify(|reg| reg.m_en().clear());
        regs.fifo_config_0().modify(|reg| {
            reg.tx_fifo_clr().fill();
            reg.rx_fifo_clr().fill();
        });
        regs.int_sts().modify(|reg| {
            reg.end_clr().fill();
            reg.nak_clr().fill();
            reg.arb_clr().fill();
        });
    }

    #[inline]
    fn is_enabled() -> bool {
        get_registers::<PORT>().config().get().m_en().get() != 0
    }

    #[inline]
    fn is_busy() -> bool {
        get_registers::<PORT>().bus_busy().get().bus_busy().get() != 0
    }

    #[inline]
    fn is_end() -> bool {
        get_registers::<PORT>().int_sts().get().end_int().get() != 0
    }

    #[inline]
    fn is_nak() -> bool {
        get_registers::<PORT>().int_sts().get().nak_int().get() != 0
    }

}

impl<const PORT: u8, Scl: I2cPin, Sda: I2cPin> I2cDev for I2c<PORT, Scl, Sda> {

    fn read(&mut self, slave_addr: I2cAddr, sub_addr: Option<I2cSubAddr>, data: &mut [u8]) -> bool {

        assert!(data.len() <= 256);
        Self::init_transfer(slave_addr, sub_addr, I2cDirection::Read, 1);

        Self::enable();

        let regs = get_registers::<PORT>();

        for chunk in data.chunks_mut(4) {

            while regs.fifo_config_1().get().rx_fifo_cnt().get() == 0 {
                if Self::is_nak() {
                    return false
                }
            }

            let data = regs.fifo_rdata().get().to_le_bytes();
            chunk.copy_from_slice(&data[..chunk.len()]);

        }

        while Self::is_busy() || !Self::is_end() {
            if Self::is_nak() {
                return false
            }
        }

        Self::disable();
        !Self::is_nak()

    }

    fn write(&mut self, slave_addr: I2cAddr, sub_addr: Option<I2cSubAddr>, data: &[u8]) -> bool {

        assert!(data.len() <= 256);
        Self::init_transfer(slave_addr, sub_addr, I2cDirection::Write, data.len());
        
        let regs = get_registers::<PORT>();
        
        for chunk in data.chunks(4) {

            while regs.fifo_config_1().get().tx_fifo_cnt().get() == 0 {
                if Self::is_nak() {
                    return false
                }
            }
            
            let mut chunk_padded = [0; 4];
            chunk_padded[..chunk.len()].copy_from_slice(chunk);
            regs.fifo_wdata().set(u32::from_le_bytes(chunk_padded));

            if !Self::is_enabled() {
                Self::enable();
            }

        }

        while Self::is_busy() || !Self::is_end() {
            if Self::is_nak() {
                return false
            }
        }
        
        Self::disable();
        !Self::is_nak()

    }

}

impl<const PORT: u8, Scl: I2cPin, Sda: I2cPin> Drop for I2c<PORT, Scl, Sda> {
    fn drop(&mut self) {
        Self::disable();
        get_registers::<PORT>().int_sts().modify(|reg| {
            reg.end_mask().fill();
            reg.txf_mask().fill();
            reg.rxf_mask().fill();
            reg.nak_mask().fill();
            reg.arb_mask().fill();
            reg.fer_mask().fill();
        });
    }
}


/// Represent the initial configuration of the I²C controller. The most common defaults
/// are available using the [`Default`] implementation.
#[derive(Debug)]
pub struct I2cConfig {
    /// Frequency of the I²C bus.
    pub frequency: u32,
    /// Enable signal of I²C SCL synchronization, should be enabled to support 
    /// Multi-Master and Clock-Stretching (Normally should not be turned-off).
    pub scl_sync: bool,
    /// De-glitch function cycle count (default to 0).
    pub de_glitch_count: u8,
}

impl I2cConfig {

    /// Create a new default config for the given frequency.
    pub const fn new(frequency: u32) -> Self {
        Self { 
            frequency,
            scl_sync: true, 
            de_glitch_count: 0,
        }
    }
    
}

/// Direction of the communication **from the master**.
#[derive(Debug)]
pub enum I2cDirection {
    Read,
    Write,
}

/// Describe an I²C slave address, optional wide address (10-bit).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct I2cAddr(u16);

impl I2cAddr {

    const ADDR_MASK: u16 = (1 << 10) - 1;
    const WIDE_MASK: u16 = 1 << 10;

    /// Create a new standard address. This function doesn't check for
    /// reserved addresses.
    pub const fn new(addr: u16) -> Self {
        assert!(addr <= 0b111_1111);
        Self(addr)
    }

    /// Create a new wide address (10-bit).
    pub const fn new_wide(addr: u16) -> Self {
        assert!(addr <= 0b11_1111_1111);
        Self(addr | Self::WIDE_MASK)
    }

    /// Return true if the address is wide (10-bit).
    pub const fn wide(self) -> bool {
        (self.0 | Self::WIDE_MASK) != 0
    }

    /// Return the numeric address.
    pub const fn addr(self) -> u16 {
        self.0 & Self::ADDR_MASK
    }

}

/// Different size for sub addresses.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum I2cSubAddr {
    Size1(u32) = 0,
    Size2(u32) = 1,
    Size3(u32) = 2,
    Size4(u32) = 3,
}

/// Return the I²C registers for the given port.
#[inline]
fn get_registers<const PORT: u8>() -> I2cRegs {
    match PORT {
        0 => I2C0,
        1 => I2C1,
        2 => I2C2,
        3 => I2C3,
        _ => unreachable!()
    }
}

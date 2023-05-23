//! Base I2C peripheral.

use crate::bl808::{I2c as I2cRegs, I2C0, I2C1, I2C2, I2C3};
use crate::bl808::i2c;

use crate::gpio::{Pin, PinPull, PinDrive, PinFunction, Alternate};
use crate::clock::Clocks;


/// Base structure for accessing I2C controller of a specific port.
pub struct I2cAccess<const PORT: u8>(pub(crate) ());

impl<const PORT: u8> I2cAccess<PORT> {

    /// Initialize the I2C port and return an initialized port to run
    /// transactions on.
    pub fn init<const SCL_PIN: u8, const SDA_PIN: u8>(
        mut scl: Pin<SCL_PIN, Alternate>,
        mut sda: Pin<SDA_PIN, Alternate>,
        clocks: &Clocks,
        config: &I2cConfig,
    ) -> I2c<PORT, SCL_PIN, SDA_PIN> {

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
            0 | 1 => clocks.get_mcu_i2c_freq(),
            2 => clocks.get_mm_i2c0_freq(),
            3 => clocks.get_mm_i2c1_freq(),
            _ => unreachable!()
        };

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
        regs.config().modify(|reg| reg.m_en().clear());
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


pub struct I2c<const PORT: u8, const SCL_PIN: u8, const SDA_PIN: u8> {
    scl: Pin<SCL_PIN, Alternate>,
    sda: Pin<SDA_PIN, Alternate>,
}

impl<const PORT: u8, const SCL_PIN: u8, const SDA_PIN: u8> I2c<PORT, SCL_PIN, SDA_PIN> {

    /// Blocking read of an I2C slave on the bus.
    pub fn read(&mut self, addr: I2cAddr, sub_addr: Option<I2cSubAddr>, data: &mut [u8]) -> bool {

        assert!(data.len() <= 256);
        Self::init_transfer(addr, sub_addr, I2cDirection::Read, 0);

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

    /// Blocking write to an I2C slave on the bus.
    pub fn write(&mut self, addr: I2cAddr, sub_addr: Option<I2cSubAddr>, data: &[u8]) -> bool {

        assert!(data.len() <= 256);
        Self::init_transfer(addr, sub_addr, I2cDirection::Write, data.len());
        
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

    /// Internal function used to setup transfer for I2C read or write
    /// transaction.
    #[inline(never)]
    fn init_transfer(addr: I2cAddr, sub_addr: Option<I2cSubAddr>, dir: I2cDirection, data_len: usize) {

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

            reg.slv_10b_addr_en().set(addr.wide() as _);
            reg.slv_addr().set(addr.addr() as _);

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

    #[inline]
    fn enable() {
        get_registers::<PORT>().config().modify(|reg| reg.m_en().fill());
    }

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


#[derive(Debug)]
pub struct I2cConfig {
    /// Frequency of the I2C bus.
    pub frequency: u32,
}

/// Direction of the communication **from the master**.
#[derive(Debug)]
pub enum I2cDirection {
    Read,
    Write,
}

/// Describe an I2C slave address, optional wide address (10-bit).
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


/// Return the I2C registers for the given port.
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

//! I2C controller peripheral.

embedded_util::mmio! {
    pub struct I2c {
        [0x000] rw config: I2cConfig,
        [0x004] rw int_sts: I2cIntSts,
        [0x008] rw sub_addr: I2cSubAddr,
        [0x00C] rw bus_busy: I2cBusBusy,
        /// Length of START condition phases.
        [0x010] rw prd_start: I2cPrd,
        /// Length of STOP condition phases.
        [0x014] rw prd_stop: I2cPrd,
        /// Length of DATA phases.
        [0x018] rw prd_data: I2cPrd,
        [0x080] rw fifo_config_0: I2cFifoConfig0,
        [0x084] rw fifo_config_1: I2cFifoConfig1,
        /// The FIFO write data buffer, the buffer is sent in 
        /// little-endian on the bus (LSB is sent first).
        /// 
        /// This FIFO buffer has a depth of 2 words (8 bytes).
        [0x088] wo fifo_wdata: u32,
        /// The FIFO read data buffer, the data is received in 
        /// little-endian from the bus (LSB is received first).
        /// 
        /// This FIFO buffer has a depth of 2 words (8 bytes).
        [0x08C] ro fifo_rdata: u32,
    }
}

embedded_util::reg! {
    pub struct I2cConfig: u32 {
        /// Enable signal of I2C master function. **Setting this bit 
        /// to 1 will start the transaction, and should be cleared 
        /// when finished.**
        [00..01] m_en,
        /// Transfer direction of the packet.
        /// - 0 - Write
        /// - 1 - Read
        [01..02] pkt_dir,
        /// Enable signal of I2C input de-glitch function (for all
        /// input pins).
        [02..03] deg_en,
        /// Enable signal of I2C SCL synchronization, should be enable
        /// to support Multi-Master and Clock-Stretching (normally 
        /// should not be turned-off).
        [03..04] scl_sync_en,
        /// Enable signal of I2C sub-address field.
        [04..05] sub_addr_en,
        /// Sub-address field byte count.
        /// - 0 - 1 byte
        /// - 1 - 2 bytes
        /// - 2 - 3 bytes
        /// - 3 - 4 bytes
        [05..07] sub_addr_bc, 
        /// Slave address 10-bit mode enable.
        [07..08] slv_10b_addr_en,
        /// Slave address for I2C translation. Can be either 7 bits or
        /// 10 bits depending on `slv_10b_addr_en`.
        [08..18] slv_addr,
        /// Packet length (unit: byte).
        [20..28] pkt_len,
        /// De-glitch function cycle count.
        [28..32] deg_cnt,
    }
}

embedded_util::reg! {
    pub struct I2cIntSts: u32 {
        [00..01] end_int,
        [01..02] txf_int,
        [02..03] rxf_int,
        [03..04] nak_int,
        [04..05] arb_int,
        [05..06] fer_int,
        [08..09] end_mask,
        [09..10] txf_mask,
        [10..11] rxf_mask,
        [11..12] nak_mask,
        [12..13] arb_mask,
        [13..14] fer_mask,
        [16..17] end_clr,
        [19..20] nak_clr,
        [20..21] arb_clr,
        [24..25] end_en,
        [25..26] txf_en,
        [26..27] rxf_en,
        [27..28] nak_en,
        [28..29] arb_en,
        [29..30] fer_en,
    }
}

embedded_util::reg! {
    pub struct I2cSubAddr: u32 {
        [00..08] sub_addr_b0,
        [08..16] sub_addr_b1,
        [16..24] sub_addr_b2,
        [24..32] sub_addr_b3,
    }
}

embedded_util::reg! {
    pub struct I2cBusBusy: u32 {
        [00..01] bus_busy,
        [01..02] bus_busy_clr,
    }
}

embedded_util::reg! {
    /// Description of START/STOP condition phases.
    pub struct I2cPrd: u32 {
        [00..08] phase_0,
        [08..16] phase_1,
        [16..24] phase_2,
        [24..32] phase_3,
    }
}

embedded_util::reg! {
    pub struct I2cFifoConfig0: u32 {
        /// Enable signal of dma_tx_req/ack interface.
        [00..01] dma_tx_en,
        /// Enable signal of dma_rx_req/ack interface.
        [01..02] dma_rx_en,
        /// Clear signal of TX FIFO.
        [02..03] tx_fifo_clr,
        /// Clear signal of RX FIFO.
        [03..04] rx_fifo_clr,
        /// Overflow flag of TX FIFO, can be cleared by tx_fifo_clr.
        [04..05] tx_fifo_overflow,
        /// Underflow flag of TX FIFO, can be cleared by tx_fifo_clr.
        [05..06] tx_fifo_underflow,
        /// Overflow flag of RX FIFO, can be cleared by rx_fifo_clr.
        [06..07] rx_fifo_overflow,
        /// Underflow flag of RX FIFO, can be cleared by rx_fifo_clr.
        [07..08] rx_fifo_underflow,
    }
}

embedded_util::reg! {
    pub struct I2cFifoConfig1: u32 {
        /// TX FIFO available count (in words).
        [00..02] tx_fifo_cnt,
        /// RX FIFO available count (in words).
        [08..10] rx_fifo_cnt,
        /// TX FIFO threshold, dma_tx_req will not be asserted if 
        /// tx_fifo_cnt is less than this value.
        [16..17] tx_fifo_th,
        /// RX FIFO threshold, dma_rx_req will not be asserted if
        /// rx_fifo_cnt is less than this value
        [24..25] rx_fifo_th,
    }
}

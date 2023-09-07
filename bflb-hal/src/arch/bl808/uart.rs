//! UART management registers.
//! 
//! This module is sourced from the Reference Manual.


embedded_util::mmio! {
    pub struct Uart {
        [0x00] rw utx_cfg: UartUtxCfg,
        [0x04] rw urx_cfg: UartUrxCfg,
        [0x08] rw bit_prd: UartBitPrd,
        [0x0C] rw data_cfg: UartDataCfg,
        [0x10] rw utx_ir_position: u32,
        [0x14] rw urx_ir_position: u32,
        [0x18] rw urx_rto_timer: u32,
        [0x1C] rw sw_mode: UartSwMode,
        [0x20] ro int_sts: u32,
        [0x24] rw int_mask: u32,
        [0x28] rw int_clear: u32,
        [0x2C] rw int_en: u32,
        [0x30] ro status: UartStatus,
        [0x34] ro sts_urx_abr_prd: u32,
        [0x38] ro urx_abr_prd_b01: u32,
        [0x3C] ro urx_abr_prd_b23: u32,
        [0x40] ro urx_abr_prd_b45: u32,
        [0x44] ro urx_abr_prd_b67: u32,
        [0x48] rw urx_abr_pw_tol: u32,
        [0x50] rw urx_bcr_int_cfg: u32,
        [0x54] rw utx_rs485_cfg: u32,
        [0x80] rw fifo_cfg0: UartFifoCfg0,
        [0x84] rw fifo_cfg1: UartFifoCfg1,
        [0x88] wo fifo_wdata: u8,
        [0x8C] ro fifo_rdata: u8,
    }
}

embedded_util::reg! {
    pub struct UartUtxCfg: u32 {
        /// Enable UART TX.
        [0..1] en,
        /// Enable UART TX CTS flow control function.
        [1..2] cts_en,
        /// Enable UART TX freerun mode.
        [2..3] frm_en,
        /// Enable UART TX LIN mode.
        [3..4] lin_en,
        /// Enable parity mode.
        [4..5] parity_en,
        /// Parity Odd = 1, Parity Even = 0
        [5..6] parity_sel,
        /// Enable UART TX IR mode.
        [6..7] ir_en,
        /// Inverse UART TX in IR mode.
        [7..8] ir_inv,
        /// Data bit count for each character. On 3 bit.
        [8..11] bit_count_d,
        /// Stop bit count. On 2 bit.
        [11..13] bit_count_p,
        /// Break bit count. On 3 bit.
        [13..16] bit_count_b,
        /// Length of UART TX data transfer (unit: character).
        [16..32] len,
    }
}

embedded_util::reg! {
    pub struct UartUrxCfg: u32 {
        /// Enable UART RX.
        [0..1] en,
        /// Enable UART RX Auto BaudRate detection.
        [1..2] abr_en,
        /// Enable UART TX LIN mode.
        [3..4] lin_en,
        /// Enable parity mode.
        [4..5] parity_en,
        /// Parity Odd = 1, Parity Even = 0
        [5..6] parity_sel,
        /// Enable UART TX IR mode.
        [6..7] ir_en,
        /// Inverse UART TX in IR mode.
        [7..8] ir_inv,
        /// Data bit count for each character. On 3 bit.
        [8..11] bit_count_d,
        /// Enable de-glitch function.
        [11..12] deg_en,
        /// De-glitch function cycle count. On 4 bit.
        [12..16] deg_count,
        /// Length of UART TX data transfer (unit: character).
        /// When this length is reached, a end_interrupt is asserted.
        [16..32] len,
    }
}

embedded_util::reg! {
    /// Bit period register.
    pub struct UartBitPrd: u32 {
        /// Period of each UART TX bit, related to baudrate.
        [0..16] utx_period,
        /// Period of each UART RX bit, related to baudrate.
        [16..32] urx_period,
    }
}

embedded_util::reg! {
    pub struct UartDataCfg: u32 {
        /// - 0 - Each byte is sent out LSB-first.
        /// - 1 - Each byte is sent out MSB-first.
        [0..1] bit_inv,
    }
}

embedded_util::reg! {
    pub struct UartSwMode: u32 {
        [0..1] utx_txd_sw_mode,
        [1..2] utx_txd_sw_val,
        [2..3] urx_rxd_sw_mode,
        [3..4] urx_rts_sw_val,
    }
}

embedded_util::reg! {
    pub struct UartStatus: u32 {
        [0..1] utx_bus_busy,
        [1..2] urx_bus_busy,
    }
}

embedded_util::reg! {
    pub struct UartFifoCfg0: u32 {
        /// Enable dma_tx_req/ack interface
        [0..1] dma_tx_en,
        /// Enable dma_tx_req/ack interface
        [1..2] dma_rx_en,
        /// Clear TX FIFO. Write only.
        [2..3] tx_fifo_clear,
        /// Clear RX FIFO. Write only.
        [3..4] rx_fifo_clear,
        /// Overflow flag of TX FIFO. Read only.
        [4..5] tx_fifo_overflow,
        /// Underflow flag of TX FIFO. Read only.
        [5..6] tx_fifo_underflow,
        /// Overflow flag of RX FIFO. Read only.
        [6..7] rx_fifo_overflow,
        /// Underflow flag of RX FIFO. Read only.
        [7..8] rx_fifo_underflow,
    }
}

embedded_util::reg! {
    pub struct UartFifoCfg1: u32 {
        /// TX FIFO available count.
        [0..6] tx_fifo_count,
        /// RX FIFO available count.
        [8..14] rx_fifo_count,
        /// TX FIFO threshold.
        [16..21] tx_fifo_th,
        /// RX FIFO threshold.
        [24..29] rx_fifo_th,
    }
}

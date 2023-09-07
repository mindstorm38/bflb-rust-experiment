//! 

embedded_util::mmio! {
    pub struct Gpip {
        [0x000] rw gpadc_config: GpipGpadcConfig,
        [0x004] rw gpadc_dma_rdata: GpipGpadcDmaRdata,
        [0x020] rw gpadc_pir_train: GpipGpadcPirTrain,
        [0x040] rw gpdac_config: GpipGpdacConfig,
        [0x044] rw gpdac_dma_config: GpipGpdacDmaConfig,
        [0x048] rw gpdac_dma_wdata: GpipGpdacDmaWdata,
        [0x04C] rw gpdac_tx_fifo_status: GpipGpdacTxFifoStatus,
    }
}

embedded_util::reg! {
    pub struct GpipGpadcConfig: u32 {
        [00..01] gpadc_dma_en,
        [01..02] gpadc_fifo_clr,
        [02..03] gpadc_fifo_ne,
        [03..04] gpadc_fifo_full,
        [04..05] gpadc_rdy,
        [05..06] gpadc_fifo_overrun,
        [06..07] gpadc_fifo_underrun,
        [08..09] gpadc_rdy_clr,
        [09..10] gpadc_fifo_overrun_clr,
        [10..11] gpadc_fifo_underrun_clr,
        [12..13] gpadc_rdy_mask,
        [13..14] gpadc_fifo_overrun_mask,
        [14..15] gpadc_fifo_underrun_mask,
        [16..22] gpadc_fifo_data_count,
        [22..24] gpadc_fifo_thl,
    }
}

embedded_util::reg! {
    pub struct GpipGpadcDmaRdata: u32 {
        [00..26] gpadc_dma_rdata,
    }
}

embedded_util::reg! {
    pub struct GpipGpadcPirTrain: u32 {
        [00..05] pir_extend,
        [08..13] pir_cnt_v,
        [16..17] pir_train,
        [17..18] pir_stop,
    }
}

embedded_util::reg! {
    pub struct GpipGpdacConfig: u32 {
        [00..01] gpdac_en,
        [08..11] gpdac_mode,
        [16..20] gpdac_ch_a_sel,
        [20..24] gpdac_ch_b_sel,
    }
}

embedded_util::reg! {
    pub struct GpipGpdacDmaConfig: u32 {
        [00..01] gpdac_dma_tx_en,
        [01..02] gpdac_dma_inv_msb,
        [04..08] gpdac_dma_format,
    }
}

embedded_util::reg! {
    pub struct GpipGpdacDmaWdata: u32 {
        [00..32] gpdac_dma_wdata,
    }
}

embedded_util::reg! {
    pub struct GpipGpdacTxFifoStatus: u32 {
        [00..01] tx_fifo_empty,
        [01..02] tx_fifo_full,
        [02..04] tx_cs,
        [04..08] txfifordptr,
        [08..10] txfifowrptr,
    }
}

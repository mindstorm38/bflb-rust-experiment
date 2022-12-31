//! MJPEG encoding on BL808.

use super::mmio::MJPEG;


pub const MJPEG_MAX_FRAME_COUNT: u32 = 4;


pub struct Mjpeg {

}

impl Mjpeg {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn init(&mut self, config: &MjpegConfig) {

        let resolution_x = config.resolution_x as u32;
        let resolution_y = config.resolution_y as u32;
        let aligned16_x = resolution_x % 16 != 0;
        let aligned16_y = resolution_y % 16 != 0;

        // Stop module before initialization.
        MJPEG.mjpeg_control_1().modify(|reg| reg.mjpeg_enable().clear());

        // Contrigure input format.
        MJPEG.mjpeg_control_1().set_with(|reg| {

            // Burst length of 16
            reg.burst_len().set(3);
            reg.read_fwrap().fill();
            reg.mjpeg_bit_order().fill();

            match config.input_format {
                InputFormat::Gray => self.set_frame_size((resolution_x + 7) >> 3, (resolution_y + 7) >> 3),
                InputFormat::Yuv422yuyv |
                InputFormat::Yuv422yvyu |
                InputFormat::Yuv422uyvy |
                InputFormat::Yuv422vyuy |
                InputFormat::Yuy422spNv16 |
                InputFormat::Yuy422spNv61 |
                InputFormat::Yuy422spNv21 => self.set_frame_size((resolution_x + 15) >> 4, (resolution_y + 7) >> 3),
                InputFormat::Yuy422spNv12 => self.set_frame_size((resolution_x + 15) >> 4, (resolution_y + 15) >> 4),
            }

            match config.input_format {
                InputFormat::Gray => reg.yuv_mode().set(1),
                InputFormat::Yuv422yuyv |
                InputFormat::Yuv422yvyu |
                InputFormat::Yuv422uyvy |
                InputFormat::Yuv422vyuy => reg.yuv_mode().set(3),
                InputFormat::Yuy422spNv16 |
                InputFormat::Yuy422spNv61 => reg.yuv_mode().set(2),
                InputFormat::Yuy422spNv12 |
                InputFormat::Yuy422spNv21 => reg.yuv_mode().set(0),
            }

            match config.input_format {
                InputFormat::Gray => {
                    if aligned16_x { reg.last_hf_wblk_dmy().fill(); }
                    if aligned16_y { reg.last_hf_hblk_dmy().fill(); }
                }
                InputFormat::Yuv422yuyv => self.set_yuv422_interleave(0, 1, 2, 3),
                InputFormat::Yuv422yvyu => self.set_yuv422_interleave(0, 3, 2, 1),
                InputFormat::Yuv422uyvy => self.set_yuv422_interleave(1, 0, 3, 2),
                InputFormat::Yuv422vyuy => self.set_yuv422_interleave(1, 2, 3, 0),
                InputFormat::Yuy422spNv16 => reg.order_u_even().fill(),
                InputFormat::Yuy422spNv61 => (),
                InputFormat::Yuy422spNv12 |
                InputFormat::Yuy422spNv21 => {
                    reg.order_u_even().fill();
                    if aligned16_x { reg.last_hf_wblk_dmy().fill(); }
                }
            }

        });

        MJPEG.mjpeg_control_2().modify(|reg| {
            reg.mjpeg_wait_cycle().set(0x100);
            reg.mjpeg_sw_mode().clear();
        });

        MJPEG.mjpeg_swap_mode().modify(|reg| {
            reg.w_swap_mode().clear();
        });

        MJPEG.mjpeg_yy_frame_addr().set(config.input_bufaddr_yy);
        MJPEG.mjpeg_uv_frame_addr().set(config.input_bufaddr_uv);

        let rows = MJPEG_MAX_FRAME_COUNT * resolution_y / 8;
        let min_frame_size;

        match config.input_format {
            InputFormat::Gray => {
                min_frame_size = resolution_x * resolution_y * MJPEG_MAX_FRAME_COUNT;
            }
            InputFormat::Yuv422yuyv |
            InputFormat::Yuv422yvyu |
            InputFormat::Yuv422uyvy |
            InputFormat::Yuv422vyuy |
            InputFormat::Yuy422spNv16 |
            InputFormat::Yuy422spNv61 => {
                min_frame_size = resolution_x * resolution_y * 2 * MJPEG_MAX_FRAME_COUNT;
            }
            InputFormat::Yuy422spNv12 |
            InputFormat::Yuy422spNv21 => {
                min_frame_size = resolution_x * resolution_y * 3 / 2 * MJPEG_MAX_FRAME_COUNT;
            }
        }

        assert!(min_frame_size <= config.output_bufsize, "minimum frame size is greater than given output buffer size");

        MJPEG.mjpeg_yuv_mem().set_with(|reg| {
            reg.yy_mem_hblock().set(rows);
            match config.input_format {
                InputFormat::Yuy422spNv16 |
                InputFormat::Yuy422spNv61 |
                InputFormat::Yuy422spNv12 |
                InputFormat::Yuy422spNv21 => reg.uv_mem_hblock().set(rows),
                _ => ()
            }
        });

        MJPEG.jpeg_frame_addr().set(config.output_bufaddr);
        MJPEG.jpeg_store_memory().set(config.output_bufsize / 128);

        MJPEG.mjpeg_control_3().modify(|reg| {
            reg.int_normal_en().clear();
            reg.int_cam_en().clear();
            reg.int_mem_en().clear();
            reg.int_frame_en().clear();
            reg.int_idle_en().clear();
            reg.int_swap_en().clear();
            // Set to trigger interruption with one frame.
            reg.frame_count_trigger_int().set(1);
        });

        MJPEG.mjpeg_header_byte().modify(|reg| {
            reg.head_byte().clear();
            reg.tail_exp().clear();
        });

        MJPEG.mjpeg_frame_fifo_pop().modify(|reg| {
            reg.int_normal_clear().clear();
            reg.int_cam_clear().clear();
            reg.int_mem_clear().clear();
            reg.int_frame_clear().clear();
            reg.int_idle_clear().clear();
            reg.int_swap_clear().clear();
        });

        let mut yy_table = [0; 64];
        let mut uv_table = [0; 64];
        config.calc_quantize_table(&mut yy_table, &mut uv_table);
        self.set_quantize_table(&yy_table, &uv_table);

    }

    /// Set the interleave order for YUV format.
    /// 
    /// *This function is used by [`init`] depending on the given 
    /// configuration. You can change this afterward if needed.*
    pub fn set_yuv422_interleave(&mut self, y0: u8, u0: u8, y1: u8, v0: u8) {
        MJPEG.mjpeg_header_byte().modify(|reg| {
            reg.y0_order().set(y0 as _);
            reg.u0_order().set(u0 as _);
            reg.y1_order().set(y1 as _);
            reg.v0_order().set(v0 as _);
        });
    }

    /// Set the configured frame size.
    /// 
    /// *This function is used by [`init`] depending on the given 
    /// configuration. You can change this afterward if needed.*
    pub fn set_frame_size(&mut self, x: u32, y: u32) {
        MJPEG.mjpeg_frame_size().modify(|reg| {
            reg.frame_wblock().set(x);
            reg.frame_hblock().set(y);
        });
    }

    /// Set the quantization tables.
    /// 
    /// *This function is used by [`init`] depending on the given 
    /// configuration. You can change this afterward if needed.*
    pub fn set_quantize_table(&mut self, yy: &[u16; 64], uv: &[u16; 64]) {

        fn fill_table(input: &[u16; 64], output: &mut [u16; 64]) {
            for i in 0..8 {
                for j in 0..4 {
                    let mut tmp1 = 2048 / input[16 + j + i];
                    let mut tmp2 = 2048 / input[16 * j + i + 8];
                    if 20480 / input[16 + j + i] % 10 > 4 {
                        tmp1 += 1;
                    }
                    if 20480 / input[16 * j + i + 8] % 10 > 4 {
                        tmp2 += 1;
                    }
                    let index = (i * 4 + j) * 2;
                    output[index + 0] = tmp1;
                    output[index + 1] = tmp2;
                }
            }
        }

        MJPEG.mjpeg_q_param_00().modify(|output| fill_table(yy, output));
        MJPEG.mjpeg_q_param_40().modify(|output| fill_table(uv, output));
        MJPEG.mjpeg_q_enc().modify(|reg| reg.q_sram_sw().fill());

    }

}


/// Configuration for MJPEG decoder module.
#[derive(Debug, Clone, Copy)]
pub struct MjpegConfig {
    /// X resolution, must be a multiple of 8 or 16.
    pub resolution_x: u16,
    /// Y resolution must be a multiple of 8 or 16.
    pub resolution_y: u16,
    /// Input format for the MJPEG decoder.
    pub input_format: InputFormat,
    /// MJPEG quality.
    pub quality: u8,
    /// Input buffer address 0 for YY, must be align 16.
    pub input_bufaddr_yy: u32,
    /// Input buffer address 1 for UV, must be align 16.
    pub input_bufaddr_uv: u32,
    /// Output buffer address, must be align 16.
    pub output_bufaddr: u32,
    /// Output buffer size, must be larger than 
    /// `resolution_x * resolution_y * 2 * MJPEG_MAX_FRAME_COUNT`.
    pub output_bufsize: u32,
    /// Input Y quantization table.
    pub input_yy_table: &'static [u16; 64],
    /// Input UV quantization table.
    pub input_uv_table: &'static [u16; 64],
}

impl MjpegConfig {

    /// Create a default MJPEG configuration.
    /// This configuration is not usable.
    pub const fn new() -> Self {
        Self {
            resolution_x: 0,
            resolution_y: 0,
            input_format: InputFormat::Gray,
            quality: 0,
            input_bufaddr_yy: 0,
            input_bufaddr_uv: 0,
            output_bufaddr: 0,
            output_bufsize: 0,
            input_yy_table: &[
                16, 11, 10, 16, 24,  40,  51,  61,
                12, 12, 14, 19, 26,  58,  60,  55,
                14, 13, 16, 24, 40,  57,  69,  56,
                14, 17, 22, 29, 51,  87,  80,  62,
                18, 22, 37, 56, 68,  109, 103, 77,
                24, 35, 55, 64, 81,  104, 113, 92,
                49, 64, 78, 87, 103, 121, 120, 101,
                72, 92, 95, 98, 112, 100, 103, 99
            ],
            input_uv_table: &[
                17, 18, 24, 47, 99, 99, 99, 99,
                18, 21, 26, 66, 99, 99, 99, 99,
                24, 26, 56, 99, 99, 99, 99, 99,
                47, 66, 99, 99, 99, 99, 99, 99,
                99, 99, 99, 99, 99, 99, 99, 99,
                99, 99, 99, 99, 99, 99, 99, 99,
                99, 99, 99, 99, 99, 99, 99, 99,
                99, 99, 99, 99, 99, 99, 99, 99
            ],
        }
    }

    /// Calculate the real quantization table for Luminance and Chrominance
    /// depending on the configured quality and input tables.
    pub fn calc_quantize_table(&self, yy_table: &mut [u16; 64], uv_table: &mut [u16; 64]) {
        
        fn inner(quality: u8, input: &[u16; 64], output: &mut [u16; 64]) {

            let quality = quality.clamp(1, 100);
            let scale_factor = if quality < 50 {
                5_000_000 / quality as u32
            } else {
                200_000 - quality as u32 * 2000
            };

            for (inp, out) in input.iter().zip(output) {
                *out = (((*inp as u32 * scale_factor + 50_000) / 100_000) as u16).clamp(1, 0xFF);
            }

        }

        inner(self.quality, self.input_yy_table, yy_table);
        inner(self.quality, self.input_uv_table, uv_table);

    }

}

/// Format configuration for MJPEG input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Gray,
    Yuv422yuyv,
    Yuv422yvyu,
    Yuv422uyvy,
    Yuv422vyuy,
    Yuy422spNv16,
    Yuy422spNv61,
    Yuy422spNv12,
    Yuy422spNv21,
}

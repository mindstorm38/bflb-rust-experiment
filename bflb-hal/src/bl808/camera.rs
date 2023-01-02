//! Camera management on BL808.

use super::mmio::{self, Cam, CAM_FRONT, CSI};
use super::clock::Clocks;
use super::AsCoreId;


/// Abstract camera controller structure.
pub struct Camera<C> {
    port: CameraPort,
    cam: Cam,
    #[allow(unused)]
    core_id: C,
}

impl<C: AsCoreId> Camera<C> {
    
    pub fn new(port: CameraPort, core_id: C) -> Self {
        Self {
            port,
            cam: match port {
                CameraPort::Port0 => mmio::CAM0,
                CameraPort::Port1 => mmio::CAM1,
                CameraPort::Port2 => mmio::CAM2,
                CameraPort::Port3 => mmio::CAM3,
                CameraPort::Port4 => mmio::CAM4,
                CameraPort::Port5 => mmio::CAM5,
                CameraPort::Port6 => mmio::CAM6,
                CameraPort::Port7 => mmio::CAM7,
            },
            core_id,
        }
    }

    pub fn init(&mut self, config: CameraConfig, clocks: &Clocks<C>) {

        let mut resolution_x = config.resolution_x as u32;
        let mut resolution_y = config.resolution_y as u32;
        let mut tmp_val;

        let _ = clocks;

        match config.input_source {
            InputSource::Dvp => {
                tmp_val = 0x15;
                CAM_FRONT.pix_data_ctrl().modify(|reg| reg.isp_dtsrc_src().clear());
            }
            InputSource::Csi => {
                tmp_val = 0x24;
                CAM_FRONT.pix_data_ctrl().modify(|reg| reg.isp_dtsrc_src().fill());
                CAM_FRONT.adj_a_ctrl_2().modify(|reg| *reg |= 1);
            }
        }

        self.cam.dvp2axi_addr_start().set(config.output_bufaddr);
        self.cam.dvp2axi_fram_exm().set_with(|reg| {
            reg.resolution_x().set(resolution_x);
            reg.resolution_y().set(resolution_y);
        });

        self.cam.dvp_debug().set(0);

        let mut hsync_crop = self.cam.dvp2axi_hsync_crop().get();
        if hsync_crop.crop_end().get() > resolution_x {
            resolution_x = hsync_crop.crop_end().get() - hsync_crop.crop_start().get();
        }

        let mut vsync_crop = self.cam.dvp2axi_vsync_crop().get();
        if vsync_crop.crop_end().get() > resolution_y {
            resolution_y = vsync_crop.crop_end().get() - vsync_crop.crop_start().get();
        }

        if config.input_source != InputSource::Csi {
            todo!() // TODO:
        }

        let mut dvp2axi_cfg = self.cam.dvp2axi_cfg().get();
        dvp2axi_cfg.sw_mode().clear();
        dvp2axi_cfg.drop_enable().clear();
        dvp2axi_cfg.drop_even().clear();
        dvp2axi_cfg.dvp_data_bsel().clear();
        dvp2axi_cfg.vertical_subsample_enable().clear();
        dvp2axi_cfg.vertical_subsample_polarity().clear();

        // Default pixel size if set to 1 byte.
        let mut pixel_size = 1;
        let mut data_mode = 0;

        match (config.input_format, config.output_format) {
            (
                InputFormat::Yuv422yuyv | 
                InputFormat::Yuv422uyvy, 
                OutputFormat::RgbBgr888 | 
                OutputFormat::Rgb888ToRgb565 | 
                OutputFormat::Rgb888ToBgr565 | 
                OutputFormat::Rgb888ToRgba8888
            ) => { // Matches YUYV/UYVY -> RGB(A)/BGR

                tmp_val = 0x23;

                cam_front_swap_yu_order(config.input_format == InputFormat::Yuv422uyvy);

                CAM_FRONT.y2ra_cfg0().set_with(|reg| {
                    reg.y2ra_en().set(1);
                    match config.input_source {
                        InputSource::Dvp => reg.y2ra_sel().set(10),
                        InputSource::Csi => reg.y2ra_sel().set(1),
                    }
                });
            
                CAM_FRONT.y2ra_cfg1().set_with(|reg| {
                    reg.y2ra_pre_1().set(0b110000000);
                });
            
                CAM_FRONT.y2ra_cfg2().set_with(|reg| {
                    reg.y2ra_pre_2().set(0b110000000);
                });
            
                CAM_FRONT.y2ra_cfg3().set_with(|reg| {
                    reg.y2ra_mtx_00().set(0x200);
                });
            
                CAM_FRONT.y2ra_cfg4().set_with(|reg| {
                    reg.y2ra_mtx_02().set(0x2CE);
                    reg.y2ra_mtx_10().set(0x200);
                });
            
                CAM_FRONT.y2ra_cfg5().set_with(|reg| {
                    reg.y2ra_mtx_11().set(0xF50);
                    reg.y2ra_mtx_12().set(0xE92);
                });
            
                CAM_FRONT.y2ra_cfg6().set_with(|reg| {
                    reg.y2ra_mtx_20().set(0x200);
                    reg.y2ra_mtx_21().set(0x38B);
                });
            
                CAM_FRONT.y2ra_cfg7().set_with(|reg| {
                    reg.y2ra_mtx_22().set(0);
                });

                // Output format delegated after the current match.

            }
            (
                InputFormat::Yuv422yuyv | 
                InputFormat::Yuv422uyvy | 
                InputFormat::Yuv422yvyu |
                InputFormat::Yuv422vyuy,
                OutputFormat::Auto |
                OutputFormat::Gray |
                OutputFormat::Yuv422 |
                OutputFormat::Yuv422uv |
                OutputFormat::Yuv420uv
            ) => { // Matches YUV -> YUV/GRAY

                // not called? cam_front_swap_yu_order(...)

                match config.output_format {
                    OutputFormat::Auto | OutputFormat::Yuv422 => {
                        data_mode = 0;  // 16-bit wide
                        pixel_size = 2;
                    }
                    OutputFormat::Gray => {
                        data_mode = 4;  // 8-bit wide
                        pixel_size = 1;
                    }
                    OutputFormat::Yuv422uv => {
                        dvp2axi_cfg.dvp_data_bsel().set(1);
                        data_mode = 4;  // 8-bit wide
                        pixel_size = 1;
                    }
                    OutputFormat::Yuv420uv => {
                        dvp2axi_cfg.dvp_data_bsel().set(1);
                        dvp2axi_cfg.vertical_subsample_enable().fill();
                        dvp2axi_cfg.vertical_subsample_polarity().fill();
                        data_mode = 4;  // 8-bit wide
                        pixel_size = 0; // DIVIDE BY 2
                    }
                    _ => unreachable!("because of pattern")
                }

            }
            (
                InputFormat::Gray, 
                OutputFormat::Auto | 
                OutputFormat::Gray
            ) => { // Matches GRAY -> GRAY

                data_mode = 0; // 16-bit wide
                pixel_size = 1;

            }
            (
                InputFormat::Rgb565 | 
                InputFormat::Bgr565, 
                OutputFormat::Auto | 
                OutputFormat::RgbBgr565
            ) => { // Matches RGB565/BGR565 -> RGB565/BGR565

                data_mode = 0; // 16-bit wide
                pixel_size = 2;

            }
            (
                InputFormat::Rgb888 | 
                InputFormat::Bgr888, 
                OutputFormat::Auto | 
                OutputFormat::RgbBgr888 |
                OutputFormat::Rgb888ToRgb565 |
                OutputFormat::Rgb888ToBgr565 |
                OutputFormat::Rgb888ToRgba8888
            ) => { // Matches RGB888/BGR888 -> RGB888/BGR888 (to RGB565/BGR565/RGB888/RGBA8888)
                
                // Output format delegated after the current match.

            }
            (input, output) => {
                unimplemented!("'{output:?}' output format is not supported with '{input:?}' input format");
            }
        }

        // The RGB/BGR output formats matching is delegated here.
        match config.output_format {
            OutputFormat::RgbBgr888 => {
                data_mode = 1;  // 24-bit mode
                pixel_size = 3;
            }
            OutputFormat::Rgb888ToRgb565 => {
                self.cam.dvp2axi_misc().set_with(|reg| {
                    reg.format_565().set(5);
                });
                data_mode = 2;  // 24-comp-16-bit mode
                pixel_size = 2;
            }
            OutputFormat::Rgb888ToBgr565 => {
                self.cam.dvp2axi_misc().set_with(|reg| {
                    reg.format_565().set(0);
                });
                data_mode = 2;  // 24-comp-16-bit mode
                pixel_size = 2;
            }
            OutputFormat::Rgb888ToRgba8888 => {
                data_mode = 3;  // 24-exp-32-bit mode
                pixel_size = 4;
            }
            _ => {}
        }

        // Compute the total frame size in bytes.
        let frame_size;
        if pixel_size == 0 {
            frame_size = resolution_x * resolution_y / 2;
        } else {
            frame_size = resolution_x * resolution_y * pixel_size;
        }

        // Write back data mode and config/frame size
        dvp2axi_cfg.dvp_data_mode().set(data_mode);
        self.cam.dvp2axi_frame_size().set(frame_size);
        self.cam.dvp2axi_cfg().set(dvp2axi_cfg);

        // Modify again the cfg register for burst count.
        let mem_size = match self.cam.dvp2axi_cfg().get().burst_len().get() {
            0 => config.output_bufsize >> 3, // INCR1
            1 => config.output_bufsize >> 5, // INCR4
            2 => config.output_bufsize >> 6, // INCR8
            3 => config.output_bufsize >> 7, // INCR16
            5 => config.output_bufsize >> 8, // INCR32
            6 => config.output_bufsize >> 9, // INCR64
            _ => unimplemented!("unsupported burst len constant")
        };
        self.cam.dvp2axi_mem_size().set(mem_size);

        // Update dvp selector depending on the port.
        match self.port {
            CameraPort::Port0 |
            CameraPort::Port1 |
            CameraPort::Port2 |
            CameraPort::Port3 => {
                CAM_FRONT.dvp2bus_src_sel_1().modify(|reg| {
                    match self.port {
                        CameraPort::Port0 => reg.d2b_dvp_sel_a().set(tmp_val),
                        CameraPort::Port1 => reg.d2b_dvp_sel_b().set(tmp_val),
                        CameraPort::Port2 => reg.d2b_dvp_sel_c().set(tmp_val),
                        CameraPort::Port3 => reg.d2b_dvp_sel_d().set(tmp_val),
                        _ => {} // Unreachable
                    }
                });
            }
            CameraPort::Port4 |
            CameraPort::Port5 |
            CameraPort::Port6 |
            CameraPort::Port7 => {
                CAM_FRONT.dvp2bus_src_sel_2().modify(|reg| {
                    match self.port {
                        CameraPort::Port4 => reg.d2b_dvp_sel_e().set(tmp_val),
                        CameraPort::Port5 => reg.d2b_dvp_sel_f().set(tmp_val),
                        CameraPort::Port6 => reg.d2b_dvp_sel_g().set(tmp_val),
                        CameraPort::Port7 => reg.d2b_dvp_sel_h().set(tmp_val),
                        _ => {} // Unreachable
                    }
                });
            }
        }

    }

    /// Start the camera.
    pub fn start(&mut self) {

        self.cam.dvp2axi_cfg().modify(|reg| {
            reg.enable().fill();
        });

        if CAM_FRONT.pix_data_ctrl().get().isp_dtsrc_src().get() != 0 {
            CAM_FRONT.cfg().modify(|reg| {
                reg.dvpas_enable().fill();
            });
        }

    }

    /// Stop the camera.
    pub fn stop(&mut self) {

        self.cam.dvp2axi_cfg().modify(|reg| {
            reg.enable().clear();
        });

        CAM_FRONT.cfg().modify(|reg| {
            reg.dvpas_enable().clear();
        });

    }

}


/// Internal function for common setup of YUYV/UYVY input formats.
fn cam_front_swap_yu_order(swap: bool) {
    // Change output format to UYVY.
    CAM_FRONT.cfg().modify(|reg| {
        reg.dvpas_da_order().set(swap as _);
    });
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraPort {
    Port0,
    Port1,
    Port2,
    Port3,
    Port4,
    Port5,
    Port6,
    Port7,
}


#[derive(Debug, Clone, Copy)]
pub struct CameraConfig {
    /// X resolution.
    pub resolution_x: u16,
    /// Y resolution.
    pub resolution_y: u16,
    /// Hsync blank.
    pub h_blank: u16,
    /// Pixel clock.
    pub pixel_clock: u32,
    /// With MJPEG or not.
    pub with_mjpeg: bool,
    /// Input format of the camera.
    pub input_format: InputFormat,
    /// Input source.
    pub input_source: InputSource,
    /// Output format.
    pub output_format: OutputFormat,
    /// Output buffer address, must be aligned to 16.
    pub output_bufaddr: u32,
    /// Output buffer size, must not be less than one frame size.
    pub output_bufsize: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    /// Input YUYV
    Yuv422yuyv,
    /// Input YVYU
    Yuv422yvyu,
    /// Input UYVY (inverted YUYV)
    Yuv422uyvy,
    /// Input VYUY (inverted YVYU)
    Yuv422vyuy,
    Gray,
    Rgb565,
    Bgr565,
    Rgb888,
    Bgr888,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSource {
    Dvp,
    Csi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Depending on the input format.
    Auto,
    /// Only chrominance Y.
    Gray,
    /// 4 bits Y, 2 bits U/V.
    Yuv422,
    /// 4 bits Y, 2 bits U/V.
    Yuv422uv,
    /// 4 bits Y, 1 bits U/V.
    Yuv420uv,
    RgbBgr565,
    RgbBgr888,
    Rgb888ToRgb565,
    Rgb888ToBgr565,
    Rgb888ToRgba8888,
}


/// CSI controller to use for cameras.
pub struct Csi {
    /// Current configured lane count.
    lane_count: Option<CsiLaneCount>,
}

impl Csi {

    /// Create a new CSI controller.
    /// 
    /// **You must** ensure that only a single instance of this structure
    /// exists at the same time.
    pub fn new() -> Self {
        Self {
            lane_count: None,
        }
    }

    /// Initialize and configure the MIPI CSI module.
    pub fn init(&mut self, config: &CsiConfig) {

        self.lane_count = Some(config.lane_count);

        CSI.mipi_config().modify(|reg| {

            reg.cr_vc_dvp0().set(config.dvp_vc_num);
            reg.cr_vc_dvp1().set(1);

            reg.cr_unpack_en().set(config.unpack_enable as _);
            reg.cr_sync_sp_en().set(config.sync_sp_enable as _);
            reg.cr_data_bit_inv().set(config.data_bit_inv_enable as _);

            reg.cr_lane_num().set(match config.lane_count {
                CsiLaneCount::Rx1Lane => 0,
                CsiLaneCount::Rx2Lane => 1,
            });

        });

    }

    /// Enable CSI module.
    pub fn enable(&mut self) {
        CSI.mipi_config().modify(|reg| reg.cr_csi_en().fill());
    }

    /// Disable CSI module.
    pub fn disable(&mut self) {
        CSI.mipi_config().modify(|reg| reg.cr_csi_en().clear())
    }

    /// Enable CSI differencial pairs.
    pub fn enable_dphy(&mut self) {
        CSI.dphy_config_0().modify(|reg| {
            
        });
    }

}


/// Configuration structure for CSI module, used when initializing it.
#[derive(Debug, Clone)]
pub struct CsiConfig {
    /// Number of lanes to use for CSI.
    pub lane_count: CsiLaneCount,
    /// Number of the DVP ??.
    pub dvp_vc_num: u32,
    /// Enable CSI unpacking or not.
    pub unpack_enable: bool,
    /// Enable Sync ??.
    pub sync_sp_enable: bool,
    /// Enable inverted data bit ??.
    pub data_bit_inv_enable: bool,
}


/// Restricted number of CSI lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsiLaneCount {
    Rx1Lane,
    Rx2Lane,
}

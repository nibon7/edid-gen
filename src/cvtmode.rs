use super::Version;

bitflags! {
    struct CvtModeFlags: u32 {
        const PHSYNC = 1<<0;
        const NHSYNC = 1<<1;
        const PVSYNC = 1<<2;
        const NVSYNC = 1<<3;
        const INTERLACE = 1<<4;
    }
}

impl Default for CvtModeFlags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug)]
enum XYRatio {
    Ratio16_10,
    Ratio4_3,
    Ratio5_4,
    Ratio16_9,
    RatioUnknown,
}

impl ToString for XYRatio {
    fn to_string(&self) -> String {
        match *self {
            Self::Ratio16_10 => "XY_RATIO_16_10".to_owned(),
            Self::Ratio4_3 => "XY_RATIO_4_3".to_owned(),
            Self::Ratio5_4 => "XY_RATIO_5_4".to_owned(),
            Self::Ratio16_9 => "XY_RATIO_16_9".to_owned(),
            Self::RatioUnknown => "RATION UNKNOWN".to_owned(),
        }
    }
}

impl Default for XYRatio {
    fn default() -> Self {
        Self::RatioUnknown
    }
}

#[derive(Debug, Default)]
pub struct CvtMode {
    clock: u64,
    hdisplay: i32,
    hsync_start: i32,
    hsync_end: i32,
    htotal: i32,
    vdisplay: i32,
    vsync_start: i32,
    vsync_end: i32,
    vtotal: i32,
    vrefresh: i32,
    flags: CvtModeFlags,
    xy_ratio: XYRatio,
}

impl CvtMode {
    // stolen from linux/drivers/gpu/drm/drm_modes.c
    pub fn new(
        hdisplay: i32,
        vdisplay: i32,
        _vrefresh: i32,
        reduced: bool,
        interlaced: bool,
        margins: bool,
    ) -> Self {
        let mut mode = CvtMode::default();

        const HV_FACTOR: i32 = 1000;
        /* 1) top/bottom margin size (% of height) - default: 1.8, */
        const CVT_MARGIN_PERCENTAGE: i32 = 18;
        /* 2) character cell horizontal granularity (pixels) - default 8 */
        const CVT_H_GRANULARITY: i32 = 8;
        /* 3) Minimum vertical porch (lines) - default 3 */
        const CVT_MIN_V_PORCH: i32 = 3;
        /* 4) Minimum number of vertical back porch lines - default 6 */
        const CVT_MIN_V_BPORCH: i32 = 6;
        /* Pixel Clock step (kHz) */
        const CVT_CLOCK_STEP: i32 = 250;

        let vfieldrate: u32;
        let hperiod: u32;
        let hdisplay_rnd: i32;
        let mut hmargin: i32;
        let vdisplay_rnd: i32;
        let vmargin: i32;
        let vsync: i32;
        let interlace: i32;
        let mut tmp1: i32;
        let tmp2: i32;

        /* the CVT default refresh rate is 60Hz */
        let vrefresh = match _vrefresh {
            x if x <= 0 => 60,
            _ => _vrefresh,
        };

        /* the required field fresh rate */
        if interlaced {
            vfieldrate = vrefresh as u32 * 2;
        } else {
            vfieldrate = vrefresh as u32;
        }

        /* horizontal pixels */
        hdisplay_rnd = hdisplay - (hdisplay % CVT_H_GRANULARITY);

        /* determine the left&right borders */
        if margins {
            hmargin = hdisplay_rnd * CVT_MARGIN_PERCENTAGE / 1000;
            hmargin -= hmargin % CVT_H_GRANULARITY;
        } else {
            hmargin = 0;
        }

        /* find the total active pixels */
        mode.hdisplay = hdisplay_rnd + 2 * hmargin;

        /* find the number of lines per field */
        if interlaced {
            vdisplay_rnd = vdisplay / 2;
        } else {
            vdisplay_rnd = vdisplay;
        }

        /* find the top & bottom borders */
        if margins {
            vmargin = vdisplay_rnd * CVT_MARGIN_PERCENTAGE / 1000;
        } else {
            vmargin = 0;
        }

        mode.vdisplay = vdisplay + 2 * vmargin;

        /* Interlaced */
        if interlaced {
            interlace = 1;
        } else {
            interlace = 0;
        }

        /* Determine VSync Width from aspect ratio */
        if ((vdisplay % 3) == 0) && ((vdisplay * 4 / 3) == hdisplay) {
            mode.xy_ratio = XYRatio::Ratio4_3;
            vsync = 4;
        } else if ((vdisplay % 9) == 0) && ((vdisplay * 16 / 9) == hdisplay) {
            mode.xy_ratio = XYRatio::Ratio16_9;
            vsync = 5;
        } else if ((vdisplay % 10) == 0) && ((vdisplay * 16 / 10) == hdisplay) {
            mode.xy_ratio = XYRatio::Ratio16_10;
            vsync = 6;
        } else if ((vdisplay % 4) == 0) && ((vdisplay * 5 / 4) == hdisplay) {
            mode.xy_ratio = XYRatio::Ratio5_4;
            vsync = 7;
        } else if ((vdisplay % 9) == 0) && ((vdisplay * 15 / 9) == hdisplay) {
            mode.xy_ratio = XYRatio::RatioUnknown;
            vsync = 7;
        } else {
            /* custom */
            mode.xy_ratio = XYRatio::RatioUnknown;
            vsync = 10;
        }

        if !reduced {
            /* simplify the GTF calculation */
            /* 4) Minimum time of vertical sync + back porch interval (µs)
             * default 550.0
             */
            const CVT_MIN_VSYNC_BP: i32 = 550;
            /* 3) Nominal HSync width (% of line period) - default 8 */
            const CVT_HSYNC_PERCENTAGE: i32 = 8;
            let mut hblank_percentage: u32;
            let vsyncandback_porch: i32;
            let _vback_porch: i32;
            let mut hblank: i32;

            /* estimated the horizontal period */
            tmp1 = HV_FACTOR * 1000000 - CVT_MIN_VSYNC_BP * HV_FACTOR * vfieldrate as i32;
            tmp2 = (vdisplay_rnd + 2 * vmargin + CVT_MIN_V_PORCH) * 2 + interlace;
            hperiod = tmp1 as u32 * 2 / (tmp2 as u32 * vfieldrate);

            tmp1 = CVT_MIN_VSYNC_BP * HV_FACTOR / hperiod as i32 + 1;
            /* 9. Find number of lines in sync + backporch */
            if tmp1 < (vsync + CVT_MIN_V_PORCH) {
                vsyncandback_porch = vsync + CVT_MIN_V_PORCH;
            } else {
                vsyncandback_porch = tmp1;
            }
            /* 10. Find number of lines in back porch */
            _vback_porch = vsyncandback_porch - vsync;
            mode.vtotal = vdisplay_rnd + 2 * vmargin + vsyncandback_porch + CVT_MIN_V_PORCH;
            /* 5) Definition of Horizontal blanking time limitation */
            /* Gradient (%/kHz) - default 600 */
            const CVT_M_FACTOR: i32 = 600;
            /* Offset (%) - default 40 */
            const CVT_C_FACTOR: i32 = 40;
            /* Blanking time scaling factor - default 128 */
            const CVT_K_FACTOR: i32 = 128;
            /* Scaling factor weighting - default 20 */
            const CVT_J_FACTOR: i32 = 20;
            const CVT_M_PRIME: i32 = CVT_M_FACTOR * CVT_K_FACTOR / 256;
            const CVT_C_PRIME: i32 =
                (CVT_C_FACTOR - CVT_J_FACTOR) * CVT_K_FACTOR / 256 + CVT_J_FACTOR;
            /* 12. Find ideal blanking duty cycle from formula */
            hblank_percentage =
                CVT_C_PRIME as u32 * HV_FACTOR as u32 - CVT_M_PRIME as u32 * hperiod / 1000;
            /* 13. Blanking time */
            if hblank_percentage < 20 * HV_FACTOR as u32 {
                hblank_percentage = 20 * HV_FACTOR as u32;
            }

            hblank = ((mode.hdisplay as u32 * hblank_percentage)
                / (100 * HV_FACTOR as u32 - hblank_percentage)) as i32;
            hblank -= hblank % (2 * CVT_H_GRANULARITY);
            /* 14. find the total pixels per line */
            mode.htotal = mode.hdisplay + hblank;
            mode.hsync_end = mode.hdisplay + hblank / 2;
            mode.hsync_start = mode.hsync_end - (mode.htotal * CVT_HSYNC_PERCENTAGE) / 100;
            mode.hsync_start += CVT_H_GRANULARITY - mode.hsync_start % CVT_H_GRANULARITY;
            /* fill the Vsync values */
            mode.vsync_start = mode.vdisplay + CVT_MIN_V_PORCH;
            mode.vsync_end = mode.vsync_start + vsync;
        } else {
            /* Reduced blanking */
            /* Minimum vertical blanking interval time (µs)- default 460 */
            const CVT_RB_MIN_VBLANK: i32 = 460;
            /* Fixed number of clocks for horizontal sync */
            const CVT_RB_H_SYNC: i32 = 32;
            /* Fixed number of clocks for horizontal blanking */
            const CVT_RB_H_BLANK: i32 = 160;
            /* Fixed number of lines for vertical front porch - default 3*/
            const CVT_RB_VFPORCH: i32 = 3;
            let mut vbilines: i32;
            /* 8. Estimate Horizontal period. */
            tmp1 = HV_FACTOR * 1000000 - CVT_RB_MIN_VBLANK * HV_FACTOR * vfieldrate as i32;
            tmp2 = vdisplay_rnd + 2 * vmargin;
            hperiod = tmp1 as u32 / (tmp2 as u32 * vfieldrate);
            /* 9. Find number of lines in vertical blanking */
            vbilines = CVT_RB_MIN_VBLANK * HV_FACTOR / hperiod as i32 + 1;
            /* 10. Check if vertical blanking is sufficient */
            if vbilines < (CVT_RB_VFPORCH + vsync + CVT_MIN_V_BPORCH) {
                vbilines = CVT_RB_VFPORCH + vsync + CVT_MIN_V_BPORCH;
            }
            /* 11. Find total number of lines in vertical field */
            mode.vtotal = vdisplay_rnd + 2 * vmargin + vbilines;
            /* 12. Find total number of pixels in a line */
            mode.htotal = mode.hdisplay + CVT_RB_H_BLANK;
            /* Fill in HSync values */
            mode.hsync_end = mode.hdisplay + CVT_RB_H_BLANK / 2;
            mode.hsync_start = mode.hsync_end - CVT_RB_H_SYNC;
            /* Fill in VSync values */
            mode.vsync_start = mode.vdisplay + CVT_RB_VFPORCH;
            mode.vsync_end = mode.vsync_start + vsync;
        }
        /* 15/13. Find pixel clock frequency (kHz for xf86) */
        mode.clock = mode.htotal as u64; /* perform intermediate calcs in u64 */
        mode.clock *= HV_FACTOR as u64 * 1000;
        mode.clock /= hperiod as u64;
        mode.clock -= mode.clock % CVT_CLOCK_STEP as u64;

        /* 18/16. Find actual vertical frame frequency */
        /* ignore - just set the mode flag for interlaced */
        if interlaced {
            mode.vtotal *= 2;
            mode.flags |= CvtModeFlags::INTERLACE;
        }
        /* Fill the mode line name */
        if reduced {
            mode.flags |= CvtModeFlags::PHSYNC | CvtModeFlags::NVSYNC;
        } else {
            mode.flags |= CvtModeFlags::PVSYNC | CvtModeFlags::NHSYNC;
        }

        mode.vrefresh = vrefresh;

        mode
    }

    pub fn generate_edid_asm(&self, version: Version, timing_name: &str) -> String {
        let mut s = format!(
            "#define VERSION {major}
#define REVISION {minor}
#define CLOCK {clock}
#define XPIX {xpix}
#define XBLANK {xblank}
#define XOFFSET {xoffset}
#define XPULSE {xpulse}
#define YPIX {ypix}
#define YBLANK {yblank}
#define YOFFSET {yoffset}
#define YPULSE {ypulse}
#define VFREQ {vfreq}
#define TIMING_NAME \"{timing_name}\"
#define DPI 96
#define HSYNC_POL 1
#define VSYNC_POL 1\n",
            major = version.major(),
            minor = version.minor(),
            clock = self.clock,
            xpix = self.hdisplay,
            xblank = self.htotal - self.hdisplay,
            xoffset = self.hsync_start - self.hdisplay,
            xpulse = self.hsync_end - self.hsync_start,
            ypix = self.vdisplay,
            yblank = self.vtotal - self.vdisplay,
            yoffset = self.vsync_start - self.vdisplay,
            ypulse = self.vsync_end - self.vsync_start,
            vfreq = self.vrefresh,
            timing_name = timing_name,
        );

        if let XYRatio::Ratio16_10 | XYRatio::Ratio16_9 | XYRatio::Ratio4_3 | XYRatio::Ratio5_4 =
            self.xy_ratio
        {
            s.push_str(&format!("#define XY_RATIO {}\n", self.xy_ratio.to_string()));
        }

        s.push_str("#include \"edid.S.template\"");

        s
    }
}

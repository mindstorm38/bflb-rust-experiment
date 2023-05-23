# Tool used to parse registers .h difinition files from bouffalo lab official SDK.


def process(header_text: str, out_file: str, struct_name: str, prefix: str, doc: str):

    field_bstart = None

    # Fields list for the main structure being constructed.
    mmio_struct_fields = []
    if struct_name in STRUCT_FIELDS:
        mmio_struct_fields.extend(STRUCT_FIELDS[struct_name])

    # Registers to add after the structure definition and
    # fields of the register currently being built.
    mmio_regs = {}
    mmio_reg_fields = None

    for line_idx, line in enumerate(header_text.splitlines()):

        try:

            parts = list(filter(len, line.rstrip().split(" ")))
            if not len(parts):
                continue

            if parts[0] != "#define":
                continue

            if len(parts) > 2:
                parts[2] = parts[2].rstrip("/*")

            if parts[1].endswith("_OFFSET") and parts[1] != parts[2]:

                field_index = parse_macro_int(parts[2])

                field_name = parts[1][:-7]
                field_type = "".join(map(lambda s: f"{s[0].upper()}{s[1:].lower()}", field_name.split("_")))
                
                if field_type == struct_name:
                    field_type = f"{field_type}0"

                if field_name.startswith(prefix):
                    field_name = field_name[len(prefix):]
                field_name = field_name.lower()
                if field_name[0].isnumeric():
                    field_name = f"_{field_name}"

                mmio_struct_fields.append({
                    "index": field_index,
                    "name": field_name,
                    "type": field_type,
                })

                mmio_reg_fields = []
                mmio_regs[field_type] = mmio_reg_fields

                if field_type in REGISTER_FIELDS:
                    mmio_reg_fields.extend(REGISTER_FIELDS[field_type])

            elif mmio_reg_fields is not None:
                
                if parts[1].endswith("_POS"):
                    field_bstart = parse_macro_int(parts[2])
                elif parts[1].endswith("_LEN"):

                    if field_bstart is not None:

                        field_bend = field_bstart + parse_macro_int(parts[2])
                        field_name = parts[1][:-4]
                        if field_name.startswith(prefix):
                            field_name = field_name[len(prefix):]
                        if field_name.startswith("REG_"):
                            field_name = field_name[4:]
                        elif field_name.startswith("REG2_"):
                            field_name = field_name[5:]
                        field_name = field_name.lower()

                        field_data = {
                            "start": field_bstart,
                            "end": field_bend,
                            "name": field_name,
                        }

                        if field_name in REGISTER_DOCS:
                            field_data["doc"] = REGISTER_DOCS[field_name]

                        mmio_reg_fields.append(field_data)

                        field_bstart = None
        
        except:
            print(f"error at line {line_idx + 1}:")
            raise

    mmio_struct_fields.sort(key=lambda field: field["index"])

    with open(out_file, "wt", newline="\n") as out_fp:

        out_fp.write(f"//! {doc}\n\n")
        
        out_fp.write("embedded_util::mmio! {\n")
        out_fp.write(f"    pub struct {struct_name} {{\n")
        for field in mmio_struct_fields:
            if "doc" in field:
                out_fp.write(f"        /// {field['doc']}\n")
            out_fp.write(f"        [0x{field['index']:03X}] rw {field['name']}: {field['type']},\n")
        out_fp.write("    }\n}\n")

        for reg_name, reg_fields in mmio_regs.items():
            out_fp.write("\nembedded_util::reg! {\n")
            out_fp.write(f"    pub struct {reg_name}: u32 {{\n")
            for reg_field in reg_fields:
                if "doc" in reg_field:
                    out_fp.write(f"        /// {reg_field['doc']}\n")
                out_fp.write(f"        [{reg_field['start']:02}..{reg_field['end']:02}] {reg_field['name']},\n")
            out_fp.write("    }\n}\n")


def parse_macro_int(val: str) -> int:
    val = val.strip("()U")
    if val.startswith("0x"):
        return int(val[2:], base=16)
    else:
        return int(val)


def process_auto(out_dir: str):

    from os import path
    import requests
    
    for struct_id, struct_info in STRUCTS.items():

        print(f"Downloading and processing {struct_id}... ", end="", flush=True)

        header_res = requests.get(struct_info["header"])
        if header_res.status_code != 200:
            print("error")
            continue

        out_file = path.join(out_dir, f"{struct_id}.rs")

        process(header_res.text, out_file, struct_info["name"], struct_info["prefix"], struct_info["doc"])

        print("done")


BASE_BL808_URL = "https://raw.githubusercontent.com/bouffalolab/bouffalo_sdk/master/drivers/soc/bl808/std/include/hardware/"
BASE_LHAL_URL = "https://raw.githubusercontent.com/bouffalolab/bouffalo_sdk/master/drivers/lhal/include/hardware/"

STRUCTS = {
    "mcu_misc": {
        "header": f"{BASE_BL808_URL}mcu_misc_reg.h",
        "name": "McuMisc",
        "prefix": "MCU_MISC_",
        "doc": "MCU E907 register."
    },
    "mm_misc": {
        "header": f"{BASE_BL808_URL}mm_misc_reg.h",
        "name": "MmMisc",
        "prefix": "MM_MISC_",
        "doc": "MM C906 register."
    },
    "mm_glb": {
        "header": f"{BASE_BL808_URL}mm_glb_reg.h",
        "name": "MmGlb",
        "prefix": "MM_GLB_",
        "doc": "Multimedia global register."
    },
    "hbn": {
        "header": f"{BASE_BL808_URL}hbn_reg.h",
        "name": "Hbn",
        "prefix": "HBN_",
        "doc": "Hibernate register."
    },
    "glb": {
        "header": f"{BASE_BL808_URL}glb_reg.h",
        "name": "Glb",
        "prefix": "GLB_",
        "doc": "Global register, used for clock management."
    },
    "pds": {
        "header": f"{BASE_BL808_URL}pds_reg.h",
        "name": "Pds",
        "prefix": "PDS_",
        "doc": "Power Down Sleep register."
    },
    "cci": {
        "header": f"{BASE_BL808_URL}cci_reg.h",
        "name": "Cci",
        "prefix": "CCI_",
        "doc": ""
    },
    "sf_ctrl": {
        "header": f"{BASE_BL808_URL}sf_ctrl_reg.h",
        "name": "SfCtrl",
        "prefix": "SF_CTRL_",
        "doc": "Serial Flash."
    },
    "aon": {
        "header": f"{BASE_BL808_URL}aon_reg.h",
        "name": "Aon",
        "prefix": "AON_",
        "doc": "Always On register."
    },
    "dtsrc": {
        "header": "https://raw.githubusercontent.com/sipeed/M1s_BL808_SDK/master/components/platform/soc/bl808/bl808_std/BL808_BSP_Driver/dsp2_reg/dtsrc_reg.h",
        "name": "Dtsrc",
        "prefix": "DTSRC_",
        "doc": "DVP TSRC."
    },
    "dsp2_misc": {
        "header": "https://raw.githubusercontent.com/sipeed/M1s_BL808_SDK/master/components/stage/dsp2/dsp2_drv/dsp2_reg/dsp2_misc_reg.h",
        "name": "Dsp2Misc",
        "prefix": "DSP2_MISC_",
        "doc": "DSP2 misc."
    }
    # "vdo": {
    #     "header": "https://raw.githubusercontent.com/sipeed/M1s_BL808_SDK/master/components/stage/bl_mm/mm_drv/mm_reg/vdo_reg.h",
    #     "name": "Vdo",
    #     "prefix": "VDO_",
    #     "doc": "Video/H264 registers."
    # },
    # "csi": {
    #     "header": "https://raw.githubusercontent.com/sipeed/M1s_BL808_SDK/master/components/platform/soc/bl808/bl808_std/BL808_BSP_Driver/dsp2_reg/csi_reg.h",
    #     "name": "Csi",
    #     "prefix": "CSI_",
    #     "doc": "MIPI CSI registers."
    # }
}

STRUCT_FIELDS = {
    "Pds": [
        {
            "index": 0x130,
            "name": "cpu_mtimer_rtc",
            "type": "super::CpuRtc",
            "doc": "Alias for `cpu_core_cfg8`."
        },
    ],
    "MmMisc": [
        {
            "index": 0x018,
            "name": "cpu_mtimer_rtc",
            "type": "super::CpuRtc",
            "doc": "Alias for `cpu_rtc`."
        },
    ],
    "McuMisc": [
        {
            "index": 0x014,
            "name": "cpu_mtimer_rtc",
            "type": "super::CpuRtc",
            "doc": "Alias for `mcu_e907_rtc`."
        },
    ],
    "Glb": [
        {
            "index": 0x810,
            "name": "wifi_pll_cfg0_",
            "type": "super::PllCfg0",
            "doc": "Alias for `wifi_pll_cfg0`."
        },
        {
            "index": 0x814,
            "name": "wifi_pll_cfg1_",
            "type": "super::PllCfg1",
            "doc": "Alias for `wifi_pll_cfg1`."
        },
        {
            "index": 0x790,
            "name": "mipi_pll_cfg0_",
            "type": "super::PllCfg0",
            "doc": "Alias for `mipi_pll_cfg0`."
        },
        {
            "index": 0x794,
            "name": "mipi_pll_cfg1_",
            "type": "super::PllCfg1",
            "doc": "Alias for `mipi_pll_cfg1`."
        },
        {
            "index": 0x7D0,
            "name": "uhs_pll_cfg0_",
            "type": "super::PllCfg0",
            "doc": "Alias for `uhs_pll_cfg0`."
        },
        {
            "index": 0x7D4,
            "name": "uhs_pll_cfg1_",
            "type": "super::PllCfg1",
            "doc": "Alias for `uhs_pll_cfg1`."
        },
    ],
    "Cci": [
        {
            "index": 0x750,
            "name": "audio_pll_cfg0_",
            "type": "super::PllCfg0",
            "doc": "Alias for `audio_pll_cfg0`."
        },
        {
            "index": 0x754,
            "name": "audio_pll_cfg1_",
            "type": "super::PllCfg1",
            "doc": "Alias for `audio_pll_cfg1`."
        },
        {
            "index": 0x7D0,
            "name": "cpu_pll_cfg0_",
            "type": "super::PllCfg0",
            "doc": "Alias for `cpu_pll_cfg0`."
        },
        {
            "index": 0x7D4,
            "name": "cpu_pll_cfg1_",
            "type": "super::PllCfg1",
            "doc": "Alias for `cpu_pll_cfg1`."
        },
    ]
}

REGISTER_FIELDS = {
    "HbnGlb": [
        {
            "start": 0,
            "end": 1,
            "name": "xclk_sel",
            "doc": "Alias for `root_clk_sel & 1`."
        },
        {
            "start": 1,
            "end": 2,
            "name": "mcu_root_sel",
            "doc": "Alias for `(root_clk_sel >> 1) & 1`."
        },
    ],
    "HbnRsv3": [
        {
            "start": 0,
            "end": 8,
            "name": "xtal_type",
            "doc": "Alias for `rsv3 & 0xFF`."
        },
        {
            "start": 8,
            "end": 16,
            "name": "xtal_flag",
            "doc": "Alias for `(rsv3 >> 8) & 0xFF`."
        },
    ],
    "GlbHwRsv1": [
        {
            "start": 0,
            "end": 31,
            "name": "flash_id",
            "doc": "Numeric identifier of the flash."
        },
        {
            "start": 31,
            "end": 32,
            "name": "flash_id_valid",
            "doc": "Indicate if the stored flash identifier is valid."
        }
    ]
}

REGISTER_DOCS = {
    "dl0_enable": "Enable lane number 0.",
    "dl1_enable": "Enable lane number 1.",
    "cl_enable": "Enable clock lane.",
    "time_ck_settle": "Time interval during which the HS receiver shall ignore any clock lane HS transitions.",
    "time_ck_term_en": "Time for the clock lane receiver to enable the HS line termination.",
    "time_hs_settle": "Time interval during which the HS receiver shall ignore any data lane HS transitions.",
}


if __name__ == "__main__":
    
    import sys

    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <out_dir>")
    else:
        process_auto(sys.argv[1])

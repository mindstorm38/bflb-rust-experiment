# This tool is used to generate assembly sources and linker scripts,
# it avoid most errors by automatically generating context save 
# instructions for the given architecture.


CHIPS = {
    "bl808_m0": {
        "arch": {
            "name": "riscv",
            "id": "rv32imafc",
        },
        "max_cores": 1,
    },
    "bl808_d0": {
        "arch": {
            "name": "riscv",
            "id": "rv64imafcv",
        },
        "max_cores": 1,
    }
}

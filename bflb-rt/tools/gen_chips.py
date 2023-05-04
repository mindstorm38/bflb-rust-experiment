# This tool is used to generate assembly sources and linker scripts,
# it avoid most errors by automatically generating context save 
# instructions for the given architecture.


CHIPS = {
    "bl808_m0": {
        "arch": "rv32imafc",
        "max_cores": 1,
    },
    "bl808_d0": {
        "arch": "rv64imafcv",
        "max_cores": 1,
    }
}


def main(src_dir):
    pass


if __name__ == "__main__":  

    import sys

    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <src_dir>")
    else:
        main(sys.argv[1])

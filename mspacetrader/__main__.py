import argparse

if __name__ == "__main__":
    argparser = argparse.ArgumentParser("Midnight Space Trader")
    argparser.add_argument(
        "-d",
        "--data",
        default="solar-en.toml",
        help="The TOML file to read game data from",
    )
    args = argparse.parse_args()
    # TODO

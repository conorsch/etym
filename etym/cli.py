#!/usr/bin/env python
import argparse
from blessings import Terminal
from textwrap import fill
import sys

import etym.utils
import etym
from etym import __version__


def display_in_terminal(hit: str, etymology: str) -> None:
    """
    Print results to stdout, with fancy terminal colors.
    """
    t = Terminal()
    print(t.bold(hit))
    print(fill(etymology, width=t.width))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("word", action="store", type=str)
    parser.add_argument("--verbose", action="store_true", default=False)
    parser.add_argument(
        "--format", action="store", default="auto", choices=("auto", "plain", "terminal"),
    )
    parser.add_argument("--version", action="store_true", default=False)
    args = parser.parse_args()
    if args.version:
        print("etym v{}".format(__version__))
    else:
        query = args.word
        result = etym.utils.query_etym_online(query, verbose=args.verbose)
        if sys.stdout.isatty():
            t = Terminal()
            print(t.bold(query))
            print(result)
        else:
            print(query)
            print(result)


if __name__ == "__main__":
    main()

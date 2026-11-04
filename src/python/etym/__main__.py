import argparse
import sys
import traceback
from etym import Etymology, cli_lookup

def main():
    parser = argparse.ArgumentParser(
        prog="etym-py",
        description="Query word etymologies from EtymOnline.com via Rust core.",
    )
    
    # Positionals & Flags
    parser.add_argument(
        "word", 
        type=str, 
        help="Word for which etymology is desired"
    )
    parser.add_argument(
        "-d", "--debug", 
        action="count", 
        default=0, 
        help="Turn debugging information on (use -dd for extra verbosity)"
    )

    args = parser.parse_args()

    if args.debug > 0:
        print(f"[DEBUG] Querying etymology for word: '{args.word}'", file=sys.stderr)

    try:
        # Calls our PyO3 Rust function which prints with terminal wrapping
        cli_lookup(args.word)
    except Exception as err:
        if args.debug > 0:
            print("[DEBUG] Exception details:", file=sys.stderr)
            traceback.print_exc()
        else:
            print(f"Error: {err}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()

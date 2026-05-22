import argparse

def parse_args():
    parser = argparse.ArgumentParser(description="Spider - A simple web crawler.")
    parser.add_argument("url", help="The URL to start crawling from")
    parser.add_argument("-r", action="store_true", help="Enable recursive crawling")
    parser.add_argument("-l", type=int, default=5, help="Set the maximum depth level for crawling")
    parser.add_argument("-p", default="./data/", help="Set the output directory for crawled data")
    return parser.parse_args()

args = parse_args()
print(f"URL: {args.url}")
print(f"Recursive: {args.r}")
print(f"Max Depth Level: {args.l}")
print(f"Output Directory: {args.p}")

import argparse
import requests

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Spider - A simple web crawler.")
    parser.add_argument("url", help="The URL to start crawling from")
    parser.add_argument("-r", action="store_true", help="Enable recursive crawling")
    parser.add_argument("-l", type=int, default=5, help="Set the maximum depth level for crawling")
    parser.add_argument("-p", default="./data/", help="Set the output directory for crawled data")
    return parser.parse_args()

def fetch_page(url: str) -> str: 
    try:
        response = requests.get(url)
        response.raise_for_status()
        return(response.text)
    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP error occurred: {http_err}")
    except requests.exceptions.ConnectionError as conn_err:
        print(f"Connection error occurred: {conn_err}")
    except requests.exceptions.Timeout as timeout_err:
        print(f"Timeout error occurred: {timeout_err}")
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")
    return ("")


def main():
    # parse command line arguments
    args = parse_args()
    print(f"URL: {args.url}")
    print(f"Recursive: {args.r}")
    print(f"Max Depth Level: {args.l}")
    print(f"Output Directory: {args.p}")
    # url page content
    page_content = fetch_page(args.url)
    print(f"Fetched page content length: {len(page_content)}")

if __name__ == "__main__":
    main()
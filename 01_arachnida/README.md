# Intro to web scraping and metadata
## Two programs
### Requirements
- `spider`: a small crawler that starts from the URL, finds image links, optionally follows page links recursively, respects a depth limit, and saves image
- `scorpion`: a metadata reader for image files, esp. EXIF/basic attributes

### Rules
- Use HTTP/file/parsing lib if needed
- Write the decision logic, eg. what URLs to visit, what images to download, how recursion works, how paths are resolved, how duplicates/errors are handled

## Implementation
### Language choice: Python
- argparse: cmd line options
- requests/ urllib: http
- html.parser, BeautifulSoup: parse html
- urllib.parse.urljoin: resolve relative urls
- pathlib/ os: save files


## Notes
### What's Metadata

### What's EXIF

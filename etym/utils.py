from .exceptions import NoResultsFound
from bs4 import BeautifulSoup
from blessings import Terminal
import re
import requests


def query_etym_online(query: str, verbose: bool = False) -> str:
    """Perform lookup on etymonline.com."""
    r = requests.get("https://www.etymonline.com/index.php?search={}".format(query))
    soup = BeautifulSoup(r.content, features="html.parser")
    try:
        hit = soup.find_all("p")[0].contents
    except IndexError:
        raise NoResultsFound(query)
    s = beautify(hit)
    return s


def beautify(soup: BeautifulSoup, rich_terminal: bool = True) -> str:
    """
    Cleans up the raw HTML so it's more presentable.
    Parse BeautifulSoup HTML and return prettified string
    """
    beautifiedText = str()
    for i in soup:
        if rich_terminal:
            term = Terminal()
            span_sub = r"{t.italic}\1{t.normal}".format(t=term)
            strong_sub = r"{t.bold}\1{t.normal}".format(t=term)
        else:
            span_sub = r"\1"
            strong_sub = r"\1"
        i = re.sub(r'<span class="\w+">(.+)</span>', span_sub, str(i),)
        i = re.sub(r"<strong>(.+)</strong>", strong_sub, str(i))
        beautifiedText += " " + i

    # Remove leading whitespace.
    beautifiedText = re.sub(r"^\s+", "", beautifiedText)
    # Compress all whitespace to a single space.
    beautifiedText = re.sub(r"\s{2,}", " ", beautifiedText)
    # Trim whitespace immediately preceding common punctuation.
    beautifiedText = re.sub(r"\s+([,\)\].;:])", r"\g<1>", beautifiedText)
    # Trim whitespace immediately following common punctuation.
    beautifiedText = re.sub(r"([\(])\s+", r"\g<1>", beautifiedText)
    return beautifiedText

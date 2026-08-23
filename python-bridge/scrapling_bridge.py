"""
Oxide-Tech Local Agent: Scrapling Stealth Web Perception Worker
Tier 1: Fast local DOM, documentation, and API extraction with anti-fingerprinting.
"""

import logging
import re
from typing import Dict, List, Optional
from dataclasses import dataclass

logger = logging.getLogger("scrapling_bridge")

try:
    from scrapling import Fetcher, StealthyFetcher
    SCRAPLING_AVAILABLE = True
except ImportError:
    SCRAPLING_AVAILABLE = False
    logger.warning("scrapling library not installed; falling back to httpx/bs4")

try:
    import httpx
    from bs4 import BeautifulSoup
except ImportError:
    BeautifulSoup = None
    httpx = None


@dataclass
class ScrapeResult:
    url: str
    success: bool
    status_code: int
    title: str
    markdown_content: str
    links: List[str]
    metadata: Dict[str, str]
    error: Optional[str] = None


class ScraplingBridge:
    def __init__(self, stealth_mode: bool = True, timeout: int = 15):
        self.stealth_mode = stealth_mode
        self.timeout = timeout
        self.default_headers = {
            "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
            "Accept-Language": "en-US,en;q=0.9",
        }

    def fetch_url(self, url: str, selector: Optional[str] = None, css_clean: bool = True) -> ScrapeResult:
        """Fetch and parse a webpage using Scrapling (or fallback) with stealth headers."""
        if SCRAPLING_AVAILABLE:
            return self._fetch_scrapling(url, selector, css_clean)
        else:
            return self._fetch_fallback(url, selector, css_clean)

    def _fetch_scrapling(self, url: str, selector: Optional[str] = None, css_clean: bool = True) -> ScrapeResult:
        try:
            if self.stealth_mode:
                fetcher = StealthyFetcher()
            else:
                fetcher = Fetcher()

            response = fetcher.get(url, timeout=self.timeout)
            
            status_code = getattr(response, "status_code", 200)
            if status_code >= 400:
                return ScrapeResult(
                    url=url,
                    success=False,
                    status_code=status_code,
                    title="",
                    markdown_content="",
                    links=[],
                    metadata={"engine": "scrapling", "stealth": str(self.stealth_mode)},
                    error=f"HTTP {status_code} received from {url}",
                )

            # Extract title and body
            title = ""
            try:
                title_elem = response.css("title")
                if title_elem:
                    title = title_elem[0].text.strip()
            except Exception:
                pass

            links = []
            try:
                for a in response.css("a[href]"):
                    href = a.attrib.get("href", "")
                    if href.startswith("http") or href.startswith("/"):
                        links.append(href)
            except Exception:
                pass

            # Extract main content / selector
            raw_html = ""
            if selector:
                matched = response.css(selector)
                if matched:
                    raw_html = "".join([m.get_all_text() if hasattr(m, "get_all_text") else str(m) for m in matched])
            
            if not raw_html:
                raw_html = response.text if hasattr(response, "text") else str(response)

            markdown = self._html_to_clean_markdown(raw_html, title=title)

            return ScrapeResult(
                url=url,
                success=True,
                status_code=status_code,
                title=title,
                markdown_content=markdown,
                links=links[:50],
                metadata={
                    "engine": "scrapling",
                    "stealth": str(self.stealth_mode),
                    "bytes_received": str(len(raw_html)),
                },
            )
        except Exception as e:
            logger.error(f"Scrapling fetch error for {url}: {e}")
            return self._fetch_fallback(url, selector, css_clean, error_prefix=f"Scrapling failed ({e}); fallback: ")

    def _fetch_fallback(self, url: str, selector: Optional[str] = None, css_clean: bool = True, error_prefix: str = "") -> ScrapeResult:
        if not httpx or not BeautifulSoup:
            return ScrapeResult(
                url=url,
                success=False,
                status_code=500,
                title="",
                markdown_content="",
                links=[],
                metadata={"engine": "none"},
                error=f"{error_prefix}Neither scrapling nor httpx/bs4 available",
            )

        try:
            with httpx.Client(timeout=self.timeout, headers=self.default_headers, follow_redirects=True) as client:
                resp = client.get(url)
                if resp.status_code >= 400:
                    return ScrapeResult(
                        url=url,
                        success=False,
                        status_code=resp.status_code,
                        title="",
                        markdown_content="",
                        links=[],
                        metadata={"engine": "httpx_fallback"},
                        error=f"{error_prefix}HTTP {resp.status_code}",
                    )

                soup = BeautifulSoup(resp.text, "html.parser")
                title = soup.title.string.strip() if soup.title and soup.title.string else ""

                # Remove noise elements
                for noise in soup(["script", "style", "nav", "footer", "aside", "noscript", "svg"]):
                    noise.decompose()

                links = []
                for a in soup.find_all("a", href=True):
                    href = a["href"]
                    if href.startswith("http") or href.startswith("/"):
                        links.append(href)

                target = soup.select(selector) if selector else [soup.body or soup]
                text_blocks = []
                for elem in target:
                    text_blocks.append(elem.get_text(separator="\n", strip=True))

                raw_text = "\n\n".join(text_blocks)
                markdown = self._format_as_markdown(raw_text, title, url)

                return ScrapeResult(
                    url=url,
                    success=True,
                    status_code=resp.status_code,
                    title=title,
                    markdown_content=markdown,
                    links=links[:50],
                    metadata={"engine": "httpx_bs4_fallback"},
                )
        except Exception as e:
            return ScrapeResult(
                url=url,
                success=False,
                status_code=500,
                title="",
                markdown_content="",
                links=[],
                metadata={"engine": "failed"},
                error=f"{error_prefix}{str(e)}",
            )

    def _html_to_clean_markdown(self, html: str, title: str = "") -> str:
        clean = re.sub(r"<(script|style|svg|noscript)[^>]*>.*?</\1>", "", html, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<h1[^>]*>(.*?)</h1>", r"# \1\n", clean, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<h2[^>]*>(.*?)</h2>", r"## \1\n", clean, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<h3[^>]*>(.*?)</h3>", r"### \1\n", clean, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<code[^>]*>(.*?)</code>", r"`\1`", clean, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<pre[^>]*>(.*?)</pre>", r"```\n\1\n```\n", clean, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<li[^>]*>(.*?)</li>", r"- \1\n", clean, flags=re.DOTALL | re.IGNORECASE)
        clean = re.sub(r"<[^>]+>", "", clean)
        clean = re.sub(r"\n{3,}", "\n\n", clean).strip()

        if title and not clean.startswith(f"# {title}"):
            clean = f"# {title}\n\n{clean}"
        return clean

    def _format_as_markdown(self, text: str, title: str, url: str) -> str:
        header = f"# {title}\n\n> Source: {url}\n\n" if title else f"> Source: {url}\n\n"
        cleaned_text = re.sub(r"\n{3,}", "\n\n", text).strip()
        return header + cleaned_text


if __name__ == "__main__":
    worker = ScraplingBridge()
    res = worker.fetch_url("https://docs.rs/embedded-hal/latest/embedded_hal/")
    print(f"Fetch success: {res.success}, Status: {res.status_code}, Title: {res.title}")
    print(f"Markdown preview:\n{res.markdown_content[:250]}...")

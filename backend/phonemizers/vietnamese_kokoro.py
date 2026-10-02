"""
Independent Vietnamese phonemizer adapter for Kokoro Vietnamese.

Converts UTF-8 Vietnamese text into phoneme tokens matching Kokoro's
fixed vocabulary inventory. Built directly on top of sea-g2p (Apache-2.0)
and the Kokoro-Vietnamese phonetic specifications (Apache-2.0).

Key properties:
- Preserves Vietnamese onset contrasts:
    * t vs th (tường [t] vs thường [θ], teo [t] vs theo [θ])
    * tr vs ch (trước [ʈʂ] vs chước [ʧ])
    * s vs x   (sinh [ʂ] vs xinh [s], số [ʂ] vs xố [s])
    * gi vs d  (giải [ʝ] vs dải [z], gì [ʝ] vs dì [z])
- English onset cluster guard: start, style, school retain standard onset [s].
- Vowel contrast preservation: e- mapped to [æ] (cách [æ] vs kếch [e]).
- Explicit lexical tone representation via Kokoro arrow symbols:
    * Level/ngang (1/7): →
    * Low-falling/huyền (2): ↘
    * High-rising/sắc (3/ɜ): ↗
    * Dipping/hỏi (4): ↓
    * Glottalized-rising/ngã (5): ʔ↗
    * Glottalized-falling/nặng (6): ʔ↓
- Consonant standardizations: tʃ -> ʧ, t̪ -> t, ɗ -> d, ʐ -> ʒ, đ -> d.
- Punctuation and whitespace preservation with symbol cleanup.
- Thread-safe and deterministic.

License: Apache-2.0
"""

from __future__ import annotations

import logging
import re
import threading
from typing import Optional

logger = logging.getLogger(__name__)

# Non-Vietnamese 's' consonant clusters that should NOT be rewritten to retroflex ʂ
NON_VI_S_CLUSTERS: tuple[str, ...] = (
    "sc",
    "sh",
    "sk",
    "sl",
    "sm",
    "sn",
    "sp",
    "st",
    "sw",
)

# Tokenizer pattern: words (including accented characters and internal apostrophes/hyphens), whitespace, or other single characters
TOKEN_RE = re.compile(r"[A-Za-zÀ-ỹĐđ]+(?:[-'][A-Za-zÀ-ỹĐđ]+)*|\s+|.", re.UNICODE)
WORD_RE = re.compile(r"^[A-Za-zÀ-ỹĐđ]+(?:[-'][A-Za-zÀ-ỹĐđ]+)*$", re.UNICODE)
GI_VOWEL_RE = re.compile(r"^g[iìíỉĩị]", re.UNICODE)

# Dental t temporary placeholder to prevent conflict during th-onset rewrite
_DENTAL_T_SENTINEL = "\ue100"


class VietnameseKokoroPhonemizer:
    """
    Independent Vietnamese phonemizer adapter for Kokoro Vietnamese.

    Wraps sea_g2p.G2P('vi') and applies contrast-preserving Kokoro
    vocabulary adaptation.
    """

    def __init__(self):
        self._g2p = None
        self._lock = threading.Lock()

    @property
    def g2p(self):
        """Lazy-initialize sea_g2p instance in a thread-safe manner."""
        if self._g2p is None:
            with self._lock:
                if self._g2p is None:
                    import sea_g2p

                    self._g2p = sea_g2p.G2P("vi")
        return self._g2p

    def phonemize_word(self, word: str) -> str:
        """
        Convert a single word to Kokoro-compatible phonemes, applying onset contrast rules.
        """
        raw = self.g2p.convert(word)
        w_lower = word.lower()

        # Temporarily shelter dental t (t̪) so the th-onset rewrite does not touch it
        raw = raw.replace("t̪", _DENTAL_T_SENTINEL)

        # 1. Onset contrast: 'th' vs 't'
        # sea_g2p assigns 't' to 'th-' words and 't̪' to 't-' words.
        # Rewrite raw onset 't' to 'θ' for words beginning with 'th'.
        if w_lower.startswith("th"):
            raw = re.sub(r"([ˈˌ]?)t(?![ʃ])", r"\1θ", raw, count=1)

        # 2. Onset contrast: 'tr' vs 'ch'
        # sea_g2p assigns 'tʃ' to both 'tr-' and 'ch-'.
        # Rewrite 'tʃ' to 'ʈʂ' for words beginning with 'tr'.
        elif w_lower.startswith("tr"):
            raw = re.sub(r"([ˈˌ]?)tʃ", r"\1ʈʂ", raw, count=1)

        # 3. Onset contrast: 's' vs 'x'
        # sea_g2p assigns 's' to both 's-' and 'x-'.
        # Rewrite 's' to 'ʂ' for Vietnamese words beginning with 's',
        # while preserving standard 's' for English onset clusters like 'st-', 'sp-', etc.
        elif w_lower.startswith("s"):
            if not any(w_lower.startswith(cluster) for cluster in NON_VI_S_CLUSTERS):
                raw = re.sub(r"([ˈˌ]?)s", r"\1ʂ", raw, count=1)

        # 4. Onset contrast: 'gi' vs 'd'
        # sea_g2p assigns 'z' to both 'gi-' and 'd-'.
        # Rewrite 'z' to 'ʝ' for words beginning with 'gi' or accented 'gì', 'gí', etc.
        elif w_lower.startswith("gi") or GI_VOWEL_RE.match(w_lower):
            raw = re.sub(r"([ˈˌ]?)z", r"\1ʝ", raw, count=1)

        return raw

    @staticmethod
    def normalize_units(text: str) -> str:
        """
        Normalize Vietnamese unit abbreviations (e.g. km, km/h) before phonemization.

        Ensures unit tokens are pronounced naturally in Vietnamese ('ki lô mét')
        rather than spelled out as English letters ('K-M').
        Preserves non-unit words containing 'km' as a substring (e.g. 'kmart', 'bookmark').
        """
        # 1. km/h unit (e.g. "60 km/h", "60km/h")
        text = re.sub(r"(?<=\d)\s*(?:km/h|KM/H|Km/h)\b", " ki lô mét trên giờ", text)
        text = re.sub(r"\b(?:km/h|KM/H|Km/h)\b", "ki lô mét trên giờ", text)

        # 2. km unit (e.g. "10 km", "3.260 km", "5 KM", "12km")
        text = re.sub(r"(?<=\d)\s*(?:km|KM|Km)\b", " ki lô mét", text)
        text = re.sub(r"\b(?:km|KM|Km)\b", "ki lô mét", text)

        return text

    def phonemize(self, text: str) -> str:
        """
        Convert full Vietnamese text into Kokoro-compatible phoneme string.

        Args:
            text: UTF-8 Vietnamese text.

        Returns:
            Phoneme string formatted with Kokoro vocabulary tokens.
        """
        if not text:
            return ""

        # Normalize curly apostrophes to straight single quotes before tokenization
        normalized = text.replace("’", "'").replace("‘", "'")

        # Normalize Vietnamese unit abbreviations (km, km/h) before tokenization
        normalized = self.normalize_units(normalized)

        tokens = TOKEN_RE.findall(normalized)
        output_chunks = []
        for token in tokens:
            if WORD_RE.match(token):
                output_chunks.append(self.phonemize_word(token))
            else:
                output_chunks.append(token)

        res = "".join(output_chunks)

        # Apply Kokoro vocabulary transformations and symbol standardizations
        res = res.replace("tʃ", "ʧ")
        res = res.replace(_DENTAL_T_SENTINEL, "t")
        res = res.replace("e-", "æ")

        # Lexical tone mappings to Kokoro tone arrow inventory
        res = res.replace("1", "→").replace("7", "→")
        res = res.replace("2", "↘")
        res = res.replace("3", "↗").replace("ɜ", "↗")
        res = res.replace("4", "↓")
        res = res.replace("5", "ʔ↗")
        res = res.replace("6", "ʔ↓")

        # Consonant standardizations
        res = res.replace("ɗ", "d")
        res = res.replace("ʐ", "ʒ")
        res = res.replace("đ", "d")

        # Punctuation & delimiter cleanup
        res = res.replace("–", "—")
        res = res.replace("*", "")
        res = res.replace("/", " ")
        res = res.replace("&", " ")

        # Strip leftover diacritics and delimiters
        for char in ("̪", "̩", "-", "'", "’", "‘"):
            res = res.replace(char, "")

        return res

    def __call__(self, text: str) -> str:
        return self.phonemize(text)


# Global singleton instance
_GLOBAL_PHONEMIZER: Optional[VietnameseKokoroPhonemizer] = None
_GLOBAL_LOCK = threading.Lock()


def get_vietnamese_phonemizer() -> VietnameseKokoroPhonemizer:
    """Get or create the global VietnameseKokoroPhonemizer instance."""
    global _GLOBAL_PHONEMIZER
    if _GLOBAL_PHONEMIZER is None:
        with _GLOBAL_LOCK:
            if _GLOBAL_PHONEMIZER is None:
                _GLOBAL_PHONEMIZER = VietnameseKokoroPhonemizer()
    return _GLOBAL_PHONEMIZER


def phonemize_vietnamese(text: str) -> str:
    """
    Main entrypoint: Convert Vietnamese text into Kokoro-compatible phoneme string.
    """
    return get_vietnamese_phonemizer().phonemize(text)


# Compatibility aliases
phonemize_text = phonemize_vietnamese
phonemize = phonemize_vietnamese

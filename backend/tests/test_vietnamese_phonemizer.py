"""
Tests for independent Vietnamese phonemizer adapter (backend.phonemizers.vietnamese_kokoro).

Verifies phonetic accuracy, contrast preservation, lexical tone mapping,
and punctuation handling based on the Apache-2.0 Kokoro-Vietnamese specification.
"""

import unittest
from backend.phonemizers.vietnamese_kokoro import (
    VietnameseKokoroPhonemizer,
    get_vietnamese_phonemizer,
    phonemize_vietnamese,
)


class TestVietnameseKokoroPhonemizer(unittest.TestCase):
    """Test suite for independent Vietnamese phonemizer adapter."""

    def setUp(self):
        self.phonemizer = get_vietnamese_phonemizer()

    def test_basic_greetings(self):
        """Test basic common greetings."""
        res_chao = self.phonemizer.phonemize("xin chào")
        self.assertIn("sˈin", res_chao)
        self.assertIn("ʧˈaː↘w", res_chao)

        res_vn = self.phonemizer.phonemize("Việt Nam")
        self.assertIn("vˈiɛʔ↓t", res_vn)
        self.assertIn("nˈaːm", res_vn)

    def test_onset_contrast_t_vs_th(self):
        """Preserves onset distinction between orthographic 't' and 'th'."""
        res_tuong = self.phonemizer.phonemize("tường")
        res_thuong = self.phonemizer.phonemize("thường")
        self.assertTrue(res_tuong.startswith("t"))
        self.assertTrue(res_thuong.startswith("θ"))

        # Unaccented test
        res_teo = self.phonemizer.phonemize("teo")
        res_theo = self.phonemizer.phonemize("theo")
        self.assertTrue(res_teo.startswith("t"))
        self.assertTrue(res_theo.startswith("θ"))

    def test_onset_contrast_tr_vs_ch(self):
        """Preserves onset distinction between orthographic 'tr' [ʈʂ] and 'ch' [ʧ]."""
        res_truoc = self.phonemizer.phonemize("trước")
        res_chuoc = self.phonemizer.phonemize("chước")
        self.assertTrue(res_truoc.startswith("ʈʂ"))
        self.assertTrue(res_chuoc.startswith("ʧ"))

        res_trung = self.phonemizer.phonemize("trúng")
        res_chung = self.phonemizer.phonemize("chúng")
        self.assertTrue(res_trung.startswith("ʈʂ"))
        self.assertTrue(res_chung.startswith("ʧ"))

    def test_onset_contrast_s_vs_x(self):
        """Preserves onset distinction between orthographic 's' [ʂ] and 'x' [s]."""
        res_sinh = self.phonemizer.phonemize("sinh")
        res_xinh = self.phonemizer.phonemize("xinh")
        self.assertTrue(res_sinh.startswith("ʂ"))
        self.assertTrue(res_xinh.startswith("s"))

        res_so = self.phonemizer.phonemize("số")
        res_xo = self.phonemizer.phonemize("xố")
        self.assertTrue(res_so.startswith("ʂ"))
        self.assertTrue(res_xo.startswith("s"))

    def test_english_s_cluster_guard(self):
        """English words with 's' clusters like 'start', 'style' retain standard 's'."""
        res_start = self.phonemizer.phonemize("start")
        res_style = self.phonemizer.phonemize("style")
        self.assertTrue(res_start.startswith("st"))
        self.assertTrue(res_style.startswith("st"))
        self.assertNotIn("ʂ", res_start)
        self.assertNotIn("ʂ", res_style)

    def test_onset_contrast_gi_vs_d(self):
        """Preserves onset distinction between orthographic 'gi' [ʝ] and 'd' [z]."""
        res_giai = self.phonemizer.phonemize("giải")
        res_dai = self.phonemizer.phonemize("dải")
        self.assertTrue(res_giai.startswith("ʝ"))
        self.assertTrue(res_dai.startswith("z"))

        res_gi = self.phonemizer.phonemize("gì")
        res_di = self.phonemizer.phonemize("dì")
        self.assertTrue(res_gi.startswith("ʝ"))
        self.assertTrue(res_di.startswith("z"))

    def test_vowel_contrast_e_vs_ae(self):
        """Preserves vowel contrast via e- -> æ."""
        res_cach = self.phonemizer.phonemize("cách")
        res_kech = self.phonemizer.phonemize("kếch")
        self.assertIn("æ", res_cach)
        self.assertIn("e", res_kech)
        self.assertNotIn("æ", res_kech)

        res_manh = self.phonemizer.phonemize("mạnh")
        res_menh = self.phonemizer.phonemize("mệnh")
        self.assertIn("æ", res_manh)
        self.assertIn("e", res_menh)

    def test_six_lexical_tones(self):
        """Verifies that all 6 Vietnamese tones map to expected arrow symbols."""
        # 1: ngang (level) -> → (or implicit if unmarked)
        # 2: huyền (falling) -> ↘
        # 3: sắc (rising) -> ↗
        # 4: hỏi (dipping) -> ↓
        # 5: ngã (glottalized rising) -> ʔ↗
        # 6: nặng (glottalized falling) -> ʔ↓
        phrase = "Ma mà má mả mã mạ"
        res = self.phonemizer.phonemize(phrase)
        tokens = res.split()
        self.assertEqual(len(tokens), 6)
        self.assertIn("↘", tokens[1])
        self.assertIn("↗", tokens[2])
        self.assertIn("↓", tokens[3])
        self.assertIn("ʔ↗", tokens[4])
        self.assertIn("ʔ↓", tokens[5])

    def test_punctuation_and_whitespace(self):
        """Verifies punctuation cleanup, en-dash conversion and whitespace preservation."""
        text = "Xin chào, bạn thế nào? Rất vui được gặp bạn! (test & demo – ok)"
        res = self.phonemizer.phonemize(text)
        self.assertIn(",", res)
        self.assertIn("?", res)
        self.assertIn("!", res)
        self.assertIn("(", res)
        self.assertIn(")", res)
        self.assertIn("—", res)  # en-dash converted to em-dash
        self.assertNotIn("&", res)  # ampersand converted to space
        self.assertNotIn("*", res)

    def test_curly_apostrophes(self):
        """Curly apostrophes normalized to straight apostrophes."""
        res1 = self.phonemizer.phonemize("don't")
        res2 = self.phonemizer.phonemize("don’t")
        self.assertEqual(res1, res2)

    def test_deterministic_output(self):
        """Phonemizer produces identical output for repeated calls."""
        text = "Trí tuệ nhân tạo HODUSON giúp nâng cao hiệu suất làm việc."
        out1 = self.phonemizer.phonemize(text)
        out2 = self.phonemizer.phonemize(text)
        out3 = phonemize_vietnamese(text)
        self.assertEqual(out1, out2)
        self.assertEqual(out2, out3)


    def test_km_unit_pronunciation(self):
        """Verifies that 'km' and 'KM' units are pronounced as 'ki lô mét' in Vietnamese context."""
        expected_km_phonemes = "kˈi lˈo mˈɛ↗t"
        english_km_spelling = "kˌeɪˈɛm"

        test_cases = [
            "10 km",
            "3.260 km",
            "5 KM",
            "Quãng đường dài 12 km.",
        ]
        for tc in test_cases:
            res = self.phonemizer.phonemize(tc)
            self.assertIn(
                expected_km_phonemes,
                res,
                f"Failed for '{tc}': expected '{expected_km_phonemes}' in '{res}'",
            )
            self.assertNotIn(
                english_km_spelling,
                res,
                f"Failed for '{tc}': found English spelling '{english_km_spelling}' in '{res}'",
            )

    def test_km_negative_cases(self):
        """Verifies that words containing 'km' as a substring are NOT mistakenly normalized to 'ki lô mét'."""
        km_phonemes = "kˈi lˈo mˈɛ↗t"
        for word in ["kmart", "bookmark", "KMART"]:
            res = self.phonemizer.phonemize(word)
            self.assertNotIn(
                km_phonemes,
                res,
                f"Negative case failed for '{word}': should NOT contain '{km_phonemes}' in '{res}'",
            )

    def test_km_per_hour_unit(self):
        """Verifies that 'km/h' is pronounced as 'ki lô mét trên giờ' in Vietnamese context."""
        text = "Tốc độ là 60 km/h."
        res = self.phonemizer.phonemize(text)
        self.assertIn("kˈi lˈo mˈɛ↗t", res)
        self.assertNotIn("kˌeɪˈɛm", res)


if __name__ == "__main__":
    unittest.main()

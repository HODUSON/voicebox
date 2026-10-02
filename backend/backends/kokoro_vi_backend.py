"""
Kokoro Vietnamese TTS backend implementation.

Wraps the Kokoro-Vietnamese model for fast, lightweight Vietnamese text-to-speech.
82M parameters, CPU realtime, 24kHz output.

Architecture B: KokoroVietnamese PyTorch on CPU.
Uses pre-built Vietnamese voice style vectors (14 preset voices).

Licensing:
- Kokoro-Vietnamese upstream source: Apache-2.0
- sea-g2p: Apache-2.0 (direct dependency)
- vig2p dependency removed; replaced by independent Vietnamese phonemizer adapter (backend.phonemizers.vietnamese_kokoro)
"""

import asyncio
import logging
from threading import RLock
from typing import Optional

import numpy as np

from .base import (
    combine_voice_prompts as _combine_voice_prompts,
    model_load_progress,
)

logger = logging.getLogger(__name__)

# HuggingFace repo for Kokoro-Vietnamese model and voicepacks
KOKORO_VI_HF_REPO = "contextboxai/Kokoro-Vietnamese"
KOKORO_VI_SAMPLE_RATE = 24000

# Default voice if none specified
KOKORO_VI_DEFAULT_VOICE = "diem_trinh"

# 14 audited Vietnamese preset voices: (voice_id, display_name, gender, lang_code)
# Upstream metadata (voices.json) contains only label and filename without gender fields.
# Names with clear Vietnamese gender associations are assigned accordingly.
# 'storyvert' has no upstream gender; 'male' is an unverified fallback to conform
# to Voicebox's binary gender schema ('male' | 'female').
KOKORO_VI_VOICES = [
    ("diem_trinh", "Diễm Trinh", "female", "vi"),
    ("hung_thinh", "Hưng Thịnh", "male", "vi"),
    ("mai_linh", "Mai Linh", "female", "vi"),
    ("mai_loan", "Mai Loan", "female", "vi"),
    ("manh_dung", "Mạnh Dũng", "male", "vi"),
    ("my_yen", "Mỹ Yến", "female", "vi"),
    ("ngoc_huyen", "Ngọc Huyền", "female", "vi"),
    ("phat_tai", "Phát Tài", "male", "vi"),
    ("thanh_dat", "Thành Đạt", "male", "vi"),
    ("thuc_trinh", "Thục Trinh", "female", "vi"),
    ("tuan_ngoc", "Tuấn Ngọc", "male", "vi"),
    ("storyvert", "Storyvert", "male", "vi"),
    ("duc_an", "Đức An", "male", "vi"),
    ("duc_duy", "Đức Duy", "male", "vi"),
]

VALID_VOICE_IDS = {vid for vid, _name, _gender, _lang in KOKORO_VI_VOICES}


class KokoroViTTSBackend:
    """Kokoro Vietnamese 82M TTS backend — lightweight, CPU-friendly."""

    def __init__(self):
        self._model = None
        self._device: str = "cpu"
        self.model_size = "default"
        self._voicepacks: dict = {}  # voice_id -> torch.Tensor
        self._current_voice: Optional[str] = None
        self._state_lock = RLock()

    @property
    def device(self) -> str:
        return self._device

    def is_loaded(self) -> bool:
        with self._state_lock:
            return self._model is not None

    def _get_model_path(self, model_size: str = "default") -> str:
        return KOKORO_VI_HF_REPO

    def _is_model_cached(self, model_size: str = "default") -> bool:
        """Check if Kokoro Vietnamese model files are cached locally."""
        from .base import is_model_cached

        return is_model_cached(
            KOKORO_VI_HF_REPO,
            required_files=["config.json", "kokoro_vi.pth"],
        )

    async def load_model(self, model_size: str = "default") -> None:
        """Load the Kokoro Vietnamese model."""
        with self._state_lock:
            if self._model is not None:
                return
        await asyncio.to_thread(self._load_model_sync)

    def _load_model_sync(self):
        """Synchronous model loading on CPU with double-check locking."""
        with self._state_lock:
            if self._model is not None:
                return

            model_name = "kokoro-vi"
            is_cached = self._is_model_cached()

            with model_load_progress(model_name, is_cached):
                import kokoro_vietnamese.core
                from ..phonemizers.vietnamese_kokoro import phonemize_vietnamese

                # Override kokoro_vietnamese phonemizer with independent implementation
                kokoro_vietnamese.core.phonemize = phonemize_vietnamese

                from kokoro_vietnamese import KokoroVietnamese

                logger.info("Loading Kokoro Vietnamese 82M on CPU...")
                self._model = KokoroVietnamese(
                    repo_id=KOKORO_VI_HF_REPO,
                    voice=KOKORO_VI_DEFAULT_VOICE,
                    device="cpu",
                )
                self._current_voice = KOKORO_VI_DEFAULT_VOICE
                if hasattr(self._model, "voicepack") and self._model.voicepack is not None:
                    self._voicepacks[KOKORO_VI_DEFAULT_VOICE] = self._model.voicepack

            logger.info("Kokoro Vietnamese 82M loaded successfully on CPU")

    def _ensure_voice(self, voice_name: str) -> None:
        """Ensure the specified voicepack is active on the model (caller must hold _state_lock or it acquires re-entrant lock)."""
        if voice_name not in VALID_VOICE_IDS:
            available = ", ".join(sorted(VALID_VOICE_IDS))
            raise ValueError(f"Unknown preset voice '{voice_name}' for kokoro_vi. Available: {available}")

        with self._state_lock:
            if (
                self._current_voice == voice_name
                and self._model is not None
                and getattr(self._model, "voicepack", None) is not None
            ):
                return

            import torch
            from huggingface_hub import hf_hub_download

            if voice_name not in self._voicepacks:
                voicepack_path = hf_hub_download(
                    repo_id=KOKORO_VI_HF_REPO,
                    filename=f"voicepacks/{voice_name}.pt",
                )
                self._voicepacks[voice_name] = torch.load(
                    voicepack_path,
                    map_location="cpu",
                    weights_only=True,
                )

            if self._model is not None:
                self._model.voicepack = self._voicepacks[voice_name]
                self._current_voice = voice_name

    def unload_model(self) -> None:
        """Unload model to free memory."""
        with self._state_lock:
            if self._model is not None:
                del self._model
                self._model = None
                self._voicepacks.clear()
                self._current_voice = None

                logger.info("Kokoro Vietnamese unloaded")

    async def create_voice_prompt(
        self,
        audio_path: str,
        reference_text: str,
        use_cache: bool = True,
    ) -> tuple[dict, bool]:
        """
        Create voice prompt for Kokoro Vietnamese.

        Kokoro Vietnamese uses preset voice style vectors, not audio-based cloning.
        Fallback returns the default preset voice.
        """
        return {
            "voice_type": "preset",
            "preset_engine": "kokoro_vi",
            "preset_voice_id": KOKORO_VI_DEFAULT_VOICE,
        }, False

    async def combine_voice_prompts(
        self,
        audio_paths: list[str],
        reference_texts: list[str],
    ) -> tuple[np.ndarray, str]:
        """Combine voice prompts — uses base implementation for audio concatenation."""
        return await _combine_voice_prompts(
            audio_paths, reference_texts, sample_rate=KOKORO_VI_SAMPLE_RATE
        )

    async def generate(
        self,
        text: str,
        voice_prompt: dict,
        language: str = "vi",
        seed: Optional[int] = None,
        instruct: Optional[str] = None,
    ) -> tuple[np.ndarray, int]:
        """
        Generate audio from text using Kokoro-Vietnamese.

        Args:
            text: Vietnamese text to synthesize
            voice_prompt: Dict containing preset_voice_id
            language: Language code (vi)
            seed: Random seed for reproducibility
            instruct: Not supported by Kokoro (ignored)

        Returns:
            Tuple of (audio_array, sample_rate)
        """
        await self.load_model()

        voice_name = (
            voice_prompt.get("preset_voice_id")
            or voice_prompt.get("kokoro_vi_voice")
            or voice_prompt.get("kokoro_voice")
            or KOKORO_VI_DEFAULT_VOICE
        )

        def _generate_sync():
            import torch

            # Voice switching, seed setting, and synthesis must share the same critical section
            # to guarantee atomic voicepack state and prevent concurrent requests from overwriting
            # each other's random seed before synthesis begins.
            with self._state_lock:
                if self._model is None:
                    raise RuntimeError("Kokoro Vietnamese model is not loaded")

                self._ensure_voice(voice_name)

                if seed is not None:
                    torch.manual_seed(seed)

                audio, _phonemes = self._model.synthesize(text, speed=1.0)

                if audio is None or len(audio) == 0:
                    raise RuntimeError("Kokoro Vietnamese returned empty audio")

                return audio.astype(np.float32), KOKORO_VI_SAMPLE_RATE

        return await asyncio.to_thread(_generate_sync)

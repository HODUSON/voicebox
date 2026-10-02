# THIRD-PARTY SOFTWARE NOTICES AND ATTRIBUTIONS

**Product:** HODUSON Voice Studio  
**Version:** 0.6.0  
**Repository:** https://github.com/HODUSON/voicebox  
**Bundle Identifier:** io.github.hoduson.voicestudio  

This document contains licensing, copyright, and third-party attribution notices for open-source components bundled with, linked into, or redistributed alongside HODUSON Voice Studio.

---

## 1. Primary Upstream Project: Voicebox

- **Component:** Voicebox
- **Purpose:** Core desktop application architecture, multi-engine routing, timeline editor, sidecar server process orchestration, and user interface foundation.
- **Source:** https://github.com/jamiepine/voicebox
- **License:** MIT License
- **Copyright:**
  ```text
  Copyright (c) 2026 Voicebox Contributors
  Created by Jamie Pine & contributors
  ```
- **Distribution Note:** HODUSON Voice Studio is a fork of Voicebox. In accordance with the MIT License, the original copyright notice and permission notice are preserved in full in the root `LICENSE` file. The root source code and Rust crate (`Cargo.toml`) are published under the MIT License; however, the compiled desktop application distribution contains bundled runtime components governed by additional licenses as documented below.

---

## 2. Kokoro-Vietnamese (Engine & Model)

- **Component:** Kokoro-Vietnamese
- **Purpose:** 82M-parameter text-to-speech model and phonemizer pipeline adapted for the Vietnamese language (14 natural voices across Northern, Central, and Southern dialects).
- **Source:** https://github.com/iamdinhthuan/Kokoro-Vietnamese (Pinned commit: `a249afe5555aec6c435165c2f61ec0f71284812f`)
- **License:** Apache License 2.0
- **Copyright / Notice:**
  - Upstream Notice Status: `NO UPSTREAM NOTICE FILE FOUND`
  - Canonical License Text: Available in `licenses/THIRD_PARTY_LICENSES/Apache-2.0.txt`
- **Distribution Note:** Included in backend runtime for text normalization, Vietnamese phoneme conversion, and acoustic inference.

---

## 3. sea-g2p (Southeast Asian Grapheme-to-Phoneme)

- **Component:** sea-g2p
- **Author:** Phạm Nguyễn Ngọc Bảo
- **Purpose:** Rule-based and dictionary-driven grapheme-to-phoneme converter for Southeast Asian languages, used independently for Vietnamese phonemization.
- **Source:** https://github.com/pnnbao97/sea-g2p (PyPI: `sea-g2p`)
- **License:** Apache License 2.0
- **Copyright / Notice:**
  - Upstream Notice Status: `NO UPSTREAM NOTICE FILE FOUND`
  - Canonical License Text: Available in `licenses/THIRD_PARTY_LICENSES/Apache-2.0.txt`
- **Distribution Note:** Packaged with backend sidecar binary (`voicebox-server.exe`). Operates locally and offline without external API calls.

---

## 4. Kokoro TTS (Base Model & Runtime)

- **Component:** Kokoro-82M & misaki
- **Purpose:** Compact, high-speed multi-lingual TTS engine (English, Japanese, Chinese, French, etc.) and G2P runtime.
- **Source:** https://huggingface.co/hexgrad/Kokoro-82M and https://github.com/hexgrad/kokoro
- **License:** Apache License 2.0
- **Copyright / Notice:**
  - Upstream Notice Status: `NO UPSTREAM NOTICE FILE FOUND`
  - Canonical License Text: Available in `licenses/THIRD_PARTY_LICENSES/Apache-2.0.txt`
- **Distribution Note:** Model weights are downloaded on demand from Hugging Face Hub to user app data directory; runtime inference code is bundled in backend sidecar.

---

## 5. Hugging Face Transformers & Accelerate

- **Component:** `transformers`, `accelerate`, `huggingface_hub`
- **Purpose:** Model architecture definitions, tokenizers, tensor utilities, and local Hugging Face model cache management.
- **Source:** https://github.com/huggingface/transformers
- **License:** Apache License 2.0
- **Copyright / Notice:**
  ```text
  Copyright 2018-The Hugging Face team. All rights reserved.
  Licensed under the Apache License, Version 2.0.
  ```
- **Distribution Note:** Bundled into `voicebox-server.exe` sidecar. Canonical Apache-2.0 license text provided in `licenses/THIRD_PARTY_LICENSES/Apache-2.0.txt`.

---

## 6. PyTorch & Torchaudio

- **Component:** `torch`, `torchaudio`
- **Purpose:** Tensor computations, neural network runtime, autograd engine, audio tensor resamplers, and CUDA/CPU acceleration.
- **Source:** https://github.com/pytorch/pytorch
- **License:** Modified BSD 3-Clause License
- **Copyright:**
  ```text
  From PyTorch:
  Copyright (c) 2016-     Facebook, Inc            (Adam Paszke)
  Copyright (c) 2014-     Facebook, Inc            (Soumith Chintala)
  Copyright (c) 2011-2014 Idiap Research Institute (Ronan Collobert)
  Copyright (c) 2012-2014 Deepmind Technologies    (Koray Kavukcuoglu)
  Copyright (c) 2011-2012 NEC Laboratories America (Koray Kavukcuoglu)
  Copyright (c) 2011-2013 NYU                      (Clement Farabet)
  Copyright (c) 2006-2010 NEC Laboratories America (Ronan Collobert, Leon Bottou, Iain Melvin, Jason Weston)
  Copyright (c) 2006      Idiap Research Institute (Samy Bengio)
  Copyright (c) 2001-2004 Idiap Research Institute (Ronan Collobert, Samy Bengio, Johnny Mariethoz)
  All rights reserved.
  ```
- **Distribution Note:** CPU build runtime bundled into `voicebox-server.exe` (and optional CUDA PyTorch libraries downloaded on supported configurations). Canonical BSD-3-Clause license text provided in `licenses/THIRD_PARTY_LICENSES/BSD-3-Clause.txt`.

---

## 7. Pedalboard (Spotify Audio Effects) & Subcomponents

- **Component:** `pedalboard`
- **Installed Version:** `0.9.25` (Repository requirement: `pedalboard>=0.9.0`)
- **Purpose:** Digital Signal Processing (DSP) audio effects pipeline (Reverb, Delay, Highpass/Lowpass Filters, PitchShift, Compressor, Chorus, Gain).
- **Source:** https://github.com/spotify/pedalboard/tree/v0.9.25
- **License:** GNU General Public License v3.0 (GPLv3)
- **Subcomponents & Internal Component Licenses:**
  - `pedalboard` core: GPLv3 (Copyright 2021-2026 Spotify AB)
  - `JUCE 6`: Dual commercial / GPLv3 (Copyright (C) 2021 Raw Material Software Limited)
  - `VST3 SDK`: GPLv3 (Copyright (C) 2019 Steinberg Media Technologies GmbH)
  - `Rubber Band Library`: Commercial / GPLv2 or later (Breakfast Quay)
  - `FFTW`: GPLv2 or later (Matteo Frigo, Steven G. Johnson, MIT)
  - `libmp3lame`: LGPLv2 upstream; treated and upgraded under GPLv3 within Pedalboard distribution as documented upstream
  - `libgsm`: ISC License (GPLv3 compatible)
  - `dr_wav`: Public Domain / Unlicense (David Reid, https://github.com/mackron/dr_libs, https://unlicense.org/); used to decode WAV formats not natively supported by JUCE (ADPCM, A-law, µ-law, 64-bit float)
- **AnalogTapeModel Classification:**
  - Status: `UPSTREAM NOTICE / TEST-ONLY`
  - Rationale: The upstream NOTICE file mentions binaries from Jatin Chowdhury's `AnalogTapeModel` project. Verification confirms these binaries are used exclusively for upstream automated test suites and are **not** bundled into HODUSON production `voicebox-server.exe` sidecar.
- **Verbatim Upstream NOTICE File:**
  ```text
  Pedalboard
  Copyright 2021 Spotify AB

  This product includes software developed at
  Spotify AB (http://www.spotify.com/).

  This product includes software from JUCE (GPLv3).
  * Copyright (C) 2021 Raw Material Software Limited.

  JUCE includes software from the VST3 SDK, licensed under the GPLv3.
  * Copyright (C) 2019 Steinberg Media Technologies GmbH.

  The tests for `pedalboard` include binaries from Jatin Chowdhury's
  `AnalogTapeModel` project, licensed under the GPLv3. The source code
  for these binaries can be found at:
  https://github.com/jatinchowdhury18/AnalogTapeModel
  ```
- **GPL Distribution Status & Review Classification:**
  ```text
  PEDALBOARD LICENSE: GPLv3 VERIFIED
  GPL DISTRIBUTION STATUS: REVIEW
  CLOSED-SOURCE DISTRIBUTION: NOT APPROVED
  OPEN-SOURCE PUBLIC RELEASE: REQUIRES GPL DISTRIBUTION CHECKLIST
  ```
- **Distribution & License Texts:**
  - Exact GPLv3 license text: `licenses/THIRD_PARTY_LICENSES/pedalboard-GPL-3.0.txt`
  - Exact upstream notice text: `licenses/THIRD_PARTY_LICENSES/pedalboard-NOTICE.txt`
  - **Corresponding Source Release Gate:** Before any HODUSON Voice Studio binary is publicly distributed, the complete corresponding source for that exact release, including build scripts and modifications required to reproduce the distributed covered components, must be made publicly accessible at no further charge and clearly linked next to the binary release. (Current local source for release v0.6.0 is uncommitted and not yet published on GitHub; public source release will occur concurrently with the official binary release).

---

## 8. Rust Desktop & Audio Libraries

### Tauri Framework
- **Component:** `tauri`, `tauri-build`, `tauri-plugin-*`
- **Source:** https://github.com/tauri-apps/tauri
- **License:** Apache License 2.0 OR MIT License

### Hound
- **Component:** `hound` (WAV audio decoder and encoder in Rust)
- **Source:** https://github.com/ruuda/hound
- **License:** Apache License 2.0
- **Copyright:** Copyright (c) 2015 Ruud van Asseldonk

### Symphonia
- **Component:** `symphonia` (Pure-Rust multimedia audio decoder)
- **Source:** https://github.com/pdeljanov/Symphonia
- **License:** Mozilla Public License 2.0 (MPL-2.0)
- **Copyright:** Copyright (c) 2019-2022 The Symphonia Developers
- **Distribution Note:** Dynamically/statically linked in Rust. Unmodified source code for Symphonia is available at https://github.com/pdeljanov/Symphonia. Exact MPL-2.0 text provided in `licenses/THIRD_PARTY_LICENSES/MPL-2.0.txt`.

### cpal
- **Component:** `cpal` (Cross-platform audio I/O library in Rust)
- **Source:** https://github.com/RustAudio/cpal
- **License:** Apache License 2.0

---

## 9. Core Python Utility Packages

| Package | Version | License | Copyright / Source |
| :--- | :--- | :--- | :--- |
| `fastapi` | 0.142.2 | MIT | Copyright (c) Sebastián Ramírez |
| `uvicorn` | 0.54.0 | BSD-3-Clause | Copyright (c) Encode OSS Ltd |
| `pydantic` | 2.13.5 | MIT | Copyright (c) Samuel Colvin et al. |
| `sqlalchemy` | 2.1.1 | MIT | Copyright (c) SQLAlchemy authors |
| `alembic` | 1.20.0 | MIT | Copyright (c) Michael Bayer |
| `numpy` | 1.26.4 | BSD-3-Clause | Copyright (c) NumPy Developers |
| `numba` | 0.60.0 | BSD-2-Clause | Copyright (c) Anaconda, Inc. |
| `soundfile` | 0.14.0 | BSD-3-Clause | Copyright (c) Bastian Bechtold |
| `librosa` | 0.11.0 | ISC | Copyright (c) librosa development team |
| `pillow` | 12.3.0 | HPND | Copyright (c) Jeffrey A. Clark et al. |
| `httpx` | 0.28.1 | BSD-3-Clause | Copyright (c) Encode OSS Ltd |
| `loguru` | 0.7.3 | MIT | Copyright (c) Delgan |
| `fastmcp` | 3.4.7 | Apache-2.0 | Copyright (c) FastMCP contributors |

---

## 10. Packaging & Distribution of Legal Files

All legal notices and full text licenses are bundled into the desktop installer via Tauri v2 `bundle.resources` mapping:

```text
licenses/
├── LICENSE
├── THIRD_PARTY_NOTICES.md
└── THIRD_PARTY_LICENSES/
    ├── Apache-2.0.txt
    ├── BSD-3-Clause.txt
    ├── MIT.txt
    ├── MPL-2.0.txt
    ├── pedalboard-GPL-3.0.txt
    └── pedalboard-NOTICE.txt
```

End users installing HODUSON Voice Studio receive a complete, unbundled copy of these legal files within the installed application directory.

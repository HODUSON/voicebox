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
- **Installed & Tested Version:** `0.10.0` (Pinned: `sea-g2p==0.10.0`)
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

### phonemizer-fork
- **Component:** `phonemizer-fork`
- **Installed Version:** `3.3.2`
- **Purpose:** Multilingual text-to-phoneme conversion library. Required at runtime by upstream `misaki.espeak` (imported during `kokoro.pipeline` initialization for standard Kokoro English G2P synthesis).
- **Source:** https://github.com/thewh1teagle/phonemizer-fork (Fork of https://github.com/bootphon/phonemizer)
- **License:** GNU General Public License v3.0 or later (GPL-3.0-or-later)
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.
- **Upstream License Text:** Preserved under GPLv3 terms (see `licenses/THIRD_PARTY_LICENSES/pedalboard-GPL-3.0.txt` for verbatim GPL-3.0 text).

### espeakng-loader
- **Component:** `espeakng-loader`
- **Installed Version:** `0.2.4`
- **Purpose:** Shared library loader and runtime binary assets for eSpeak NG. Required at runtime by `misaki.espeak` to locate and load `espeak-ng.dll` and phonetic data tables (`espeak-ng-data`).
- **Source:** https://github.com/thewh1teagle/espeakng-loader (loader) and https://github.com/espeak-ng/espeak-ng (eSpeak NG engine)
- **License:** MIT License for Python loader wrapper; GNU General Public License v3.0 or later (GPL-3.0-or-later) for bundled native binaries (`espeak-ng.dll`) and dictionary/voice assets.
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.

### Strong Copyleft Distribution Note & Corresponding Source Release Gate
The strong copyleft runtime dependencies bundled into the production backend executable `voicebox-server.exe` carry the following exact licenses:
- `pedalboard` 0.9.25: **GNU General Public License v3.0 (GPL-3.0)**
- `phonemizer-fork` 3.3.2: **GNU General Public License v3.0 or later (GPL-3.0-or-later)**
- `espeak-ng` native runtime assets loaded through `espeakng-loader`: **GNU General Public License v3.0 or later (GPL-3.0-or-later)** according to verified upstream source.

**Corresponding Source Release Gate:**
- **PUBLIC BINARY RELEASE:** `BLOCKED` until matching v0.6.0 corresponding source code and release tag are published to the repository (https://github.com/HODUSON/voicebox).
- **Internal local installer testing:** `ALLOWED`.

---

## 8. Weak / File-Level Copyleft Runtime Dependencies (LGPL-2.1 & MPL-2.0)

### num2words
- **Component:** `num2words`
- **Installed Version:** `0.5.14` (Pinned: `num2words==0.5.14`)
- **Purpose:** Number-to-words conversion across multiple languages, utilized in text normalization preprocessing pipelines.
- **Source:** https://github.com/savoirfairelinux/num2words
- **License:** GNU Lesser General Public License v2.1 or later (LGPL-2.1-or-later)
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.
- **Canonical License Text:** Available in `licenses/THIRD_PARTY_LICENSES/LGPL-2.1.txt`.
- **Distribution Note:** Included as pure-Python modules within the frozen runtime payload. Corresponding source code for `num2words` 0.5.14 is publicly available upstream at https://github.com/savoirfairelinux/num2words.

### soxr (python-soxr)
- **Component:** `soxr`
- **Installed Version:** `1.1.0` (Pinned: `soxr==1.1.0`)
- **Purpose:** High-speed, high-quality 1D audio sample-rate conversion wrapper around the SoX Resampler library (`libsoxr`).
- **Source:** https://github.com/dofuuz/python-soxr
- **License:** GNU Lesser General Public License v2.1 or later (LGPL-2.1-or-later) (Core C library `libsoxr`: LGPL-2.1+; Python wrapper: LGPL-2.1+; PFFFT component: BSD-like).
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.
- **Canonical License Text:** Available in `licenses/THIRD_PARTY_LICENSES/LGPL-2.1.txt`.
- **Distribution Note:** Binary extension module linked in the frozen sidecar. Corresponding source code is publicly accessible at https://github.com/dofuuz/python-soxr and https://sourceforge.net/projects/soxr/.

### certifi
- **Component:** `certifi`
- **Installed Version:** `2026.7.22`
- **Purpose:** Curated collection of Root Certificates for validating SSL/TLS certificates while making network requests.
- **Source:** https://github.com/certifi/python-certifi
- **License:** Mozilla Public License 2.0 (MPL-2.0)
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.
- **Canonical License Text:** Available in `licenses/THIRD_PARTY_LICENSES/MPL-2.0.txt`.

### orjson
- **Component:** `orjson`
- **Installed Version:** `3.12.0`
- **Purpose:** Fast, correct JSON library for Python.
- **Source:** https://github.com/ijl/orjson
- **License:** `MPL-2.0 AND (Apache-2.0 OR MIT)`
- **Classification:** `MIXED / MPL OBLIGATION` (Weak / File-Level Copyleft). Because upstream licensing contains the conjunction `AND`, MPL-2.0 obligations remain active for covered components. Canonical MPL-2.0 text is bundled in `licenses/THIRD_PARTY_LICENSES/MPL-2.0.txt`.
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.

### tqdm
- **Component:** `tqdm`
- **Installed Version:** `4.70.1`
- **Purpose:** Extensible progress meter for loops and command-line interfaces.
- **Source:** https://github.com/tqdm/tqdm
- **License:** `MPL-2.0 AND MIT` (file/contribution-level licensing)
- **Classification:** `MIXED / MPL OBLIGATION` (Weak / File-Level Copyleft). Upstream `LICENCE` contains both MIT and MPL-2.0 terms applying at the file/contribution level. Canonical MPL-2.0 text is bundled in `licenses/THIRD_PARTY_LICENSES/MPL-2.0.txt`.
- **Bundled Status:** Bundled in `voicebox-server.exe` sidecar.

---

## 9. Build Tools & Embedded Bootloader Licensing (PyInstaller)

### PyInstaller
- **Component:** `pyinstaller`
- **Installed Version:** `6.22.3`
- **Purpose:** Build-time packaging tooling used solely to create standalone Windows executables (`voicebox-server.exe`, `voicebox-mcp.exe`).
- **Source:** https://github.com/pyinstaller/pyinstaller
- **License Classification:**
  - Bootloader & loader embedded files: `GPL-2.0-or-later WITH Bootloader Exception`
  - Runtime hooks & helper modules: `Apache-2.0`
- **Exact License File:** `licenses/THIRD_PARTY_LICENSES/PyInstaller-COPYING.txt`
- **Bootloader Exception Text:**
  ```text
  Bootloader Exception
  --------------------
  In addition to the permissions in the GNU General Public License, the
  authors give you unlimited permission to link or embed compiled bootloader
  and related files into combinations with other programs, and to distribute
  those combinations without any restriction coming from the use of those
  files. (The General Public License restrictions do apply in other respects;
  for example, they cover modification of the files, and distribution when
  not linked into a combined executable.)
  ```
- **Distribution Note:** Under the Bootloader Exception, bundling the Python application using PyInstaller does **not** cause the application itself or its independent modules to become subject to the GNU General Public License v2.

### pyinstaller-hooks-contrib
- **Component:** `pyinstaller-hooks-contrib`
- **Installed Version:** `2026.8`
- **Purpose:** Community-maintained hooks for PyInstaller.
- **Source:** https://github.com/pyinstaller/pyinstaller-hooks-contrib
- **Actual Bundled Files:** Runtime hooks only (`_pyinstaller_hooks_contrib/rthooks/*`)
- **Runtime Hooks License:** `Apache-2.0`
- **Standard GPL Hooks:** `BUILD-TIME ONLY / NOT BUNDLED` (Standard hooks run only during compilation on the build machine and are not packaged into runtime executables).
- **Classification:** Build Tool / Runtime Hook (Permissive runtime hook).

---

## 10. Rust Desktop & Audio Libraries

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

## 11. Core Python Utility Packages

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

## 12. Packaging & Distribution of Legal Files

All legal notices and full text licenses are bundled into the desktop installer via Tauri v2 `bundle.resources` mapping:

```text
licenses/
├── LICENSE
├── THIRD_PARTY_NOTICES.md
└── THIRD_PARTY_LICENSES/
    ├── Apache-2.0.txt
    ├── BSD-3-Clause.txt
    ├── LGPL-2.1.txt
    ├── MIT.txt
    ├── MPL-2.0.txt
    ├── pedalboard-GPL-3.0.txt
    ├── pedalboard-NOTICE.txt
    └── PyInstaller-COPYING.txt
```

End users installing HODUSON Voice Studio receive a complete, unbundled copy of these legal files within the installed application directory.

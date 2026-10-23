"""Offline tests for the optional VOICEVOX adapter (no voice model needed)."""

import importlib.util
import os
from pathlib import Path
import sys
import tempfile
import types
import unittest
from unittest.mock import patch


SCRIPT = Path(__file__).resolve().parents[1] / "speak.py"
spec = importlib.util.spec_from_file_location("waifu_speak", SCRIPT)
speak = importlib.util.module_from_spec(spec)
spec.loader.exec_module(speak)


class VoicevoxTests(unittest.TestCase):
    def test_cached_dialogue_skips_second_synthesis(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            (root / "models" / "vvms").mkdir(parents=True)
            (root / "dict" / "open_jtalk_dic_utf_8-1.11").mkdir(parents=True)
            (root / "onnxruntime" / "lib").mkdir(parents=True)
            (root / "models" / "vvms" / "0.vvm").write_bytes(b"model")
            (root / "onnxruntime" / "lib" / "test-onnxruntime.dll").write_bytes(b"runtime")

            calls = {"synthesis": 0, "runtime": 0}
            played = []

            class FakeOnnxruntime:
                LIB_RECOMMENDED_VERSIONED_FILENAME = "test-onnxruntime.dll"

                @staticmethod
                def load_once(*, filename):
                    calls["runtime"] += 1
                    return object()

            class FakeOpenJtalk:
                def __init__(self, path):
                    self.path = path

            class FakeVoiceModelFile:
                @classmethod
                def open(cls, path):
                    return cls()

                def __enter__(self):
                    return self

                def __exit__(self, *exc):
                    pass

            class FakeSynthesizer:
                def __init__(self, *args):
                    pass

                def load_voice_model(self, model):
                    pass

                def tts(self, text, style_id):
                    calls["synthesis"] += 1
                    return b"RIFF-test-wav"

            pkg = types.ModuleType("voicevox_core")
            sub = types.ModuleType("voicevox_core.blocking")
            sub.Onnxruntime = FakeOnnxruntime
            sub.OpenJtalk = FakeOpenJtalk
            sub.Synthesizer = FakeSynthesizer
            sub.VoiceModelFile = FakeVoiceModelFile

            env = {
                "WAIFU_VOICEVOX_HOME": str(root),
                "WAIFU_VOICE_CACHE_DIR": str(root / "cache"),
                "WAIFU_VOICEVOX_STYLE_ID": "0",
            }

            with (
                patch.dict(os.environ, env),
                patch.dict(sys.modules, {
                    "voicevox_core": pkg,
                    "voicevox_core.blocking": sub,
                }),
                patch.object(speak, "_play_wav", side_effect=played.append),
            ):
                speak._speak_voicevox("おかえりなさい")
                speak._speak_voicevox("おかえりなさい")

            self.assertEqual(calls["synthesis"], 1)
            self.assertEqual(calls["runtime"], 1)
            self.assertEqual(played, [b"RIFF-test-wav"] * 2)
            self.assertEqual(len(list((root / "cache").glob("*.wav"))), 1)

    def test_unknown_backend_fails_cleanly(self):
        with (
            patch.dict(os.environ, {"WAIFU_TTS_BACKEND": "unknown"}),
            patch.object(sys, "argv", ["speak.py", "テスト"]),
        ):
            self.assertEqual(speak.main(), 1)


if __name__ == "__main__":
    unittest.main()

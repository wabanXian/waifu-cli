# -*- coding: utf-8 -*-
"""Optional on-demand TTS for waifu-cli.

WAIFU_TTS_BACKEND=cevio (default) or voicevox.
VOICEVOX requires the official Python wheel plus downloaded runtime, dictionary,
and VVM assets. No VOICEVOX editor or HTTP server is needed.
"""

import hashlib
import os
from pathlib import Path
import sys


def _speak_cevio(text: str) -> None:
    import clr

    dll_path = r"C:\Program Files\CeVIO\CeVIO AI\CeVIO.Talk.RemoteService2.dll"
    clr.AddReference(dll_path)

    from CeVIO.Talk.RemoteService2 import ServiceControl2
    from CeVIO.Talk.RemoteService2.InteroperableComponents import Talker2

    if not ServiceControl2.IsHostStarted:
        ServiceControl2.StartHost(True)

    talker = Talker2()
    talker.Cast = "さとうささら"
    talker.Emotion = "元気"
    talker.EmotionLevel = 80
    talker.Volume = 85
    talker.Speed = 50
    talker.Tone = 100
    state = talker.Speak(text)
    state.Wait()


def _voicevox_settings():
    home_text = os.environ.get("WAIFU_VOICEVOX_HOME", "").strip()
    home = Path(home_text).expanduser() if home_text else (
        Path(__file__).resolve().parent / "voicevox_core"
    )

    dictionary = home / "dict" / "open_jtalk_dic_utf_8-1.11"
    model_text = os.environ.get("WAIFU_VOICEVOX_MODEL", "").strip()
    model = Path(model_text).expanduser() if model_text else (
        home / "models" / "vvms" / "0.vvm"
    )

    try:
        style_id = int(os.environ.get("WAIFU_VOICEVOX_STYLE_ID", "0"))
    except ValueError as exc:
        raise RuntimeError("WAIFU_VOICEVOX_STYLE_ID 必须是整数") from exc

    if not dictionary.is_dir():
        raise RuntimeError(
            f"VOICEVOX 日语词典不存在：{dictionary}；请设置 WAIFU_VOICEVOX_HOME"
        )
    if not model.is_file():
        raise RuntimeError(
            f"VOICEVOX 声音模型不存在：{model}；请设置 WAIFU_VOICEVOX_MODEL"
        )

    return home, dictionary, model, style_id


def _play_wav(wav: bytes) -> None:
    # Windows built-in PCM WAV playback; no third-party player or GUI process.
    import winsound

    winsound.PlaySound(wav, winsound.SND_MEMORY | winsound.SND_SYNC)


def _speak_voicevox(text: str) -> None:
    home, dictionary, model, style_id = _voicevox_settings()

    # Cache first: repeated fixed dialogue needs no model initialization at all.
    stat = model.stat()
    signature = f"{model.resolve()}|{stat.st_size}|{stat.st_mtime_ns}|{style_id}|{text}"
    cache_key = hashlib.sha256(signature.encode("utf-8")).hexdigest()
    cache_root = os.environ.get("WAIFU_VOICE_CACHE_DIR", "").strip()
    cache_dir = (
        Path(cache_root).expanduser()
        if cache_root else
        Path(os.environ.get("LOCALAPPDATA", str(Path.home() / ".cache")))
        / "waifu-cli" / "voice-cache"
    )
    cache_file = cache_dir / f"{cache_key}.wav"

    if cache_file.is_file():
        _play_wav(cache_file.read_bytes())
        return

    try:
        from voicevox_core.blocking import (
            Onnxruntime, OpenJtalk, Synthesizer, VoiceModelFile
        )
    except ImportError as exc:
        raise RuntimeError(
            "找不到 VOICEVOX CORE Python wheel。"
            "仅下载 C API DLL 不等于安装 Python API；请安装官方对应 wheel。"
        ) from exc

    onnx_file = home / "onnxruntime" / "lib" / (
        Onnxruntime.LIB_RECOMMENDED_VERSIONED_FILENAME
    )
    if not onnx_file.is_file():
        raise RuntimeError(
            f"VOICEVOX ONNX Runtime 不存在：{onnx_file}；"
            "请使用官方 downloader 下载运行库"
        )

    runtime = Onnxruntime.load_once(filename=str(onnx_file))
    synthesizer = Synthesizer(runtime, OpenJtalk(str(dictionary)))
    with VoiceModelFile.open(str(model)) as voice_model:
        synthesizer.load_voice_model(voice_model)
        wav = synthesizer.tts(text, style_id)

    cache_dir.mkdir(parents=True, exist_ok=True)
    temporary = cache_dir / f"{cache_key}.{os.getpid()}.tmp"
    try:
        temporary.write_bytes(wav)
        os.replace(temporary, cache_file)
    finally:
        temporary.unlink(missing_ok=True)

    _play_wav(wav)


def main() -> int:
    backend = os.environ.get("WAIFU_TTS_BACKEND", "cevio").strip().lower()

    try:
        if len(sys.argv) > 1 and sys.argv[1] == "--check":
            print(f"语音后端：{backend}")
            if backend == "voicevox":
                home, dictionary, model, style_id = _voicevox_settings()
                from voicevox_core.blocking import Onnxruntime
                runtime = home / "onnxruntime" / "lib" / (
                    Onnxruntime.LIB_RECOMMENDED_VERSIONED_FILENAME
                )
                if not runtime.is_file():
                    raise RuntimeError(f"VOICEVOX ONNX Runtime 不存在：{runtime}")
                print(f"VOICEVOX 根目录：{home}")
                print(f"日语词典：{dictionary}")
                print(f"声音模型：{model}")
                print(f"声音风格 ID：{style_id}")
                print("VOICEVOX 基础检查通过（尚未执行合成）")
            elif backend == "cevio":
                print("CeVIO 模式：实际发声时检查宿主服务")
            else:
                raise RuntimeError(f"不支持的语音后端：{backend}")
            return 0

        text = sys.argv[1] if len(sys.argv) > 1 else "何も言うことがないよ〜"
        if backend == "voicevox":
            _speak_voicevox(text)
        elif backend == "cevio":
            _speak_cevio(text)
        else:
            raise RuntimeError(
                f"不支持的语音后端 {backend!r}，请使用 cevio 或 voicevox"
            )
        return 0
    except Exception as exc:
        print(f"语音播放失败：{exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())

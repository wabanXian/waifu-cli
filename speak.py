# speak.py
import sys
import clr
import winsound
# winsound.Beep(1000, 300)
# print("hi from Python")

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

text = sys.argv[1] if len(sys.argv) > 1 else "何も言うことがないよ〜"
state = talker.Speak(text)
state.Wait()
Add-Type -Path "${env:ProgramFiles}\CeVIO\CeVIO AI\CeVIO.Talk.RemoteService2.dll"

[CeVIO.Talk.RemoteService2.ServiceControl2]::StartHost($true) >$null
$talker = New-Object CeVIO.Talk.RemoteService2.InteroperableComponents.Talker2

#キャスト設定
$talker.Cast = "さとうささら";

#（例）音量設定
$talker.Volume = 50

#（例）再生
$state = $talker.Speak("こんにちは")
$state.Wait()
$state = $talker.Speak("さとうささらです")
$state.Wait()

#（例）音素データ取得
$phonemes = $talker.GetPhonemes("はじめまして")

# （例）音素データをトレース出力
foreach ($phoneme in $phonemes.Core)
{
    Write-Host ("" + $phoneme.Phoneme + " " + $phoneme.StartTime + " " + $phoneme.EndTime)
}
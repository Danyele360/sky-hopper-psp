@echo off
cd /d "%~dp0"
if not exist ".tools\ppsspp\PPSSPPWindows64.exe" (
  echo PPSSPP non trovato. Scaricalo da https://www.ppsspp.org/download/
  echo e apri dist\SkyHopper.EBOOT.PBP.
  pause
  exit /b 1
)
start "Sky Hopper PSP" ".tools\ppsspp\PPSSPPWindows64.exe" "%~dp0dist\SkyHopper.EBOOT.PBP"

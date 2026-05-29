@echo off
REM Build Windows + Linux + macOS release folders (bypasses PowerShell script policy).
cd /d "%~dp0.."
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0build-all.ps1" %*
if errorlevel 1 exit /b 1

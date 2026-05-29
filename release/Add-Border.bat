@echo off
REM ImageArtBorder - single-file launcher (drag-and-drop or pass path as argument)
REM Author: Júlio Papel <info@juliopapel.pt>
REM Edit BORDER and COLOR below to match your studio defaults.

setlocal
set "TOOL_DIR=%~dp0"
set "EXE=%TOOL_DIR%ImageArtBorder.exe"
set "BORDER=40"
set "COLOR=#FFFFFF"

if "%~1"=="" (
    echo Usage: drag an image onto this file, or:
    echo   Add-Border.bat "C:\path\to\image.jpg"
    exit /b 1
)

if not exist "%EXE%" (
    echo Error: ImageArtBorder.exe not found in %TOOL_DIR%
    exit /b 1
)

"%EXE%" -b %BORDER% -c %COLOR% -f "%~1"
set "ERR=%ERRORLEVEL%"
if %ERR% neq 0 (
    echo ImageArtBorder failed with exit code %ERR%
)
exit /b %ERR%

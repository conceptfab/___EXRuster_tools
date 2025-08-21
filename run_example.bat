@echo off
REM Skrypt do uruchamiania aplikacji z przykladowa konfiguracja.

REM --- ZMIEN TE WARTOSCI ---
set SOURCE_FOLDER="C:\_cloud\___EXRuster_tools\data"
set DEST_FOLDER="C:\_cloud\___EXRuster_tools\thumb"
set THUMB_HEIGHT=256
set INFO_FILENAME="stats.txt"
REM --- KONIEC KONFIGURACJI ---

echo [INFO] Uruchamianie aplikacji z ponizszymi ustawieniami:
set | findstr "SOURCE_FOLDER="
set | findstr "DEST_FOLDER="
set | findstr "THUMB_HEIGHT="
set | findstr "INFO_FILENAME="
echo.

.\target\release\readEXR.exe --source-folder %SOURCE_FOLDER% --dest-folder %DEST_FOLDER% --height %THUMB_HEIGHT% --info %INFO_FILENAME%

echo.

echo [INFO] Praca aplikacji zakonczona.
pause
@echo off
REM Skrypt do kompilacji projektu Rust w trybie release

echo [INFO] Rozpoczynanie kompilacji projektu exr_thumbnailer...

cargo build --release

if %errorlevel% equ 0 (
    echo [SUCCESS] Kompilacja zakonczona sukcesem!
    echo Plik wykonywalny znajduje sie w: .\target\release\exr_thumbnailer.exe
) else (
    echo [ERROR] Wystapil blad podczas kompilacji.
)

echo.
pause
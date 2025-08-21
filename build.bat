@echo off
REM Skrypt do kompilacji projektu Rust w trybie release

echo [INFO] Rozpoczynanie kompilacji projektu readEXR...

cargo build --release

if %errorlevel% equ 0 (
    echo [SUCCESS] Kompilacja zakonczona sukcesem!
    echo Plik wykonywalny znajduje sie w: .\target\release\readEXR.exe
) else (
    echo [ERROR] Wystapil blad podczas kompilacji.
)

echo.
pause
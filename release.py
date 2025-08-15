#!/usr/bin/env python3
import subprocess
import sys
import os

def main():
    # Ścieżka do pliku wykonywalnego w folderze release
    exe_path = os.path.join("target", "release", "exr_thumbnailer.exe")
    
    # Parametry dla exr_thumbnailer.exe z nowymi funkcjonalnościami
    cmd = [
        exe_path, 
        "-s", "data",           # source folder
        "-d", "thumb",          # destination folder
        "-t", "130",            # thumbnail height
        "-l",                   # enable linear tone mapping
        "-g", "2.2",            # gamma correction
        "-f", "lanczos3"        # scaling filter algorithm
    ]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
        print("Nowe funkcjonalności:")
        print("  - Linear tone mapping dla HDR")
        print("  - Gamma correction (2.2)")
        print("  - Filtr skalowania: Lanczos3")
        result = subprocess.run(cmd, check=True)
        print("Polecenie wykonane pomyślnie!")
    except subprocess.CalledProcessError as e:
        print(f"Błąd podczas wykonywania polecenia: {e}")
        sys.exit(1)
    except FileNotFoundError:
        print(f"Błąd: Nie znaleziono pliku '{exe_path}'. Upewnij się, że projekt został zbudowany.")
        print("Użyj: cargo build --release")
        sys.exit(1)

if __name__ == "__main__":
    main()

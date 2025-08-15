#!/usr/bin/env python3
import subprocess
import sys
import os

def main():
    # Ścieżka do pliku wykonywalnego w folderze release
    exe_path = os.path.join("target", "release", "exr_thumbnailer.exe")
    
    # Parametry dla exr_thumbnailer.exe
    cmd = [exe_path, "-s", "data", "-d", "thumb", "-t", "200"]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
        result = subprocess.run(cmd, check=True)
        print("Polecenie wykonane pomyślnie!")
    except subprocess.CalledProcessError as e:
        print(f"Błąd podczas wykonywania polecenia: {e}")
        sys.exit(1)
    except FileNotFoundError:
        print(f"Błąd: Nie znaleziono pliku '{exe_path}'. Upewnij się, że projekt został zbudowany.")
        sys.exit(1)

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import subprocess
import sys
import os

def main():
    # Ścieżka do pliku wykonywalnego w folderze release
    exe_path = os.path.join("target", "release", "exruster_tools.exe")
    
    # Parametry dla exruster_tools.exe z nowymi funkcjonalnościami TIFF
    cmd = [
        exe_path, 
        "-s", "data",           # source folder
        "-d", "tiff",           # destination folder
        "-c", "deflate",        # TIFF compression (deflate dla lepszej kompresji)
        "--info", "statystyki.txt"  # custom stats filename
    ]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
        print("Nowe funkcjonalności TIFF:")
        print("  - Konwersja EXR do TIFF z zachowaniem rozdzielczości")
        print("  - Kompresja Deflate (najlepsza kompresja)")
        print("  - Przetwarzanie wsadowe wszystkich plików EXR")
        print("  - Statystyki w pliku 'statystyki.txt'")
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

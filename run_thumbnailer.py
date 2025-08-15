#!/usr/bin/env python3
import subprocess
import sys

def main():
    # Parametry dla polecenia cargo run z nowymi funkcjonalnościami TIFF
    cmd = [
        "cargo", "run", "--",
        "-s", "data",           # source folder
        "-d", "tiff",           # destination folder
        "-c", "lzw",            # TIFF compression (lzw, deflate, none)
        "--info", "konwersja.txt"  # custom stats filename
    ]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
        print("Nowe funkcjonalności TIFF:")
        print("  - Konwersja EXR do TIFF z zachowaniem rozdzielczości")
        print("  - Kompresja LZW (domyślna)")
        print("  - Przetwarzanie wsadowe wszystkich plików EXR")
        print("  - Statystyki w pliku 'konwersja.txt'")
        result = subprocess.run(cmd, check=True)
        print("Polecenie wykonane pomyślnie!")
    except subprocess.CalledProcessError as e:
        print(f"Błąd podczas wykonywania polecenia: {e}")
        sys.exit(1)
    except FileNotFoundError:
        print("Błąd: Nie znaleziono polecenia 'cargo'. Upewnij się, że Rust jest zainstalowany.")
        sys.exit(1)

if __name__ == "__main__":
    main()

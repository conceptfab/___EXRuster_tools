#!/usr/bin/env python3
import subprocess
import sys

def main():
    # Parametry dla polecenia cargo run z poprawnymi opcjami
    cmd = [
        "cargo", "run", "--",
        "-s", "data",           # source folder
        "-d", "PNG",            # destination folder
        "--stats", "konwersja.txt"  # custom stats filename
    ]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
        print("Konwersja EXR do PNG:")
        print("  - Folder źródłowy: data")
        print("  - Folder docelowy: PNG")
        print("  - Konwersja EXR do PNG z zachowaniem rozdzielczości")
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

#!/usr/bin/env python3
import subprocess
import sys

def main():
    # Parametry dla polecenia cargo run z nowymi funkcjonalnościami
    cmd = [
        "cargo", "run", "--",
        "-s", "data",           # source folder
        "-d", "thumb",          # destination folder
        "-t", "200",            # thumbnail height
        "-l",                   # enable linear tone mapping
        "-g", "2.2",            # gamma correction
        "-f", "gaussian"        # scaling filter algorithm (gaussian dla szybszego skalowania)
    ]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
        print("Nowe funkcjonalności:")
        print("  - Linear tone mapping dla HDR")
        print("  - Gamma correction (2.2)")
        print("  - Filtr skalowania: Gaussian (szybszy niż Lanczos3)")
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

#!/usr/bin/env python3
import subprocess
import sys

def main():
    # Parametry dla polecenia cargo run
    cmd = ["cargo", "run", "--", "-s", "data", "-d", "thumb", "-t", "200"]
    
    try:
        print("Uruchamiam: " + " ".join(cmd))
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

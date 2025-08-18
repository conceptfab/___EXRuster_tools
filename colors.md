# Sugestia: Rozpoznawanie warstw czarno-białych vs. kolorowych

**Opis funkcji:**
Aplikacja powinna mieć możliwość rozpoznawania, czy dana warstwa w pliku EXR zawiera wyłącznie kolory czarno-białe (lub odcienie szarości), czy też zawiera szerszą gamę kolorów. Na podstawie tej analizy, można by podjąć decyzje o sposobie eksportu (np. automatyczne zapisywanie jako 8-bitowe pliki dla warstw czarno-białych, bez konieczności używania prefiksu).

**Wyzwania implementacyjne:**

1.  **Definicja "czarno-białe" / "kolorowe":**
    *   Należy precyzyjnie zdefiniować, co oznacza "czarno-białe". Czy to tylko czysta czerń i biel, czy też obejmuje wszystkie odcienie szarości?
    *   Jakie kryteria określają "kolorową" warstwę? Czy wystarczy jeden piksel o innym odcieniu, czy musi być ich więcej?

2.  **Koszt obliczeniowy:**
    *   Analiza każdego piksela w każdej warstwie w celu określenia jej palety kolorów może być bardzo zasobochłonna, zwłaszcza dla dużych obrazów i wielu warstw. Może to znacząco wydłużyć czas konwersji.

3.  **Metody analizy:**
    *   **Kwantyzacja kolorów:** Można by spróbować zredukować liczbę kolorów w warstwie do bardzo małej liczby (np. 2 dla czerni i bieli) i sprawdzić, czy obraz nadal wygląda poprawnie.
    *   **Analiza histogramu:** Sprawdzenie histogramu kolorów warstwy. Jeśli wszystkie wartości kolorów (R, G, B) są identyczne dla każdego piksela, a wartości te mieszczą się w zakresie od 0 do 1, to prawdopodobnie jest to warstwa w skali szarości.
    *   **Progowanie:** Zastosowanie progowania do wartości pikseli i sprawdzenie, czy wszystkie piksele mieszczą się w określonych zakresach dla czerni i bieli.

4.  **Heurystyka:**
    *   Prawdopodobnie konieczne będzie zastosowanie heurystyk, które mogą nie być w 100% dokładne w każdym przypadku, ale będą wystarczające dla większości zastosowań.

**Potencjalne korzyści:**
*   Automatyzacja procesu eksportu dla warstw czarno-białych/szarości.
*   Zmniejszenie rozmiaru plików dla warstw, które naturalnie są czarno-białe, bez ręcznego oznaczania prefiksami.

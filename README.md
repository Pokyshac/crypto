# Crypto
----------------------------------------
## Task 1
Для заданного $m \in N$ вывести таблицу вывести таблицу Кэли для операций сложения и умножения.

## Task 2
Для заданных $a \in Z$ и $b \in N$, при $(a,m)=1$, вывести систему вычетов для $a\cdot x + b$ по модулю $m$, где $x$ пробегает полную систему вычетов.

## Task 3
Для заданного $m \in N$ вычислить значение функции Эйлера $\phi(m)$.

## Task 4
Для заданного $m \in N$ найти все $n \in N$, такие что $\phi(n)=m$.

## Task 5
Для заданных $a \in Z$ и $k, m \in N$ вычислить остаток при делении $a^k$ на $m$.

## Task 6
Реализовать алгоритм RSA для шифрования и дешифрования текста.

## Task 7
Реализовать алгоритм нахождения решений системы сравнений:
```math
$$\begin{cases}x\equiv a \; (mod \; m)\\x\equiv b \; (mod \; n)\end{cases}$$
```

----------------------------------------
## Utils
Содержит некоторые вспомогательные функции:

 - modulo - деление с остатком, где остаток неотрицателен, возвращает целую часть и остаток в виде пары значений.
 - get_prime_factors - возвращает простые делители натурального числа без повторений.
 - euclidean_algorithm - расширенный алгоритм Евклида.

equal(X, X).
parent(john, mary).
parent(mary, alice).
grandparent(X, Y) :- parent(X, Z), parent(Z, Y).
head([H|Y], H).
test_is(X):- X is 2.
test_is_extra(X) :- equal(X, 2).
listbreak([H|Y], Y).
sum_list([], 0).
sum_list([H|T], Sum) :-
    sum_list(T, Rest),
    Sum is H + Rest.

head_plus_2([H|Y], X) :- X is H + 2.
factorial(0, 1).
factorial(N, F) :- N > 0, N1 is N - 1, factorial(N1, F1), F is N * F1.

tac([], []).
tac([H|T], Reversed) :- 
    tac(T, RevT), 
    append(RevT, [H], Reversed).


fib(0, 0).
fib(1, 1).
fib(N, F) :-
    N > 1,
    N1 is N - 1,
    N2 is N - 2,
    fib(N1, F1),
    fib(N2, F2),
    F is F1 + F2.
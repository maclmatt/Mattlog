equal(X, X).

head([H|Y], H).
a.
test_is(X):- X is 2.
test_is_extra(X) :- equal(X, 2).
listbreak([H|Y], Y).
b.

sum_list([], 0).
sum_list([H|T], Sum) :-
    sum_list(T, Rest),
    Sum is H + Rest.

head_plus_2([H|Y], X) :- X is H + 2.

factorial(0, 1).
factorial(N, F) :- N > 0, N1 is N - 1, factorial(N1, F1), F is N * F1.

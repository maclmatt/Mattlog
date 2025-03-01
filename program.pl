equal(X, X).
parent(john, mary).
parent(mary, alice).
grandparent(X, Y) :- parent(X, Z), parent(Z, Y).
head([H|Y], H).
a.
test_is(X):- X is 2.
test_is_extra(X) :- equal(X, 2).
listbreak([H|Y], Y).
b.

sum_list([], 0).              % Base case: empty list sums to 0
sum_list([H|T], Sum) :-
    sum_list(T, Rest),        % Recursive call to sum the tail
    Sum is H + Rest.

head_plus_2([H|Y], X) :- X is H + 2.
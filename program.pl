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
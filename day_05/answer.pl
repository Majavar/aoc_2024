% checks rule is valid
is_invalid([E|L]) :- order(X, E), member(X, L), !.
is_invalid([_|L]) :- is_invalid(L).

% Convert list of atoms to list of numbers
to_numbers([], []).
to_numbers([H|T], [N|T2]) :- atom_number(H, N), to_numbers(T, T2).

% Convert string to rule
to_rule([], []).
to_rule([H|T], [R|T2]) :- split_string(H, ",", "", L), to_numbers(L, R), to_rule(T, T2).

:- initialization(main).

% Get filename from command line or default value
filename([], 'example').
filename([F], F).

main :- 
    current_prolog_flag(argv, Arg),
    filename(Arg, F),
    read_file_to_string(F, S, []),
    split_string(S, "\n", "", L),
    parse_input(L).

% Reaching end of rules
parse_input([""|R]) :-
    process(R, Valid, Invalid),
    score(Valid, Score),
    score_fixed(Invalid, ScoreFixed),
    write('Part 1: '), write(Score),nl,
    write('Part 2: '), write(ScoreFixed),nl, halt.

parse_input([Rule|L]) :- 
    split_string(Rule, "|", "", [First, Second]),
    atom_number(First, F),
    atom_number(Second, S),
    assert(order(F, S)),
    parse_input(L).

process(R, V, I) :- to_rule(R, Rules), process(Rules, [], [], V, I).
process([], V, I, V, I).
process([Rule|L], V, I, [Rule|Valids], Invalids) :- \+ is_invalid(Rule), process(L, V, I, Valids, Invalids).
process([Rule|L], V, I, Valids, [Rule|Invalids]) :- is_invalid(Rule), process(L, V, I, Valids, Invalids).


% Evaluate rule
rule_value(R, Value) :- length(R, L), Half is L // 2 + 1, nth1(Half, R, Value).

score([], 0).
score([R|L], Score) :- score(L, S), rule_value(R, V), Score is S + V.

score_fixed([], 0).
score_fixed([R|L], Score) :- score_fixed(L, S), find_center(R, V), Score is S + V.

count_order(_, [], C, C).
count_order(E, [H|T], C, R) :- order(E, H), N is C + 1, count_order(E, T, N, R).
count_order(E, [H|T], C, R) :- order(H, E), count_order(E, T, C, R).
count_order(E, [E|T], C, R) :- count_order(E, T, C, R).

fixed_score(E, L, S) :- member(E, L), count_order(E, L, 0, S).

find_center(R, V) :-
    length(R, L),
    H is L // 2,
    member(V, R), 
    count_order(V, R, 0, H).
https://docs.ruby-lang.org/en/3.4/syntax/control_expressions_rdoc.html

These grammar snippets are mine. Could be wrong.
# [X] `if` expr

`<IF> ::=`
```
if <EXPR> (then)?
	<STATEMENTS>
(end|<ELSIF>|<ELSE>)
```

`<ELSIF> ::=`
```
elsif <EXPR> (then)?
	<STATEMENTS>
(end|<ELSIF>|<ELSE>)
```

`<ELSE> ::=`
```
else
	<STATEMENTS>
end
```

# [ ] Ternary `if`

# [ ] `unless` expr
`<UNLESS> ::=`
```
unless <EXPR> (then)?
	<STATEMENTS>
(end|<ELSIF>|<ELSE>)
```
≡
```
if (not <EXPR>) (then)?
	<STATEMENTS>
(end|<ELSIF>|<ELSE>)
```

# [ ] Modifier `if` and `unless`

# [ ] `case` expr

# [ ] `while` loop

`<WHILE> ::=`
```
while <EXPR> (do)?
	<STATEMENTS>
end
```

# [ ] `until` loop
`<UNTIL> ::=`
```
until <EXPR> (do)?
	<STATEMENTS>
end
```

# [ ] `for` loop

# [ ] Modifier `while` and `until`

# [ ] `break` stmt

# [ ] `next` stmt

# [ ] `redo` stmt

# [ ] Flip-Flop expr

# [ ] `throw` / `catch`
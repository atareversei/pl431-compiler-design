# Parsing

The role of a parser is to transform a set of tokens into a valid AST (Abstract Syntax Tree). This note is mainly about parsing CFG.

## Implementation Parser Types

Hand-written parsers:

- recursive-descent parsers

Automatically generated:

- LL(1) .. LL(k)
- LR(k): SLR(1), LALR(1)
- GLR
- PEG

## Alphabet and Languages

Alphabet is a set of available symbols:

- {a, b}

Language is also a set, which includes all strings made from the alphabet

- L1 = {a, b, aa, bb, ab, ba, bbb, ...}

We could start adding restrictions on top of languages. For instance in this case the restriction of allowing only 2-character strings has resulted in this language. These restrictions form the **grammar** of our language:

- L2 = {aa, ab, ba, bb}

## CFG

- can have one non terminal on the left hand side.
- No context on the left hand side
- Right hand side can hold any mix of terminals and non terminals

## BNF: Backus-Naur form

```txt
S -> aS
S -> bA
A -> eps
A -> cA
```

There are also other notations. For example for regular languages, we use the RegExp notation: `a*bc*`

## Left-Most Derivation and Right-Most Derivation

Left-most derivation is simpler and it is used in recursive descent:

```txt
E : E + E
  | E * E
  | number

2 + 4 * 9

1.      E + E
2. number + E
3. number + E * E
4. number + number * E
5. number + number * number
```

Right-most derivation is more powerful and it is used in more expressive languages:

```txt
E : E + E
  | E * E
  | number

2 + 4 * 9

1.                    E * E
2.               E * number
3.           E + E * number
4.      E + number * number
5. number + number * number
```

## Ambiguous Grammars

Grammars in the previous section were ambiguous, meaning that we can start with `E*E` derivation for the left-most example and `E+E` for the right-most derivation and still get the same result. This would result in different answers for arithmetic operations displayed in the examples.

```txt
            E                            E
            |                            |
       |----+----|                  |----*----|
       |         |                  |         |
       E         E                  E         E
       |         |                  |         |
       |         |                  |         |
      num     |--*--|            |--+--|     num
              |     |            |     |
              |     |            |     |
             num   num          num   num


       2+(5*3) = 17                   (2+5)*3=21
```

To resolve ambiguity, we have to enforce:

- Correct associativity. Left-associativity is achieved by left recursion.
- Correct Precedence. Closer to the start symbol, lower the precedence.

## Top Down Parsers

Feels the most intuitive. Starts from the root node and works its way down to the leaves.

- Backtracking: It backtracks if it selects a wrong path.
- Predictive It predicts which path is correct to skip backtracking. Recursive Descent typically uses this technique though it can also be implemented as backtracking algorithm.

Top Down parsers cannot handle left recursion at all.

```txt
E : E + T
  | T
  ;

T : T * F
  | F
  ;

F : number
  ;
```

Top Down parsers are often implemented as handler functions like the code snippet below which causes infinite recursion:

```js
function E() {
  E() && term("+") && T();
}
```

### Backtracking Recursive Descent

Turning the mentioned grammar into a right recursion might help but it doesn't respect the initial precedence and associativity so the parser has to backtrack:

```txt
E : T + E
  | T
  ;

T : F * T
  | F
  ;

F : number
  ;
```

and backtracking for this grammar is implemented as:

```js
function saveCursor() {
  savedCursor = cursor;
}
function backtrack() {
  cursor = savedCursor;
}

// notice the saveCursor function that saves the state before doing anything so we can backtrack of things don't work out
function E() {
  return (saveCursor(), E1()) | I(backtrack(), saveCursor(), E2());
}

function E1() {
  return T() && term("+") && E();

  function E2() {
    return T();
  }
}
```

Backtracking happens for two reasons:

- Some productions have the same prefix (the grammar is not left factored) e.g. E->T+E and E->T

## Bottom Up Parsers

Starts from the leaves and works its way top as if they have somehow seen the whole tree beforehand. They try to reduce the leaves to the starting root element, this is why they are also known as shift-reduce parsers.

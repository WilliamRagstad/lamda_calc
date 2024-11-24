# Untyped Lambda Calculus

This is a simple implementation of the untyped [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus) in Rust.
It is extended with variable assignments to terms.

## Example

```hs
((\x.(\x.x))x)
```

Is a valid term that evaluates to `λx.x`.

```hs
(((λx.λy.x y) (λz.z)) (λw.w))
```

Simply becomes `λw.w`.

```hs
((λx.(x x)) (λx.(x x)))
```

Is a non-terminating term.

```hs
F = λx.λy.(x y)
G = λz.z
H = λw.w
((F G) H)
```

Uses assignments to simplify the last term `(F G) H`, and reduces to `λz.z`.

## Usage

Start a REPL by running the following command:

```bash
./lambda_calc
```

Or run a file with lambda calculus terms:

```bash
./lambda_calc examples/identity.lc
```

## Fundamentals

The lambda calculus is a formal system for expressing computation based on function abstraction and application using variable bindings.
It is a universal model of computation that can express any computation that can be performed by a Turing machine.

The reduction operation that drives computation is mainly $β$-reduction, which is the application of a function to an argument.

$$
(\lambda x.M)\ N \quad\rightarrow\quad M[x:=N]
$$

Where $M[x:=N]$ is the result of replacing all free occurrences of $x$ in the body of the abstraction $M$ with the argument expression $N$.
The second fundamental operation is $α$-conversion ($(\lambda x.M[x])\rightarrow (\lambda y.M[y])$), which is the renaming of bound variables in an expression to avoid name collisions.

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

### Data Encoding

Boolean logic can be encoded using Church booleans:

```hs
True  = λx.λy.x
False = λx.λy.y
```

And logical operations:

```hs
Not = λb.((b False) True)
And = λa.λb.((a b) False)
Or  = λa.λb.((a True) b)
```

> [!NOTE] Try evaluating the following terms yourself
>
> ```hs
> (Not True)
> ((Or False) True)
> ((And True) False)
> ```

#### Numbers

Natural numbers can be encoded using Church numerals:

```hs
Zero  = λf.λx.x
One   = λf.λx.(f x)
Two   = λf.λx.(f (f x))
Three = λf.λx.(f (f (f x)))
```

And arithmetic operations:

```hs
Succ = λn.λf.λx.(f ((n f) x))
Add  = λm.λn.λf.λx.((m f) ((n f) x))
Mul  = λm.λn.λf.λx.((m (n f)) x)
```

> [!NOTE] Try evaluating the following terms yourself
>
> ```hs
> (Succ Zero)
> (Add One Two)
> (Mul Two Three)
> ```

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

## Extension

The implementation is extended with variable assignments to terms.
This allows for the definition of terms that can be used later in the evaluation of other terms using an environment $\Gamma$ mapping names to terms.

```hs
(((λx.x) (λx.x)) (λx.x))
```

Or with assignments:

```hs
Id = λx.x
((Id Id) Id)
```

The term `Id` can now be used in other terms to simplify expressions.
Both terms evaluate to `λx.x`.

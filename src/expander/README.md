# Expander

The idea of binding is [Binding as sets of scopes](https://dl.acm.org/doi/10.1145/2837614.2837620), this paper shows an essentail way to implement hygienic macro, which seems better than renaming based algorithm

## Scopes Set

```scheme
(let ([x 1])
  (lambda (y)
    z))
```

- the `let` form corresponds to a scope $a_{let}$.
- the `lambda` form corresponds to a $b_{lam}$.
- The set of scopes associated with `z` is $\{a_{let},b_{lam}\}$.

### Example

```scheme
(let ([x 1])
  (let-syntax ([m (syntax-rules () [(m) x])])
    (lambda (x)
      (m))))
```

so, right-hand side of `m` binding has the scope set $\{a_{let}\}$, while the final `m` has the scope $\{a_{let}, b_{ls}, c_{lam}\}$; now expand `m` (we can obviously find the correct `m`) we get `x`, but this associated with scope $\{a_{let}, d_{intro}\}$, the $d_{intro}$ is about the macro expansion.

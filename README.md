# a sketch of a design for generators etc

## TL;DR

### Traits

Introduce the following traits

```rust
trait Stream { /* push */ }
trait IntoStream { }

trait Iterator { /* pull */ }
trait IntoIterator { }
impl<I: Iterator> Stream for I { /* for somewhat silly reasons, not a supertrait */ }

trait Generator: Stream { /* pinned-pull */ }
impl<G> Iterator for G where G: Generator + Unpin { }
```

...and corresponding async variants.

`gen { .. }` (resp. `async gen { .. }`) syntax would return a `impl Generator` (resp. `impl AsyncGenerator`). The compiler would generate one impl of `Generator` that generates the "next" version (which requires the "suspendable transformation") and a `Stream` impl where each call to yield just invokes the `op` function. Note that the stream impl would just use normal stack variables and doesn't require any transformation.

## For loop desugaring

Modify the for-loop desugaring so that, if the body does not `break`/`continue`/`return`/`?`, it uses the `Stream::exec` method with a closure argument. This gives more optimal code-gen and permits the use of `for` with streams (so long as you avoid break).

## Idiom shift: prefer `impl IntoStream` over `impl IntoIterator`

Most functions should take `impl IntoStream`, not `impl IntoIterator`.
It's more general and will typically give you better codegen.

## Advantages

* you only need pinning if you are taking a generator and using is an explicit iterator -- i.e., calling next
    * in particular, people can write `fn foo(x: impl IntoStream)` and call it like `foo(gen { ... })`
when you use things as a stream, you get optimal codegen
* if you're sticking to combinators or simple for loops, you can just use it as a stream

## Disadvantages

* overall complexity
* idiom shift (prefer `impl IntoStream`) will leave ecosystem in a messy state for some time to come
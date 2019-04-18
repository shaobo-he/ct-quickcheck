# Anntation Syntax

## Type annotation
ct-quickcheck requires users to explicit annotate whether a variable is private or public.
The type annation syntax is as follows,
```
(Public|Private)<orig-type(,lower-bound,higher-bound(,length-bound))?>
```
For example, the follow annotation represents a type that is private and ranges from `'1'` to `'9'`.
```rust
Private<char, '1', '9'>
```

## Function annotation
We use Rust's attribute macros to annotate the function to check. The syntax of such annotation is as follows,
```
#[ct-quickcheck entry-point]
fn Check(arg-list) -> return-type {pre-cond}
```
, where `arg-list` has syntax like `arg-id: arg-type` and `func-spec` are a expression evaluating to `bool`.
For example, to check a C function,
```C
void foo(char* x, unsigned len);
```
, its corresponding annotation is,
```rust
#[ct-quickcheck foo]
fn Check(x: Private<Vec<char>>, len: Public<u32>) -> void {x.len()>=len}
```

# Auto-complete Rust Code at Compile Time

```rust
fn main() {
    println!("factorial 5 = {}", factorial(6));

    let u = (0.0, 1.0);
    let v = (5.0, 5.0);
    println!("projection test: {:?} * {:?} = {:?}", u, v, project(u, v));
}

autofill::autofill! {

    /// < 0 should return 1
    fn factorial(i: i32) -> i32 { todo!() }

    /// project u onto v
    fn project(u: (f64, f64), v: (f64, f64)) -> (f64, f64) { todo!() }
}
```
* make sure `ANTHROPIC_API_KEY` environment variable is set during compilation, for example by adding this to .cargo/config.toml:
```
[env]
ANTHROPIC_API_KEY = "<your key here>"
```

* to autocomplete, run your project with the AUTOFILL environment variable set to true: `AUTOFILL=true cargo run`. (NOTE: you most likely do not want to add this to the crate environment, since
* this makes every LSP change result in a new API call).
* caches reponses based on the hash of the text inside the macro.

# TODO
- automatically add #[allow(unused)] to variables inside not-yet-completed code.
- provide a custom error message at compile time if:
  - an autofilled area didn't compile correctly
  - an autofilled area wasn't cached and AUTOFILL was not set to true.
  - others?
- expose config options like model name, temp, context length, etc. both at the project level and invocation level.
- completion config for other model providers.

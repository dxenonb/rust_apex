A WIP Apex parser (Apex as used with Salesforce), written in Rust.

To build and run, in a terminal execute:

```
cargo test
```

This will run all the examples in the `test/examples` folder, as well as 
(of course) all the min tests. Note that many of these are not valid, compiling
Apex - they only test the parser implementation. There is no type checking, 
signature checking, etc.

I am just getting started on this, this is my first attempt at using pest for 
parsing.
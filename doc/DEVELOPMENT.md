# Atalanta development guidelines

### set up development machine

To run the end-to-end tests you need the tools that these tests call installed,
e.g. `node`, `npm`, `cargo`, etc.

### run tests

Run a single end-to-end test:

- add the `@this` tag to the test, like so:

  ```cucumber
  @this
  Scenario: foo
  ```
- run `make cukethis`

# Anchor CRUD Program LiteSVM Example

This repository is an example of how to test an anchor program with the litesvm testing library. The test file is heavily commented to help explain how to correctly write these tests.

This example uses an escrow program to showcase using the `litesvm` crate for a program that only handles data.

If your program also uses tokens, review the [escrow example](https://github.com/brimigs/anchor-escrow-with-litesvm) instead.

For full litesvm documentation, go to [litesvm.com](https://www.litesvm.com/)

To run litesvm tests:

```shell
anchor build
cargo test
```

These tests cover:

- Litesvm test environment setup
- Building anchor program instructions
- Building and sending transactions

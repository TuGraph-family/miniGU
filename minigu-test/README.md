# miniGU-test

This crate contains system-level tests for miniGU.

## Test Framework

We integrated two test framework in this crate: `insta` and `sqllogictest`. Generally, `insta` is more flexible and customizable. It also has command line `cargo insta` for reviewing and testing, which is more friendly when coding for new queries.

## Insta Usage

See [insta doc](https://insta.rs/docs) for detailed information.

### Unit Tests

You can run the gql unit tests with cargo or cargo-insta

```
cargo test -p minigu-test
cargo insta test -p minigu-test
```

And then to review the diffs by

```
cargo insta review/accept/reject
```

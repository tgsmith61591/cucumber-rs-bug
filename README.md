# `cucumber-rs` bug repro

### Setup:

```bash
❯ rustup default 1.74.0
```

### Run the monolithic feature file:

```bash
❯ make test-monolith
```

This tests 500 scenarios and takes **~6 minutes**

### Run the distributed feature file:

```bash
❯ make test-distributed
```

This tests 500 scenarios and takes **~20 seconds**

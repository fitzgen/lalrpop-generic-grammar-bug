# Steps to Reproduce

```
$ git clone https://github.com/fitzgen/lalrpop-generic-grammar-bug.git`
$ cd lalrpop-generic-grammar-bug
$ cargo build
```

# Expected

Builds A-OK.

# Actual

Type errors in the generated parser code involving the grammar's generic type parameter's trait's associated types.

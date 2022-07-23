# gh-labelx
[![Rust](https://github.com/siketyan/gh-labelx/actions/workflows/rust.yml/badge.svg)](https://github.com/siketyan/gh-labelx/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/gh-labelx.svg)](https://crates.io/crates/gh-labelx)

Creates issue labels on GitHub idempotently through HashiCorp Terraform.

## Features
- **Idempotent**: Apply twice, changes once.
- **Easy**: No configuration required if you are using gh CLI.
- **Simple**: Just write a simple YAML manifest to create many labels at once.
- **Extensible**: Requests to GitHub are powered by Terraform. You can customise additional labels or properties using hooks after generation.

## Getting started
```shell
gh extension install siketyan/gh-labelx
```

## Usages
First, write your manifest in YAML format:

```yaml
---
labels:
  - name: feature
    color: '0000FF'
  - name: fix
    color: 'FF0000'
  - name: refactor
    color: '00FF00'
```

Check your manifest is correct and can be applied to GitHub:

```shell
gh labelx -o <owner> -r <repo> plan -f <file>
```

Now apply to your GitHub repository:

```shell
gh labelx -o <owner> -r <repo> apply -f <file>
```

It's easy!

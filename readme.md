<div align="center">
  <h1><code>near-rust-intro</code></h1>
  <p>
    <strong><a href="https://docs.near.org/docs/tutorials/contracts/intro-to-rust">An Introduction to Rust Smart Contracts</strong>
  </p>
</div>

## Develop

```shell
make fix 
make qa
make build
make clean
```

### Run CI local

Installation [act](https://github.com/nektos/act):
```shell
brew install act
```

Setup env vars:
```shell
echo "GITHUB_TOKEN=%GITHUB_TOKEN%" | tee .secrets
```

Run
```shell
act --help
```

## Deploy test

```shell
make deploy-force
accountId=ilyar.testnet
contractName=$(cat neardev/dev-account)
near state $contractName
near delete $contractName $accountId
```

## Usage

```shell
near view $contractName get_num
near call $contractName increment --accountId $accountId
near view $contractName get_num
near call $contractName decrement --accountId $accountId
near view $contractName get_num

```

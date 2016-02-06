# dotenv-shell

Launch a new shell (or another program) with your loaded dotenv.

This application is meant to be used on development or testing environments in which setting environment variables is not practical.
It loads environment variables from a .env file, if available, and mashes those with the actual environment variables provided by the operative system.*

_* Adopted from [rust-dotenv](https://github.com/slapresta/rust-dotenv), which handles the complicated part anyway._




## Installation

```bash
cargo install dotenv-shell
```

## Usage

Simply launching your `$SHELL`:

```
dotenv-shell
```

Launching an external program:

```
dotenv-shell my-application
```

## License

MIT. See [LICENSE](LICENSE).

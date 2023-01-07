# envbed

a fast, simple file text replacer with environment variables written in Rust (alternative `envsubst`)

named from `env` + `embed`

<p align="center">
  <img src="./docs/envbed.png">
</p>

# Installation

### curl

```sh
sudo curl -sfL https://raw.githubusercontent.com/igtm/envbed/master/install.sh | sudo sh -s -- -b=/usr/local/bin
```

if you want to download old version, pass `-v` argument.

```sh
sudo curl -sfL https://raw.githubusercontent.com/igtm/envbed/master/install.sh | sudo sh -s -- -b=/usr/local/bin -v=v0.0.1
```

# Usage

```
Usage: envbed [OPTIONS]

Options:
  -f, --file <FILE>                    specifies a target file instead of piped stdin [default: ]
  -e, --env-from-file <ENV_FROM_FILE>  uses env file instead of os environment variables [default: ]
  -p, --env-prefix <ENV_PREFIX>        filters envvars with this prefix (recommended for low security risks) [default: ]
      --template-syntax-double-braces  uses {{FOO}} syntax instead of ${FOO} (avoid conflicts with OS default syntax)
  -w, --override-file                  override a target file (--file)
  -o, --out <OUT>                      specifies a output file instead of stdout [default: ]
  -h, --help                           Print help information
  -V, --version                        Print version information
```

# Example

- replace target.html with out.html using .env

```shell
envbed -f target.html -e .env -o out.html
```

- replace piped stdin with out.html using .env

```shell
cat target.html | envbed -e .env -o out.html
```

- replace piped stdin with out.html using .env to stdout (using redirect)

```shell
cat target.html | envbed -e .env > out.html
```

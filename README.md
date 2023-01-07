# envbed

a faster, simpler text replacer written in Rust (alternative `envsubst`)

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

- replace `${FOO}` in target.html with values in .env and write in out.html

```shell
envbed -f target.html -e .env -o out.html
```

- replace `${FOO}` in piped stdin with values in .env and write in out.html

```shell
cat target.html | envbed -e .env -o out.html
```

- replace `${FOO}` in piped stdin with values in .env and write in stdout (using redirect)

```shell
cat target.html | envbed -e .env > out.html
```

- replace `${FOO}` in target.html with OS environment variables and write in out.html

```shell
envbed -f target.html -o out.html
```

- replace `${FOO}` in target.html with OS environment variables and override target.html

```shell
envbed -f target.html -w
```

- replace `{{FOO}}` in target.html with OS environment variables and override target.html

```shell
envbed -f target.html -w --template-syntax-double-braces
```

# Benchmark (envsubst vs envbed)

| cli        | Command                    | Mean [s]           | Min [s]  | Max [s]  | diff                  |
| :--------- | :------------------------- | :----------------- | :------- | :------- | :-------------------- |
| envsubst   | `envsubst < a.txt > b.txt` | 57.529 s ± 0.623 s | 56.758 s | 58.617 s | 1                     |
| **envbed** | `envbed -f a.txt -o b.txt` | 21.787 s ± 0.243 s | 21.412 s | 22.159 s | **2.64 times faster** |

<details>
<summary>detail terminal log</summary>

```shell
$ hyperfine --warmup 3 'envsubst < a.txt > b.txt'
Benchmark 1: envsubst < a.txt > b.txt
  Time (mean ± σ):     57.529 s ±  0.623 s    [User: 51.381 s, System: 5.908 s]
  Range (min … max):   56.758 s … 58.617 s    10 runs


$ hyperfine --warmup 3 'envbed -f a.txt -o b.txt'
Benchmark 1: envbed -f a.txt -o b.txt
  Time (mean ± σ):     21.787 s ±  0.243 s    [User: 16.008 s, System: 5.510 s]
  Range (min … max):   21.412 s … 22.159 s    10 runs
```

</details>

a.txt: `5.7GB`

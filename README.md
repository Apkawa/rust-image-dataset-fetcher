# Image dataset fetcher

A simple image dataset  fetcher

## Features

### Sources

* [x] *booru via [Apkawa/booru-rs](https://github.com/Apkawa/booru-rs)

## Usage

```
Usage: image-dataset-fetcher [OPTIONS] --target <TARGET> <COMMAND>

Commands:
  booru  Fetch from booru
  help   Print this message or the help of the given subcommand(s)

Options:
  -p, --proxy <PROXY>    Verbosity log debug
  -t, --target <TARGET>  Path to store fetched dataset
  -h, --help             Print help
  -V, --version          Print version

```

```
$ image-dataset-fetcher help booru
Fetch from booru

Usage: image-dataset-fetcher --target <TARGET> booru [OPTIONS]

Options:
  -e, --engine <ENGINE>  Booru engine [default: danbooru]
  -u, --url <URL>        Booru url
  -q, --query <QUERY>    Query
  -l, --limit <LIMIT>    Limit [default: 100]
  -p, --pages <PAGES>    Pages [default: 1]
  -h, --help             Print help
  -V, --version          Print version

```

Example fetch from danbooru

```
image-dataset-fetcher -t /path/to/dataset/ -p https://proxyhost:3143 booru -e danbooru -q "masabodo shiroko_(blue_archive)" -limit 10
```

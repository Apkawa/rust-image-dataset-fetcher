[![CICD](https://github.com/Apkawa/rust-image-dataset-fetcher/actions/workflows/ci.yml/badge.svg)](https://github.com/Apkawa/rust-image-dataset-fetcher/actions/workflows/ci.yml)

# Image dataset fetcher

A simple image dataset  fetcher

## Features

* [x] *booru via [Apkawa/booru-rs](https://github.com/Apkawa/booru-rs)
    * fetch images by query
    * create image_name.txt with tags


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


## Dataset preparation

TODO make command for preparation dataset, see [example](https://github.com/d8ahazard/sd_dreambooth_extension/discussions/443)

1. remove dups via `apt install findimagedupes`

    ```shell
    findimagedupes -R -i 'VIEW(){ for f in "$@";do echo $f;done }' -- /path/to/dataset/ | grep .txt -v | xargs -r -n 1 -I {} rm {}
    ```

2. Resize all image and convert to jpg via `apt install imagemagick`

    ```shell
    mogrify -resize 512x512 -format jpg -path /path/to/dataset_resized/ /path/to/dataset/
    cp /path/to/dataset/*.txt /path/to/dataset_resized/
    ```

3. Rename dataset_resized to format `img/{reperats}_{instance_prompt} {class_prompt}` for `kohya_ss`.
    as example:

    * repeats - `40`
    * instance prompt `masabodo` ? TODO check docs
    * class prompt `1girl` ? TODO check docs

    folder name must be `img/40_masabodo 1girl` - it is image folder for kohya_ss

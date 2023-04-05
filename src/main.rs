use crate::cli::Cli;
use crate::cli::Commands::*;
use booru_rs::client::generic::model::Image;
use booru_rs::client::generic::BooruOptionBuilder;
use clap::Parser;
use std::fs::{create_dir_all, write};
use std::{thread, time};

mod cli;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Booru(booru_cmd) => {
            let engine = &booru_cmd.engine;
            let target = if let Some(q) = booru_cmd.query.as_ref() {
                cli.target.join(format!("{} {}", engine.to_string(), q))
            } else {
                cli.target.join(format!("{} all", engine.to_string()))
            };

            if !target.exists() {
                create_dir_all(&target).unwrap();
            }

            let mut builder = engine
                .builder()
                .proxy(cli.proxy.as_ref())
                .limit(booru_cmd.limit);

            if let Some(url) = booru_cmd.url.as_ref() {
                builder = builder.url(url);
            }

            if let Some(q) = booru_cmd.query.as_ref() {
                builder = builder.tag(q)
            }

            let res = builder.get().unwrap();

            for p in res {
                thread::sleep(time::Duration::from_millis(500));
                let images = p.images();
                let image = if let Some(img) = images.sample {
                    if img.url.is_empty() {
                        images.original
                    } else {
                        Some(img)
                    }
                } else {
                    images.original
                };
                if let Some(Image { url, .. }) = image {
                    println!("{}", url);
                    let fname = url.split('/').last().unwrap();
                    let img_path = target.join(fname);

                    let mut txt_path = target.join(fname.split('.').take(1).collect::<String>());
                    txt_path.set_extension("txt");
                    if !txt_path.exists() {
                        write(txt_path, p.tags().join(", ")).unwrap();
                    }
                    if img_path.exists() {
                        continue;
                    }
                    let media_res = engine
                        .builder()
                        .proxy(cli.proxy.as_ref())
                        .client()
                        .get(url.to_string())
                        .header("User-Agent", USER_AGENT)
                        .send()
                        .unwrap();
                    if media_res.status().is_success() {
                        write(img_path, media_res.bytes().unwrap()).unwrap();
                    } else {
                        dbg!(media_res);
                    }
                }
            }
        }
    }
}

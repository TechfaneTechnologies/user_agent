# User_Agent

[![rust badge](https://img.shields.io/static/v1?label=Made%20with&message=Rust&style=for-the-badge&logo=rust&labelColor=e82833&color=b11522)](https://www.rust-lang.org/)
[![license badge](https://img.shields.io/github/license/TechfaneTechnologies/user_agent?style=for-the-badge)](https://github.com/TechfaneTechnologies/user_agent/blob/main/LICENSE)
[![copyleft badge](https://img.shields.io/static/v1?label=&message=Copyleft&style=for-the-badge&labelColor=silver&color=silver&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAyCAQAAAC0NkA6AAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAAmJLR0QA/4ePzL8AAAAHdElNRQfjAxYBNgYPa+9oAAAEM0lEQVRYw6WYb0zVVRjHP9wQW7umA0xoKNSC+6bSNkzetKZbaVu19aLpfOGcbcw/S+uNbikuNwMsVyE3XVsro7VEXjS3ylmLxkRtC9crHGjCAv9AATK4CoZye8Hl/J7n/M7v8rvX57w55/lznt/583yf5/xyCEOlrKaSCp6ggCiQYJheLvMHv9HHA1MZ++kmmaZ1UUNZ9g6eo4X7aR3Mtvs0syJzB0U0MR3KgddOsiQTFxsZzdDBTLvFetd0OT5OHo1U+7j9tNJBN4MkgChFVLCS1Sz1aR7jHf5Lv4Yov1hfN8YRKgP1V9LIuGVxhmg6Fwv4XalPcJD8OTe3gA+YVHYXgt3kWato46nQp1jOWWs1eW7Fz5VaLbkZ3cdc6pX9UfeNkvd+a1aRtV3Fle+mLeGWEO/0mT/EWo7SxhBjjNDPKfbxtMPNVjHLKMVa+I0Q1lmG89nDTWdctPGqz80hIT+uAWRaGOqzeJEraQOw2YrzXNqNbJrlnqDFsCeJKZO3uDtnnN+wNq6cCSM74SGtd1wHlfrOkHAyyDPKrk5codIZ1n7DSlAoVF9iKjRq/cVCYZnPmJHsnWF1GcYRobiQf3yA3sr7VPM2cXp9br5Va2k0/EsAy4SixKh6a5LT6rQibGBAyaeV9SohWQabzeBvhUcTaoqPHHhdTKfSOaWk1wx/E8TN4CuhssW6pjnOCF/KiNrOxULWZPgNEbEJF4VKFT2mdbGLpNNJPzVqC9eKkTdbDK4ajy9ngVaPiHuU5AshWWe4VyIsMuwbWTi5Q7sYlYj+TdNbFBHpJZEV8vao8sOjMS8VRh64MkumrRhSh5UQ+T278s+jQdF/1PTGI4yaweNZuHiYF1RsyCiapdFcengyNajgZyP4RBhP8RpDAU42KcxqE30vNK7KYJQpploFY1NgnfmvApYiZxpskLAi6/PFVh454HBRyJ9K5yclvS5hJQggP7YA8vvZzJCi1+m3NKoUYnj8Eg31jSonDFuTTPEju9nIZuq55IP6FvUJ3iF0zjBqApLWOu6FTlp9FCgM90rX9/zpt1Z9z56QLkasatnLRfe8TT5pmHetQqI6RAoesB5A5aIy/s5jrxAl0VmrJHqFvrQuflCwCPM4Jy71s1L0tTA75IPzAyo5ea3D8eg5LORf2mWqnGaXz3Q+b3CcDm6nCtBfqeV5R+xsUyf1mC3eoBLp9qzAcocquN90qRxTW/Fhxk+Hw8o+HvQIOqPU2qkI7SLGeauAmhf8YrygVCepU0HmpkLqLaQ7nz43Ra3VJBknzqpA/SrivofpaduF64n9Kdt83OupJ/YA48ACiolRyRpHovuMd5kKs8PrA+JirjbsvlFBlE9DyP8qXnQ3+eNiblpOc+gfOCc0gGRGpeyzymq7dbLXSmch/q24qIQ1VBKjjMLUT7UheunmIq2qQgmg/wHquM6d9tIV7AAAACV0RVh0ZGF0ZTpjcmVhdGUAMjAxOS0wMy0yMlQwMTo1NDowNiswMDowMOIizoUAAAAldEVYdGRhdGU6bW9kaWZ5ADIwMTktMDMtMjJUMDE6NTQ6MDYrMDA6MDCTf3Y5AAAAAElFTkSuQmCC)](https://en.wikipedia.org/wiki/Copyleft)
 [![Build and Release](https://github.com/TechfaneTechnologies/user_agent/actions/workflows/release.yml/badge.svg)](https://github.com/TechfaneTechnologies/user_agent/actions/workflows/release.yml)

USER_AGENT is an easy to use rust CLI program and library for extracting the latest user agents of all browsers on Windows, Mac, Linux, Android, and iOS platforms.

## Features

- Show all of the latest `User Agents` under a second.

## Installation

user_agent is a compiled, statically linked program that does not require any external dependencies.

> user_agent is coming to more package managers soon!

### Cargo Install

If you have the Rust toolchain, you can also compile and install user_agent by running `cargo install user_agent`.

Remember to use an add-on like [cargo-update](https://crates.io/crates/cargo-update) to keep user_agent updated to the latest version!

### GitHub Releases

1. Download the asset suitable for your operating system from [the latest release](https://github.com/TechfaneTechnologies/user_agent/releases/latest)
2. Unzip the file and move it to a folder in your path such as `~/bin`
3. Remember to check the releases page for any updates!

### First Startup

Post Downloading the binary CD to your downloads directory and just type `user_agent` in the terminal to get the user agents displayed in the terminal. And if you have installed `user_agent` via `cargo install user_agent` just open the terminal / cmd / powershell / iterm and type `user_agent` and press enter from any directory of your likings.

```bash
╭─ ~/home/bin                                                                                                                                                   3.10.4   ✔  18:17:14  ─╮
╰─ user_agent
                                                                                                                                                                              ─╯
╭─ The Latest User Agents Are As Followings ─╮
╰─                                            ─╯

╭─ Windows User Agents ─╮
╰─[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Edg/100.0.1185.39",
    "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Vivaldi/4.3",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Vivaldi/4.3",
]

╭─ MacOs User Agents ─╮
╰─[
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12.4; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Vivaldi/4.3",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Edg/100.0.1185.39",
]

╭─ Microsoft Edge User Agents ─╮
╰─[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Edg/100.0.1185.39",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Edg/100.0.1185.39",
    "Mozilla/5.0 (Linux; Android 10; HD1913) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36 EdgA/100.0.1185.50",
    "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36 EdgA/100.0.1185.50",
    "Mozilla/5.0 (Linux; Android 10; Pixel 3 XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36 EdgA/100.0.1185.50",
    "Mozilla/5.0 (Linux; Android 10; ONEPLUS A6003) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36 EdgA/100.0.1185.50",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 EdgiOS/100.1185.50 Mobile/15E148 Safari/605.1.15",
    "Mozilla/5.0 (Windows Mobile 10; Android 10.0; Microsoft; Lumia 950XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Mobile Safari/537.36 Edge/40.15254.603",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox One) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Edge/44.18363.8131",
]

╭─ Google Chrome User Agents ─╮
╰─[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/101.0.4951.58 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/101.0.4951.58 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPod; CPU iPhone OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/101.0.4951.58 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; SM-A205U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; SM-A102U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; SM-G960U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; SM-N960U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; LM-Q720) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; LM-X420) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; LM-Q710(FGN)) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.61 Mobile Safari/537.36",
]

╭─ Mozilla Firefox User Agents ─╮
╰─[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12.4; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (X11; Linux i686; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 12_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/100.0 Mobile/15E148 Safari/605.1.15",
    "Mozilla/5.0 (iPad; CPU OS 12_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/100.0 Mobile/15E148 Safari/605.1.15",
    "Mozilla/5.0 (iPod touch; CPU iPhone OS 12_4 like Mac OS X) AppleWebKit/604.5.6 (KHTML, like Gecko) FxiOS/100.0 Mobile/15E148 Safari/605.1.15",
    "Mozilla/5.0 (Android 12; Mobile; rv:68.0) Gecko/68.0 Firefox/100.0",
    "Mozilla/5.0 (Android 12; Mobile; LG-M255; rv:100.0) Gecko/100.0 Firefox/100.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12.4; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (X11; Linux i686; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0",
]

╭─ Apple Safari User Agents ─╮
╰─[
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPod touch; CPU iPhone 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Mobile/15E148 Safari/604.1",
]

```

#### Configure

_To Be Updated_

#### Manage

_To Be Updated_

#### Delete

_To Be Updated_

## Feature Requests

If you would like to make a feature request, check the [issues](https://github.com/TechfaneTechnologies/user_agent/issues?q=is%3Aissue) to see if the feature has already been added or is planned. If not, [create a new issue](https://github.com/TechfaneTechnologies/user_agent/issues/new).

## Building from Source or Working with user_agent

Firstly, you need the Rust toolchain which includes `cargo`, `rustup`, etc. You can install these from [the Rust website](https://www.rust-lang.org/tools/install).
You'll also need the [Just](https://github.com/casey/just#installation) command runner, its basically a much better version of `make`.

If you want to build user_agent without cloning the repo then run `cargo install user_agent`.

To build the project and install it to your Cargo binary directory, clone the project and run `just install`. If you want to install it for testing a developement version, run `just` (alias for `just install-dev`).

If you want to obtain executables for a specific OS, you can run `just build-<OS>` and replace `<OS>` with `mac`, `win`, or `linux`. The produced binaries will be zipped and moved to `out/`.

You can run clippy linters using `just lint`, and integration tests using `cargo test`. Finally you can delete all the build and test artefacts by using `just clean`.

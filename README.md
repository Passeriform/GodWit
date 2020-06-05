# GodWit

[![crate](https://img.shields.io/crates/v/godwit)](https://crates.io/crates/godwit)
[![docs](https://docs.rs/godwit/badge.svg)](https://docs.rs/godwit)
[![build](https://travis-ci.org/Passeriform/GodWit.svg?branch=master)](https://travis-ci.org/Passeriform/GodWit)
[![codecov](https://codecov.io/gh/Passeriform/GodWit/branch/master/graph/badge.svg)](https://codecov.io/gh/Passeriform/GodWit)
[![maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/Passeriform/GodWit/graphs/commit-activity)

[![homepage](https://img.shields.io/website-up-down-green-red/http/passeriform.com.svg?label=Passeriform)](http://www.passeriform.com/prod/GodWit)
[![repo](https://img.shields.io/badge/github-GodWit-blue?logo=github)](https://github.com/Passeriform/GodWit)

A hackable yet sane project manager and automation suite. GodWit follows an unintrusive philosophy with maximal exposure points to get the best of both end-user and developer worlds.

Godwit is a config driven project assets and states manager suite. It supports a pluggable
environment to work with multiple plugins. Godwit also features diff-snipping to
trim out unwanted operations from track history. This command is a cli front to
godwit core, daemon and other plugins.

It tracks changes in applications across the system deemed crucial for project
development. The application can be used as knowledge compilation tool and can be
used to provide meaningful analytics on projects.

Godwit uses terms like state to represent a container that can either
contain a project and its details or another such container. The complete list of
all such containers is represented using a state-graph.

## Install
[![release](https://img.shields.io/github/release/Passeriform/GodWit.svg)](https://github.com/Passeriform/GodWit/releases/)
[![aur](https://img.shields.io/aur/version/godwit)](https://aur.archlinux.org/packages/godwit)
[![debian](https://img.shields.io/debian/v/godwit)](https://packages.debian.org/unstable/utils/godwit)
[![homebrew](https://img.shields.io/homebrew/v/godwit)](https://formulae.brew.sh/formula/godwit)

GodWit is available on AUR and PPA repositories.

Simply install it using
```bash
$ yay -Syu godwit
```
OR
```bash
$ sudo add-apt-repository ppa:passeriform/ppa
$ sudo apt update
$ sudo apt install godwit
```

## Use
Begin with initializing GodWit on your general working directory.

```bash
$ godwit init
```

Try adding a new project.

```bash
$ godwit add @organization/project ~/projects/project
```

GodWit automatically tracks the project for any development and gives a rundown of the info after tracking.

```bash
$ godwit status -v
```

## Contributing
[![issues](https://img.shields.io/github/issues/Passeriform/GodWit.svg)](https://gitHub.com/Passeriform/GodWit/issues/)
[![pull-requests](https://img.shields.io/github/issues-pr/Passeriform/GodWit)](https://github.com/Passeriform/GodWit/pulls)

If you want to contribute, start by cloning this repository.
```bash
    git clone https://github.com/Passeriform/GodWit Godwit
```
Checkout to a new branch.
```bash
    # Use kebab-case categorization format.


    # Ex: A new feature.
    git checkout feature-<feature>

    # Ex: A bugfix.
    git checkout bugfix-<bug>

    # Ex: A meta update.
    git checkout meta-<title>

    # Ex: A documentation update.
    git checkout docs-<title>

    # Ex: A CI update.
    git checkout ci-<title>

```
Do your thing...

```bash
Code up
```
**Squash commits** and issue a PR at
[https://github.com/Passeriform/GodWit](https://github.com/Passeriform/GodWit)

## License
![license](https://img.shields.io/crates/l/godwit-daemon)

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Credition

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

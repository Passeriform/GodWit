# GodWit
A hackable yet sane project manager and automation suite. GodWit follows an unintrusive philosophy with maximal exposure points to get the best of both end-user and developer worlds.

## Install
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

## Packages
  - GodWit-Daemon - A dedicated runner for handling processes. [https://github.com/Passeriform/GodWit-Daemon](https://github.com/Passeriform/GodWit-Daemon)

## Plugins

- Weaver - A templated boilerplate builder. [https://github.com/Passeriform/Weaver](https://github.com/Passeriform/Weaver)

## Development
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

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

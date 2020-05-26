= godwit(1)

== NAME

godwit - a hackable yet sane project manager and automation suite.


== SYNOPSIS

*godwit* [_FLAGS_] [_OPERATION_] [_OPTIONS_]

*godwit* [_FLAGS_] [_OPTIONS_] --help

*godwit* [_FLAGS_] [_OPTIONS_] --version


== DESCRIPTION

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

Note: It needs an *init* operation run to setup the working directory. The directory
of choice is *~/.godwit*. Custom paths can be provided by appending a path after init
command.

For more information visit https://github.com/Passeriform/GodWit/README.md

_Tip:_ Godwit will try to include any project with a *.gw* directory in its root, present
in the working directory. It is advised to commit the whole *.gw* directory as the
states are stored and used from *~/.godwit* or the specified working directory.


== EXIT STATUS

* `0` exit status occurs when a command works as expected (even in case of errors), unless *--quiet* was given.
* `1` exit status occurs when a catastrophic failure has occured. This must not
happen in regular cases. You can file a bug if its a reproducible issue https://github.com/Passeriform/GodWit/issues/new.


== HEADLESS INSTALLATION

Godwit supports a headless installation without the support of states. This type
of installation comes in use while working with plugins requiring psuedo-environment
for operation. This installation must be used when working with services like CI
and Docker.

*godwit* [_OPTIONS_] init --headless


== CONFIGURATION FILES

Godwit reserves the directory *~/.godwit* for functioning. Within the directory,
the following files may exist:

    1.  ~/.godwit/*settings.gwcore* - Settings file containing godwit working settings.
    2.  ~/.godwit/states/*state-graph-tag.gwsg* - State-file containing initial tracker data.

In case of a headless installation, a working directory isn't used. Instead the following work:

    1.  *~/.gwrc* - Settings file identical to *settings.gwcore* bar the state-related
    tags, which will be ignored.


_Note:_

    1.  No state management occurs in headless usage. All states are lost upon completion of current command.
    2.  If multiple states exist, the state-file is picked lexicographically.
    3.  Order of precedence for config files is: *.gw* > *.gwrc* > *settings.gwcore*


== SHELL COMPLETION

Shell completion files are included in the release tarball for Bash, Fish, Zsh
and PowerShell.

For *bash*, move *godwit.bash* to *$HOME/bash_completion*
or */etc/bash_completion.d/*.

For *fish*, move *godwit.fish* to *$HOME/.config/fish/completions*.

For *zsh*, move *godwit.zsh* to one of your *$fpath* directories.


== VERSION

{VERSION}


== HOMEPAGE

https://github.com/Passeriform/GodWit

Please report bugs and feature requests in the issue tracker. Please do your
best to provide a reproducible test case for bugs. Include both the output of
*godwit status -vvv* and the command you ran with a *-vvv* flag.


== AUTHORS

Utkarsh Bhardwaj (Passeriform) <passeriform.ub@gmail.com>

===============================================
clapcomp - clap completion generator as command
===============================================

The CLI arguments parsing tool - `clap <https://github.com/kbknapp/clap-rs>`_
- has a feature that can generate shell completions script,
this project expose this ability !

Now, you can generate shell completion by a single command !

Even **non-Rust projects** can benfit from clap's completion generator,
just provide your arguments config !


.. contents:: Table of Contents



Installation
========================================

From `crate.io <https://crates.io/>`_

.. code-block:: sh

    $ cargo install clapcomp


From GitHub

.. code-block:: sh

    $ cargo install --git https://github.com/wdv4758h/clapcomp/


Download Prebuilt Binary

.. code-block:: sh

    # by curl
    $ curl -O -J -L https://github.com/wdv4758h/clapcomp/releases/download/v0.1.1/clapcomp-v0.1.1-x86_64-unknown-linux-gnu.tar.gz

    # by wget
    $ wget https://github.com/wdv4758h/clapcomp/releases/download/v0.1.1/clapcomp-v0.1.1-x86_64-unknown-linux-gnu.tar.gz



Usage
========================================

.. code-block:: sh

    $ clapcomp --help
    clapcomp 0.1.1
    Chiu-Hsiang Hsu <wdv4758h@gmail.com>
    clap completion generator as command

    USAGE:
        clapcomp [OPTIONS] <input>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -f, --format <format>    input format [default: yaml]  [values: yaml]
        -o, --output <output>    output directory
        -s, --shell <shell>      target shell for completion [default: bash]  [values: bash,
                                 fish]

    ARGS:
        <input>    input file


.. code-block:: sh

    # src/arguments.yml is this project's setting
    $ clapcomp --shell bash src/arguments.yml
    $ cat clapcomp_bash.sh
    _clapcomp() {
        local i cur prev opts cmds
        COMPREPLY=()
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
        cmd=""
        opts=""

        for i in ${COMP_WORDS[@]}
        do
            case "${i}" in
                clapcomp)
                    cmd="clapcomp"
                    ;;

                clapcomp)
                    cmd+="_clapcomp"
                    ;;
                *)
                    ;;
            esac
        done

        case "${cmd}" in
            clapcomp)
                opts=" -f -s -o -h -V  --format --shell --output --help --version  <input> "
                if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                    return 0
                fi
                case "${prev}" in

                    --format)
                        COMPREPLY=($(compgen -W "yaml" -- ${cur}))
                        return 0
                        ;;
                        -f)
                        COMPREPLY=($(compgen -W "yaml" -- ${cur}))
                        return 0
                        ;;
                    --shell)
                        COMPREPLY=($(compgen -W "bash fish" -- ${cur}))
                        return 0
                        ;;
                        -s)
                        COMPREPLY=($(compgen -W "bash fish" -- ${cur}))
                        return 0
                        ;;
                    --output)
                        COMPREPLY=("<output>")
                        return 0
                        ;;
                        -o)
                        COMPREPLY=("<output>")
                        return 0
                        ;;
                    *)
                        COMPREPLY=()
                        ;;
                esac
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
                ;;

        esac
    }

    complete -F _clapcomp clapcomp


.. code-block:: sh

    # src/arguments.yml is this project's setting
    $ clapcomp --shell fish src/arguments.yml
    $ cat clapcomp.fish
    complete -c clapcomp -s f -l format -d 'input format' -r -f -a 'yaml'
    complete -c clapcomp -s s -l shell -d 'target shell for completion' -r -f -a 'bash fish'
    complete -c clapcomp -s o -l output -d 'output directory'
    complete -c clapcomp -s h -l help -d 'Prints help information'
    complete -c clapcomp -s V -l version -d 'Prints version information'


Information About Binary
========================================

Size
------------------------------

x86_64, Linux (build on Arch Linux)

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.1  | No         | 1453952      | 1.4M      |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.1  | Yes        | 1089672      | 1.1M      |
+----------+---------+------------+--------------+-----------+


x86_64, Linux, musl (build on Arch Linux)

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.1  | No         | 1428512      | 1.4M      |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.1  | Yes        | 1136928      | 1.1M      |
+----------+---------+------------+--------------+-----------+


Shared Library Dependency
------------------------------

x86_64, Linux (build on Arch Linux)

.. code-block:: sh

    $ ldd ./target/release/clapcomp
            linux-vdso.so.1 (0x00007ffd8d5d1000)
            libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007f8019d89000)
            libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f8019b73000)
            libc.so.6 => /usr/lib/libc.so.6 (0x00007f80197d2000)
            /lib64/ld-linux-x86-64.so.2 (0x00007f8019fa6000)


x86_64, Linux, musl (build on Arch Linux)

.. code-block:: sh

    $ ldd ./target/x86_64-unknown-linux-musl/release/clapcomp
            not a dynamic executable



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------

* support `zsh <http://www.zsh.org/>`_
* support generate from raw help message


v0.1.1 (2016-07-30)
------------------------------

Features
++++++++++++++++++++

* support `fish <https://fishshell.com/>`_ shell completion


v0.1.0 (2016-07-14)
------------------------------

Features
++++++++++++++++++++

* support `bash <https://www.gnu.org/software/bash/>`_ completion



Notice
========================================

I've only tested on my x86_64 Linux.
Other platforms are built by CI.
If they don't work properly, please tell me.


Developement
========================================

Making Release
------------------------------

1. update version in ``src/arguments.yml``
2. update version in ``Cargo.toml``
3. update version in ``Cargo.lock``
4. add git tag



Special Thanks
========================================

* `rust-everywhere <https://github.com/japaric/rust-everywhere/>`_ for CI integration
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

clapcomp is licensed under the MIT License (same as ``clap``) - see the ``LICENSE`` file for details

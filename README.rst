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
    $ curl -O -J -L https://github.com/wdv4758h/clapcomp/releases/download/v0.1.4/clapcomp-v0.1.4-x86_64-unknown-linux-gnu.tar.gz

    # by wget
    $ wget https://github.com/wdv4758h/clapcomp/releases/download/v0.1.4/clapcomp-v0.1.4-x86_64-unknown-linux-gnu.tar.gz



Usage
========================================

.. code-block:: sh

    $ clapcomp --help
    clapcomp 0.1.4
    clap completion generator as command

    USAGE:
        clapcomp [FLAGS] [OPTIONS] <input>

    FLAGS:
        -h, --help       Prints help information
            --stdout     output result to STDOUT
        -V, --version    Prints version information

    OPTIONS:
        -f, --format <format>      input format [default: yaml]  [values: yaml]
        -d, --outdir <outdir>      output directory
        -o, --outfile <outfile>    output file
        -s, --shell <shell>        target shell for completion [default: bash]  [values: bash,
                                   fish, zsh, powershell]

    ARGS:
        <input>    input file


.. code-block:: sh

    # src/cli.yml.yml is this project's setting
    $ clapcomp src/cli.yml.yml --shell bash
    $ cat clapcomp.bash-completion
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
                opts=" -f -s -d -o -h -V  --format --shell --outdir --outfile --stdout --help --version  <input> "
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
                    --outdir)
                        COMPREPLY=("<outdir>")
                        return 0
                        ;;
                        -d)
                        COMPREPLY=("<outdir>")
                        return 0
                        ;;
                    --outfile)
                        COMPREPLY=("<outfile>")
                        return 0
                        ;;
                        -o)
                        COMPREPLY=("<outfile>")
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

    # src/cli.yml.yml is this project's setting
    $ clapcomp src/cli.yml.yml --shell fish
    $ cat clapcomp.fish
    function __fish_using_command
        set cmd (commandline -opc)
        if [ (count $cmd) -eq (count $argv) ]
            for i in (seq (count $argv))
                if [ $cmd[$i] != $argv[$i] ]
                    return 1
                end
            end
            return 0
        end
        return 1
    end

    complete -c clapcomp -n "__fish_using_command clapcomp" -s f -l format -d "input format" -r -f -a "yaml"
    complete -c clapcomp -n "__fish_using_command clapcomp" -s s -l shell -d "target shell for completion" -r -f -a "bash fish"
    complete -c clapcomp -n "__fish_using_command clapcomp" -s d -l outdir -d "output directory"
    complete -c clapcomp -n "__fish_using_command clapcomp" -s o -l outfile -d "output file"
    complete -c clapcomp -n "__fish_using_command clapcomp" -l stdout -d "output result to STDOUT"
    complete -c clapcomp -n "__fish_using_command clapcomp" -s h -l help -d "Prints help information"
    complete -c clapcomp -n "__fish_using_command clapcomp" -s V -l version -d "Prints version information"


.. code-block:: sh

    $ clapcomp src/cli.yml.yml --shell bash fish zsh powershell



Information About Binary
========================================

Size
------------------------------

x86_64, Linux (build on Arch Linux)

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.4  | No         | 1999264      | 2.0M      |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.4  | Yes        | 1166952      | 1.2M      |
+----------+---------+------------+--------------+-----------+


x86_64, Linux, musl (build on Arch Linux)

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.4  | No         | 2361584      | 2.3M      |
+----------+---------+------------+--------------+-----------+
| clapcomp | v0.1.4  | Yes        | 1259592      | 1.3M      |
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

* support generate from raw help message


v0.1.4 (2016-12-10)
------------------------------

Features
++++++++++++++++++++

* support `zsh <http://www.zsh.org/>`_
* support powershell


v0.1.3 (2016-10-29)
------------------------------

Features
++++++++++++++++++++

* new fish completion


v0.1.2 (2016-08-03)
------------------------------

Features
++++++++++++++++++++

* support output to STDOUT and specific file
* support output multiple completion files at once


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

1. update version in ``src/cli.yml``
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

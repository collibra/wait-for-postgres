wait-for-postgres
=================

Statically-linked binary that will wait for a PostgreSQL server to be up and
running before exiting. Use the GitHub's release to easily include it to any of
your Docker images.

Installation
------------

```
ADD ... /wait-for-postgres

...

CMD /wait-for-postgres -h database -- bash -c "Hello World!"
```

Usage
-----

```
USAGE:
    wait-for-postgres [OPTIONS] --host <host> [command]...

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>            Specifies the host name of the machine on which the server is running. If the value
                                 begins with a slash, it is used as the directory for the Unix-domain socket.
    -i, --interval <interval>    Specifies the interval in milliseconds between each call. Defaults to 3000.
    -p, --port <port>            Specifies the TCP port or the local Unix-domain socket file extension on which the
                                 server is listening for connections. Defaults to 5432.

ARGS:
    <command>...    Command and arguments to execute (using execvp) at the end
```

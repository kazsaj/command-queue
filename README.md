# Command Queue

Command Queue is a simple queue processor which will execute any (shell) commands passed to the queue.

**It executes anything? But, but what about security?**

A bad actor would need access to your Redis instance, from which the command-queue pulls from. If that happens you're most likely more screwed than you think.

**B-but...**

If it's a no-no for you, here are some of the options you have:
- Use something else,
- PR or fork this project to use a different processor for the data pulled in from the redis,
- sandbox the environment the command-queue runs in (don't run it as root, limit network access, stuff it in a dedicated VM or docker container).

## Features
- Pull commands from Redis lists and execute them (FIFO) in the same context as this binary runs
- Uses multiple threads to pull from lists and can run multiple threads for the same list as well
- If a thread does not have anything to process, it will pull from the other lists (if multiple lists were set up), to avoid them sitting idle
- On execution failure run `n` times (default: 3) before giving up on the failing task and moving it to an "error" list
- Wait `m` seconds (default: 31) between each retry attempt

## Environment variables
- `COMMAND_QUEUE_INSTANCE_NAME` - how should the instance identify itself when needed, if `COMMAND_QUEUE_INSTANCE_NAME` is empty it will try to use `HOSTNAME` environment variable, and if that fails it will use `instance-NUMBER`, where number is the number of seconds since the Unix epoch during the moment of startup.
- `COMMAND_QUEUE_REDIS_HOSTNAME` - hostname of the redis instance to connect to (default: 127.0.0.1)
- `COMMAND_QUEUE_REDIS_PORT` - port of the redis instance (default: 6379)
- `COMMAND_QUEUE_REDIS_POP_TIMEOUT` - how long (in seconds) should it wait during each redis blocking pop command (default: 3)
- `COMMAND_QUEUE_RETRY_SLEEP` - how long (in seconds) should it wait between each retry attempt to process a command (default: 31)
- `COMMAND_QUEUE_RETRY_LIMIT` - how many times should it retry to process a command (failure = returning non-zero response). This number represents any additional attempts running the command, besides the original one (default: 2)
- `COMMAND_QUEUE_LOG_LEVEL` - what level of log messages should be shown, supported levels: `ERROR`, `WARNING`, `INFO`, `DEBUG` (default `INFO`)

## Arguments

```bash
command-queue QUEUE_NAME [QUEUE_NAME...]
```

e.g.

```bash
command-queue alfa bravo charlie
```

You can pass the same queue name multiple times, to have multiple threads pull from the same queue.

```bash
command-queue alfa alfa bravo charlie
```

## Repository build requirements
- [Rust](https://www.rust-lang.org/tools/install) (1.30 or later) and Cargo
- Docker and Docker-compose - which will bring up a local redis instance, useful for testing
- Make - if you want to use the Makefile commands

## Useful redis commands
- `MONITOR` - watch for any commands executed on the server, useful for debugging
- `RPUSH key value [value]` - add element to the list
    ```
    rpush alfa_priority "echo hello" "ls -l" "false"
    ```
- `BLPOP key` - remove first element from a list, or block until you find one

## TODO
- diagram explaining how the queue works
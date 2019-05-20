# Command Queue

Command Queue is a simple queue processor which will execute any (shell) commands passed to the queue.

**It executes anything? But, but what about security?**

A bad actor would need access to your Redis instance, from which the command-queue pulls from. If that happens you're most likely more screwed than you think.

## Repository requirements
- [Rust](https://www.rust-lang.org/tools/install) (1.30 or later) and Cargo
- Docker and Docker-compose - if you make it easier to locally test
- Make - if you want to use the Makefile

## Environment variables
- `COMMAND_QUEUE_REDIS_HOSTNAME` - to what hostname should it connect to
- `COMMAND_QUEUE_REDIS_PORT` - what port should it use when connecting
- `COMMAND_QUEUE_REDIS_POP_TIMEOUT` - how long should it wait between each redis blocking pop commands
- `COMMAND_QUEUE_RETRY_SLEEP` - how long should it wait between each attempt to process a command
- `COMMAND_QUEUE_RETRY_LIMIT` - how many times should it retry to process a command (failure = returning non-zero response)

## Arguments

```bash
command-queue queue_name [QUEUE_NAME...]
```

e.g.

```bash
command-queue alfa bravo charlie
```

You can pass the same queue name multiple times, to have multiple threads pull from the same queue.

```bash
command-queue alfa alfa bravo charlie
```

## Useful redis commands
- `MONITOR` - watch for any commands executed on the server, useful for debugging
- `RPUSH key value [value]` - add element to the list
    ```
    rpush alfa_priority "echo hello" "ls -l" "false"
    ```
- `BLPOP key` - remove first element from a list, or block until you find one

## TODO
- (OK) connect to redis
- (OK) connect to redis in each child thread
- (OK) get one element from work list
- (OK) handle high priority and default priority queue
- (OK) use a different queue name for each thread
- (OK) use the remaining queues if the main one for the thread does not have anything to process
- (OK) create the number of threads and queue configs based on the arguments passed  
- (OK) get redis connection details from env variables
- (OK) handle redis connection errors
- (OK) handle sigterm
- (OK) execute individual jobs
- (OK) logging to output via a formatter somewhat matches nginx
- (OK) add failed entries to a list (with datetime)
- (OK) fix issues with running a single thread only
- (OK) prepare a makefile
- (OK) makefile - build release for alpine
- diagram explaining how the queue works
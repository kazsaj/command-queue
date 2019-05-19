# Command Queue

Command Queue is a simple queue processor which will execute any (shell) commands passed to the queue.

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
command-queue alpha bravo charlie
```

You can pass the same queue name multiple times, to have multiple threads pull from the same queue.


```bash
command-queue alpha alpha bravo charlie
```

## Useful redis commands
- `MONITOR`
- `RPUSH key value [value]` - add element to the list
    ```
    rpush queue 1333 888333 11223 88833 272 4891 7219 489 71 1 2 3 4 5 6 7 8 9 10 11 12 13
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
- fix issues with running a single thread only
- add failed entries to a list (with datetime)
- prepare a makefile
    - build
    - build release (for alpine)
    - test
    - run
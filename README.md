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
- execute individual jobs
- prepare a makefile
    - build
    - build release (for alpine)
    - test
    - run
- add failed entries to a list (with datetime)
- (OK) logging to output via a formatter somewhat matches nginx

## Useful redis commands
- `MONITOR`
- `RPUSH key value [value]` - add element to the list
    ```
    rpush queue 1333 888333 11223 88833 272 4891 7219 489 71 1 2 3 4 5 6 7 8 9 10 11 12 13
    ```
- `BLPOP key` - remove first element from a list, or block until you find one
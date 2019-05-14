## TODO
- (OK) connect to redis
- (OK) connect to redis in each child thread
- read connection details from some configuration file
- (OK) get one element from work list
- decode JSON element from a work list

## Useful redis commands
- `MONITOR`
- `RPUSH key value [value]` - add element to the list
    ```
    rpush queue 1333 888333 11223 88833 272 4891 7219 489 71 1 2 3 4 5 6 7 8 9 10 11 12 13
    ```
- `BLPOP key` - remove first element from a list, or block until you find one
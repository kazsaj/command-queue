# Change Log

## 0.5.0
- three levels of priority now available _high, _default and _low instead of _priority and _default

## 0.4.0
- Added support for `COMMAND_QUEUE_INSTANCE_NAME` environment variable stored in `EnvConfig.instance_name`
- Letting the sub-threads gracefully join the main thread on shutdown
- Re-using connection to Redis for a single command cycle
- Saving last executed command in Redis under an expiring key with unique name per thread
- Fixing delay before the last command execution retry attempt

## 0.3.0
- Added support for `COMMAND_QUEUE_LOG_LEVEL` environment variable
- Refactored to use logger objects instead of calling static output methods
- Display more details about the command-queue instance on startup
- Display retry counter when processing messages

## 0.2.0
- Added debug message before executing a pulled command

## 0.1.1
Correcting COMMAND_QUEUE_RETRY_LIMIT behaviour.
- Setting it to `0` now ensures there are no re-try attempts (previously 1 would ensure this behaviour).
- Changing default retry limit to `2`.

## 0.1.0
- First release to tag the project in a working and tested state.

---

See full history: https://github.com/kazsaj/command-queue/releases
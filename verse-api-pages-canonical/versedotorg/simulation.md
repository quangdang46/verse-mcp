## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation

# Simulation module
Learn technical details about the Simulation module.
  * [`Verse.org`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg)
  * **`Simulation`**
    * [`Tags`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/tags)

## Classes and Structs
Name | Description
---|---
[`session`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/session) |  Type for which there is a single instance per round. Use `GetSession` to get the current round's `session` instance. May be used with `weak_map` to implement global variables. Note: may be changed in a future release to a single instance per game. Round-local behavior should not be relied upon.
[`team`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/team) |
[`agent`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/agent) |
[`player`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/player) |
## Functions
Name | Description
---|---
[`editable_slider`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/editable_slider) |
[`editable_number`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/editable_number) |
[`editable_vector_slider`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/editable_vector_slider) |
[`editable_vector_number`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/editable_vector_number) |
[`GetSession`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/getsession) |  Returns the `session` corresponding to the current round. The result can be used with `weak_map` to implement global variables. Note: may be changed in a future release to return a single instance per game. Round-local behavior should not be relied upon.
[`Sleep`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/sleep) |  Waits specified number of seconds and then resumes. If `Seconds` = 0.0 then it waits until next tick/frame/update. If `Seconds` = Inf then it waits forever and only calls back if canceled - such as via `race`. If `Seconds` < 0.0 then it completes immediately and does not yield to other aysnc expressions. Waiting until the next update (0.0) is especially useful in a loop of a coroutine that needs to do some work every update and this yields to other coroutines so that it doesn't hog a processor's resources. Waiting forever (Inf) will have any expression that follows never be evaluated. Occasionally it is desireable to have a task never complete such as the last expression in a `race` subtask where the task must never win the race though it still may be canceled earlier. Immediately completing (less than 0) is useful when you want programmatic control over whether an expression yields or not.
[`GetSimulationElapsedTime`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/getsimulationelapsedtime) |  Get the seconds that have elapsed since the world began simulating
## Enumerations
Name | Description
---|---
[`session_environment`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/simulation/session_environment) |  Specifies what type of environment the current session is in.

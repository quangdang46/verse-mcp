## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/concurrency/awaitable/awaitable(payload)

# awaitable(payload) interface
Learn technical details about the awaitable(payload) interface.
A parametric interface implemented by events with a `payload` that can be waited on. Matched with `signalable.`
|
---|---
Verse `using` statement | `using { /Verse.org/Concurrency }`
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description
---|---
[`Await`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/concurrency/awaitable/awaitable\(payload\)/await) |  Suspends the current task until resumed by a matching call to `signalable.Signal`. Returns the event `payload`.

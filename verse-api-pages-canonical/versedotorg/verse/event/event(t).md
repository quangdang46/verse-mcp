## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event/event(t)

# event(t) class
Learn technical details about the event(t) class.
A _recurring_ , successively signaled parametric `event` with a `payload` allowing a simple mechanism to coordinate between concurrent tasks.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
## Exposed Interfaces
This class exposes the following interfaces:
Name | Description
---|---
[`signalable(payload)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/signalable/signalable\(payload\)) |  A parametric interface implemented by events with a `payload` that can be signaled. Can be used with `awaitable`, `subscribable`, or both (see: `listenable`).
[`awaitable(payload)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/concurrency/awaitable/awaitable\(payload\)) |  A parametric interface implemented by events with a `payload` that can be waited on. Matched with `signalable.`
## Members
This class has functions, but no data members.
### Functions
Function Name | Description
---|---
[`Await`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event/event\(t\)/await) |  Suspends the current task until another task calls `Signal`. If called during another invocation of `Signal`, the the task will still suspend and resume during the next call to `Signal`.
[`Signal`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event/event\(t\)/signal) |  Concurrently resumes the tasks that were suspended by `Await` calls before this call to `Signal`. Tasks are resumed in the order they were suspended. Each task will perform as much work as it can until it encounters a blocking call, whereupon it will transfer control to the next suspended task.

## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/subscribable/subscribable(t)

# subscribable(t) interface
Learn technical details about the subscribable(t) interface.
A parametric interface implemented by events with a `payload` that can be subscribed to. Matched with `signalable.`
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description
---|---
[`Subscribe`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/subscribable/subscribable\(t\)/subscribe) |  Registers `Callback` to be invoked on matching calls to `signable.Signal`. Returns an unsubscriber object. Call `cancelable.Cancel` on the unsubscriber to unregister `Callback`.

## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable

# cancelable interface
Learn technical details about the cancelable interface.
Implemented by classes that allow users to cancel an operation. For example, calling `subscribable.Subscribe` with a callback returns a `cancelable` object. Calling `Cancel` on the return object unsubscribes the callback.
|
---|---
Verse `using` statement | `using { /Verse.org/Verse }`
## Members
This interface has functions, but no data members.
### Functions
Function Name | Description
---|---
[`Cancel`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable/cancel) |  Prevents any current or future work from completing.

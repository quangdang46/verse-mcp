## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device

# item_granter_device class
Learn technical details about the item_granter_device class.
Used to grant items to `agent`s. Items can either be dropped at the `agent`'s location or added directly to their inventory.
|
---|---
Verse `using` statement | `using { /Fortnite.com/Devices }`
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `creative_object`:
Name | Description
---|---
[`creative_object`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object) |  Base class for creative devices and props.
[`creative_device_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_device_base) |  Base class for creative_device.
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description
---|---|---
`DefaultItemCount` | `?int` |  Gets or sets the default number of items that will be awarded for all items in the Item Granter that have not been overwritten. `Count` must be greater than 0.
`GrantItemWithCountEvent` | `listenable(payload)` |  Signaled when an item is granted to an `agent`. Sends the `agent` that was granted the item, as well as the number of items granted.
`ItemGrantedEvent` | `listenable(payload)` |  Signaled when an item is granted to an `agent`. Sends the `agent` that was granted the item.
### Functions
Function Name | Description
---|---
[`ClearSaveData`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/clearsavedata) |  Clears saved data for `Agent`, preventing them from receiving items while offline. This only works when _Grant While Offline_ is set to _Yes_.
[`CycleToNextItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletonextitem) |  Cycles to the next item. If _Grant on Cycle_ is set `Agent` will be granted the item.
[`CycleToPreviousItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletopreviousitem) |  Cycles to the previous item. If _Grant on Cycle_ is set `Agent` will be granted the item.
[`CycleToRandomItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/cycletorandomitem) |  Cycles to a random item. If _Grant on Cycle_ is set `Agent` will be granted the item.
[`Disable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/disable) |  Disables this device.
[`Enable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/enable) |  Enables this device.
[`GetItemGrantCountAtIndex`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/getitemgrantcountatindex) |  Returns the number of items this Item Granter will award for the item at the specified `Index`. This will return 0 if `Index` is invalid. If _Cycle Behavior_ is _Stop_ , `Index` is clamped to the number of items in the Item Granter. If _Cycle Behavior_ is _Wrap_ , `Index` is modulo'd to the number of items in the Item Granter.
[`GetItemIndex`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/getitemindex) |  Returns the current Item Index that this device will grant when activated.
[`GetTransform`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/gettransform) |  Returns the transform of the `creative_object` with units in cm. You must check `creative_object.IsValid` before calling this if there is a possibility the object has been disposed or destroyed by gameplay. Otherwise a runtime error will result.
[`GrantItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/grantitem) |  Grants an item to `Agent`.
[`GrantItemIndex`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/grantitemindex) |  Grants an item at a specific `ItemIndex` to an `Agent`. `Index` should be between `0` and the available item count - 1. If Value is out of bounds, which item is granted is determined by _Cycle Behavior_.
[`GrantItemIndex`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/grantitemindex-1) |  Grants an item at a specific `ItemIndex` to all players. Only functions when _Receiving Players_ is set to _All_ or _Team Index_. `Index` should be between `0` and the available item count - 1. If Value is out of bounds, which item is granted is determined by _Cycle Behavior_.
[`GrantItemToAll`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/grantitemtoall) |  Grants an item without requiring an agent reference. This only works when _Receiving Players_ is set to _All_ or _Team Index_.
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto) |  Moves the `creative_object` to the specified `Position` and `Rotation` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.
[`MoveTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/moveto-1) |  Moves the `creative_object` to the specified `Transform` over the specified time, in seconds. If an animation is currently playing on the `creative_object` it will be stopped and put into the `AnimationNotSet` state.
[`RestockItems`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/restockitems) |  Restocks this device back to its starting inventory count.
[`SetItemGrantCountAtIndex`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/setitemgrantcountatindex) |  Sets the number of items this Item Granter will award for the item at the specified `ItemIndex`. `Count` must be greater than 0. If _Cycle Behavior_ is _Stop_ , `ItemIndex` is clamped to the number of items in the Item Granter. If _Cycle Behavior_ is _Wrap_ , `ItemIndex` is modulo'd to the number of items in the Item Granter.
[`SetNextItem`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/item_granter_device/setnextitem) |  Sets the next item to be granted.
  * `Index` should be between `0` and the available item count - 1.
  * Calling `SetNextItem` with an invalid index will do nothing.

[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto) |  Teleports the `creative_object` to the specified `Position` and `Rotation`.
[`TeleportTo`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_object/teleportto-1) |  Teleports the `creative_object` to the specified location defined by `Transform`, also applies rotation and scale accordingly.

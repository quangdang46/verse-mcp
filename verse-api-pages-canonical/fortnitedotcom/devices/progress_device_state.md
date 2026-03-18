## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/progress_device_state

# progress_device_state enumeration
Learn technical details about the progress_device_state enumeration.
The state of a progress_based_mesh_device.
|
---|---
Verse `using` statement | `using { /Fortnite.com/Devices }`
## Enumerators
The `progress_device_state` enumeration includes the following enumerators:
Name | Description
---|---
`Progress` |  This device is currently progressing, and will increase its 'CurrentProgress' by its 'ProgressRate'.
`Regress` |  This device is currently regressing, and will decrease its 'CurrentProgress' by its 'RegressRate'.
`Pause` |  This device is currently paused. It will not progress or regress automatically.

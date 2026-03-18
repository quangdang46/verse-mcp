## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/spawn_prop_result

# spawn_prop_result enumeration
Learn technical details about the spawn_prop_result enumeration.
Results for `SpawnProp`.
|
---|---
Verse `using` statement | `using { /Fortnite.com/Devices }`
## Enumerators
The `spawn_prop_result` enumeration includes the following enumerators:
Name | Description
---|---
`Ok` |  Success.
`UnknownError` |  An unknown error occurred. If this happens frequently, contact Epic support.
`InvalidSpawnPoint` |  The spawn point contains NaN or Inf.
`SpawnPointOutOfBounds` |  The spawn point is outside the island's boundaries.
`InvalidAsset` |  The asset is not a valid `creative_prop`.
`TooManyProps` |  More props have been spawned than are permitted by the island's rules (currently 100 per script device and 200 total per island).

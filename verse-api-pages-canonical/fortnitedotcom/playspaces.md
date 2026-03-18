## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces

# Playspaces module
Learn technical details about the Playspaces module.
Module import path: /Fortnite.com/Playspaces
  * [`Fortnite.com`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom)
  * **`Playspaces`**

## Interfaces
Name | Description
---|---
[`fort_playspace`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/playspaces/fort_playspace) |  A nested container that scopes objects, style, gameplay rules, visuals, etc. All objects and players in an experience will belong to a fort_playspace. There is typically one `fort_playspace` for an entire experience, though this may change in the future as the platform evolves. To access the `fort_playspace` for a `creative_device` use `creative_device.GetPlayspace`.

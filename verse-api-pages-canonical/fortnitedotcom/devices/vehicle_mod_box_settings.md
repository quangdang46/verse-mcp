## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_mod_box_settings



Table of Contents
  1. ![Epic Games](https://edc-cdn.net/assets/images/logo-epic.svg)[Developer](https://dev.epicgames.com/)
  2. [Documentation](https://dev.epicgames.com/documentation/ "Documentation")
  3. Fortnite
     * [](https://dev.epicgames.com/documentation/en-us/unreal-engine)
     * [](https://dev.epicgames.com/documentation/en-us/fortnite)
     * [](https://dev.epicgames.com/documentation/en-us/twinmotion)
     * [](https://dev.epicgames.com/documentation/en-us/metahuman)
     * [](https://dev.epicgames.com/documentation/en-us/realityscan)
     * [](https://dev.epicgames.com/documentation/en-us/realityscan-mobile)
     * [](https://dev.epicgames.com/documentation/en-us/fab)
  4. vehicle_mod_box_settings class


# vehicle_mod_box_settings class
Learn technical details about the vehicle_mod_box_settings class. 
On this page
Settings for vehicle mod boxes, used with vehicle_mod_box_spawner_device.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`PaintColor` | `??color` |  A custom paint color on the mod box.
  * If the associated mod box uses default visuals, it becomes a blank box with that paint color.
  * Setting this value to `false` removes any custom paint color.

  
`Icon` | `??texture` |  A custom icon on the mod box. Attempting to return this value will fail unless it's your own texture.
  * If the associated mod box uses default visuals, it becomes a blank box with that icon.
  * Setting this value to `false` removes any custom icon.

  
`IconColor` | `??color` |  The custom color of the icon on the mod box. This value has no effect if this entry's icon is not set.
  * Setting this value to `false` removes any custom icon color.

  
`IconScale` | `?float` |  The scale multiplier of the icon on the mod box, clamped between `0.1` and `2.0`. This value has no effect if this entry's icon is not set.
  * This value defaults to `1.0`.

  
### Functions
Function Name | Description  
---|---  
[`ApplyDefaultVisuals`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_mod_box_settings/applydefaultvisuals) |  Reset visual settings and make the associated mod box use its default non-customized visuals.
  * Fails if this is a custom list entry set to `Empty Slot`.

  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_mod_box_settings#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_mod_box_settings#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/vehicle_mod_box_settings#functions)





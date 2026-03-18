## https://dev.epicgames.com/documentation/en-us/fortnite/removing-and-controlling-the-fortnite-default-hud-in-unreal-editor-for-fortnite

# Removing and Controlling the Fortnite Default HUD
Learn how to customize or remove the HUD available by default in Fortnite.
![Removing and Controlling the Fortnite Default HUD](https://dev.epicgames.com/community/api/documentation/image/769ef88b-af8f-478d-a2a1-f1d0c8a17158?resizing_type=fill&width=1920&height=335)
As you design your game in **Unreal Editor for Fortnite (UEFN)** , you might want to customize or remove the HUD available by default in Fortnite.
[![View of Fortnite client with no default HUD elements](https://dev.epicgames.com/community/api/documentation/image/0a52e428-c7b8-406a-b49b-2a7c0d750016?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/0a52e428-c7b8-406a-b49b-2a7c0d750016?resizing_type=fit)
_Fortnite client launched from UEFN with all default HUD elements hidden._
The following sections describe different methods for controlling the HUD in your game.
##  Using the HUD Controller Device
You can use the [HUD Controller device](https://dev.epicgames.com/documentation/en-us/fortnite-creative/using-hud-controller-devices-in-fortnite-creative) to change what will display in your game HUD.
For how to place a device and edit its settings in UEFN, see [UEFN Controls for Creative Users](https://dev.epicgames.com/documentation/en-us/uefn/uefn-controls-for-creative-users-in-unreal-editor-for-fortnite).
##  Using the HUD Controller in Verse
You can also use the [HUD Controller API](https://dev.epicgames.com/documentation/en-us/uefn/verse-api/fortnitedotcom/ui/fort_hud_controller) in Verse to change what displays in your game HUD.
Use the following [device code](https://dev.epicgames.com/documentation/en-us/uefn/create-your-own-device-in-verse) to hide all the default HUD elements in your game:
Verse
```
using { /Fortnite.com/Devices }
using { /Verse.org/Simulation }
using { /UnrealEngine.com/Temporary/Diagnostics }
using { /Fortnite.com/UI }
using { /UnrealEngine.com/Temporary/UI }

# A Verse-authored Creative device that can be placed in a level
hide_default_hud := class(creative_device):

    # Runs when the device is started in a running game

```

Copy full snippet(25 lines long)
You can change the elements in `HUDElementsToHide` to specify what specifically should be hidden if you don’t want to hide everything.
For a list of all the HUD elements you can choose to show or hide in Verse, see the UI module in the [Verse API Reference](https://dev.epicgames.com/documentation/en-us/uefn/verse-api/fortnitedotcom/ui).

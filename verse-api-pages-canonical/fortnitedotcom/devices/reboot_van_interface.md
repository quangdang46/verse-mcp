## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface



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
  4. reboot_van_interface interface


# reboot_van_interface interface
Learn technical details about the reboot_van_interface interface. 
On this page
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/Devices }`  
## Members
This interface has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`RechargeCompleteEvent` | `listenable(payload)` |  Triggers when Reboot Van has finished recharging.
  * `agent` is the last interacting player.

  
`RebootEvent` | `listenable(payload)` |  Triggers when Reboot Van has finished rebooting a set of players.
  * `agent`is the player that started the reboot.

  
`RechargeTimerLength` | `?float` |  The length of the recharge timer in seconds, regardless of the timer's current state. Clamped between `0.0` and `3600.0`.  
`RechargeTimer` | `?float` |  The remaining time (in seconds) on the recharge timer. Clamped between `0.0` and `3600.0`.
  * If there is no active timer, getting returns `0.0`.
  * If there is no active timer, setting does nothing.

  
`RebootProgressDecay` | `?reboot_progress_decay_behavior` |  How quickly reboot progress decays when nobody is interacting with the Reboot Van. Custom Decay - Set a custom multiplier on the decay rate. Instant Reset - Instantly reset progress to zero. Battle Royale - Use Battle Royale's decay rate.  
`DecayRateMultiplier` | `??float` |  Multiplier on the decay rate of reboot progress. Clamped between `0.1` and `2.0`.
  * Only used if `RebootProgressDecay` is set to `Custom Decay`.

  
`RebootCardPurchaseEvent` | `listenable(payload)` |  Triggers when a player purchases a Reboot Card.
  * `agent` is the player that purchased the Reboot Card.

  
`CanPurchaseRebootCard` | `?logic` |  Determines if players can purchase an eliminated player's reboot card.  
`PurchaseRebootCardOptions` | `??reboot_card_purchase_options` |  Purchase reboot card options.
  * Only used if `CanPurchaseRebootCard` is `true`.

  
### Functions
Function Name | Description  
---|---  
[`EnableReboot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface/enablereboot) |  Enable the device.  
[`DisableReboot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface/disablereboot) |  Disable the device.  
[`IsEnabledReboot`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface/isenabledreboot) |  Succeeds if the device is enabled, fails if it's disabled.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ interface](https://dev.epicgames.com/community/search?query=interface)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/reboot_van_interface#functions)





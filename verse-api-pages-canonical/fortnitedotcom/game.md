## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game

# Game module
Learn technical details about the Game module.
Module import path: /Fortnite.com/Game
  * [`Fortnite.com`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom)
  * **`Game`**

## Classes and Structs
Name | Description
---|---
[`elimination_result`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/elimination_result) |  Result data for `fort_character` elimination events.
[`damage_args`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damage_args) |  Parameters for common damage functions on Fortnite objects.
[`damage_result`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damage_result) |  Results for damage events on Fortnite objects.
[`healing_args`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healing_args) |  Parameters for common healing functions on Fortnite objects.
[`healing_result`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healing_result) |  Results for healing events on Fortnite objects.
## Interfaces
Name | Description
---|---
[`fort_round_manager`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/fort_round_manager) |  This interface is implemented by the round manager living on the simulation entity.
[`positional`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/positional) |  Implemented by objects to allow reading position information.
[`healthful`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healthful) |  Implemented by Fortnite objects that have health state and can be eliminated.
[`shieldable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/shieldable) |  Implemented by Fortnite objects that have shields. A shield is a method of protection that can take incoming damage while leaving the health state unchanged.
[`damageable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/damageable) |  Implemented by Fortnite objects that can be damaged.
[`healable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/healable) |  Implemented by Fortnite objects that can be healed.
[`game_action_instigator`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/game_action_instigator) |  Implemented by Fortnite objects that initiate game actions, such as damage and heal. For example, player or agents. Event listeners often use `game_action_instigators` to calculate player damage scores.
[`game_action_causer`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/game/game_action_causer) |  Implemented by Fortnite objects that can be passed through game action events, such as damage and heal. For example: player, vehicle, or weapon. Event Listeners often use `game_action_causer` to pass along additional information about what weapon caused the damage. Systems will then use that information for completing quests or processing game specific event logic.

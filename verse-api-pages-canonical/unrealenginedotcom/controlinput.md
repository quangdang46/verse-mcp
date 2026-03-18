## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput

# ControlInput module
Learn technical details about the ControlInput module.
Module import path: /UnrealEngine.com/ControlInput
  * [`UnrealEngine.com`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom)
  * **`ControlInput`**
    * [`UI`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput/ui)

## Classes and Structs
Name | Description
---|---
[`input_events(t)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput/input_events/input_events\(t\)) |  Input_events is a container for user input events which can be subscribed to.
  * Use the 'GetPlayerInput' and 'GetInputEvents' functions to retrieve an input_events object for a given player.
  * Low-level notifications of current user input: DetectionBeginEvent, DetectionOngoingEvent, and DetectionEndEvent.
  * High-level notifications of triggered events: ActivationTriggeredEvent and ActivationCanceledEvent. /—----------<-------\ DetectionBeginEvent -> DetectionOngoingEvent -> ActivationTriggeredEvent -> DetectionEndEvent /\ /\ / ---------------------> ActivationCanceledEvent ----------------------/

[`player_input`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput/player_input) |  This is the main manager class for input-related settings and functions for a player.
## Functions
Name | Description
---|---
[`input_events`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput/input_events) |  Input_events is a container for user input events which can be subscribed to.
  * Use the 'GetPlayerInput' and 'GetInputEvents' functions to retrieve an input_events object for a given player.
  * Low-level notifications of current user input: DetectionBeginEvent, DetectionOngoingEvent, and DetectionEndEvent.
  * High-level notifications of triggered events: ActivationTriggeredEvent and ActivationCanceledEvent. /—----------<-------\ DetectionBeginEvent -> DetectionOngoingEvent -> ActivationTriggeredEvent -> DetectionEndEvent /\ /\ / ---------------------> ActivationCanceledEvent ----------------------/

[`GetPlayerInput`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/controlinput/getplayerinput) |  Access input-related data and settings for a player.

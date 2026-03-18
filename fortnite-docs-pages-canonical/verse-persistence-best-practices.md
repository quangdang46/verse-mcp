## https://dev.epicgames.com/documentation/en-us/fortnite/verse-persistence-best-practices

# Verse Persistence Best Practices
Learn best practices when handling persistable data in Verse.
![Verse Persistence Best Practices](https://dev.epicgames.com/community/api/documentation/image/1afaf2fb-3670-4888-8538-5f7f56c2703b?resizing_type=fill&width=1920&height=335)
Persistable data allows you to track and save data per player between play sessions. Persistable data works by storing data for each individual player, such as their profile or stats, in Verse. This data can then be updated as many times as the data's value changes. Because this data is persistable, it will persist across game sessions and be available any time the player is online in the game. For more information, see [Using Persistable Data in Verse](https://dev.epicgames.com/documentation/en-us/fortnite/using-persistable-data-in-verse#what-persistence-means-in-verse).
This page covers some best practices when working with persistable data in Verse.
##  Use Classes to Add New Fields Later
Currently the only type of persistable data that you can change after you publish your island is the `class` type, as long as new fields have default values. This means that loading saved data from a previous version will then include the new fields and their default values.
Let's look at an example of publishing a project with the following persistable data.
Verse
```
player_profile_data := class<final><persistable>:
    Class:player_class = player_class.Villager
    XP:int = 0
    Rank:int = 0
```

player_profile_data := class<final><persistable>: Class:player_class = player_class.Villager XP:int = 0 Rank:int = 0
Copy full snippet(4 lines long)
Since the project is published and live, players who have played the game will have this persistent data associated with them. If we added more fields to the player profile data, like quest count and history, the persistable data could then look like the following in the updated project.
Verse
```
player_profile_data := class<final><persistable>:
    Class:player_class = player_class.Villager
    XP:int = 0
    Rank:int = 0
    CompletedQuestCount:int = 0
    QuestHistory:[]string = array{}
```

player_profile_data := class<final><persistable>: Class:player_class = player_class.Villager XP:int = 0 Rank:int = 0 CompletedQuestCount:int = 0 QuestHistory:[]string = array{}
Copy full snippet(6 lines long)
The persistent data for any players who played with the first version of the `player_profile_data` class will now include the new fields:
  * `CompletedQuestCount` with the value of `0`, which is the default value that was specified.
  * `QuestHistory` with an empty string array, which is the default value that was specified.

This works because a default value was provided for the new fields to be able to update the older version of the data.
Because only classes may be updated after a project is published, we strongly recommend using a class as the value type of any module-scoped `weak_map` variable.
For more details on how to create a persistable class, see[ ](https://dev.epicgames.com/documentation/en-us/fortnite/using-persistable-data-in-verse)[Persistable Types](https://dev.epicgames.com/documentation/en-us/fortnite/using-persistable-data-in-verse#persistable-types-in-verse).
##  Using Constructors for Partial Updates
If you are using classes, we recommend using a [constructor](https://dev.epicgames.com/documentation/en-us/fortnite/constructor-in-verse) to create a new instance of your class that contains the updated state, because constructors allow you to do partial updates of classes.
The following example shows how you can update the `PlayerProfileDataMap`. The `GrantXP()` function gets the current data of the given player and then calls the `MakePlayerProfileData()` constructor to make a new version of their profile data. Because the player's source data gets passed to the constructor along with the new XP value, only the XP value will get updated while all of the player's other data will remain the same.
Verse
```
MakePlayerProfileData<constructor>(Src:player_profile_data)<transacts> := player_profile_data:
    Version := Src.Version
    Class := Src.Class
    XP := Src.XP
    Rank := Src.Rank
    CompletedQuestCount := Src.CompletedQuestCount
    QuestHistory := Src.QuestHistory

GrantXP(Agent:agent, GrantedXP:int):void=
    if:

```

Copy full snippet(24 lines long)
The previous example showed how to update one field, but you can update as many as you need to in this way:
Verse
```
set PlayerProfileDataMap[Player] = player_profile_data:
    QuestHistory := UpdatedSaveData.QuestHistory
    CompletedQuestCount := OldData.CompletedQuestCount + 1
    MakePlayerProfileData<constructor>(OldData)
```

set PlayerProfileDataMap[Player] = player_profile_data: QuestHistory := UpdatedSaveData.QuestHistory CompletedQuestCount := OldData.CompletedQuestCount + 1 MakePlayerProfileData<constructor>(OldData)
Copy full snippet(4 lines long)
##  Versioning Persistable Data
We recommend using versioning in persistable classes to detect the instance's version for data previously saved for a player. By using versions, you can detect and apply migrations if your persistable class definition or gameplay logic changes over time.
While you can use integer or string values to denote the version of your persistent class, we recommend using `option` values to store references to current and past versions of your data. Consider the following setup:
Verse
```
var SavedPlayerData:weak_map(player, player_data) = map{}

# A player data class containing optional fields of versioned player data. Only one of these
# optional values should contain a real value at any given time.
player_data := class<final><persistable>:
    V1:?v1_player_data = false
    V2:?v2_player_data = false

# Original version of player data.

```

Copy full snippet(23 lines long)
Here, the `player_data` class contains `option` values for both the first and second versions of the associated data class, which are represented by the `v1_player_data` and `v2_player_data` classes. Only one of `V1` or `V2` should ever be set to prevent players from having multiple versions of data associated with them.
The original `V1` player data contains three `int` fields. The `V2` version of the data changes the `Playtime` field to a `float`, as well as adding two new fields. Because the type of the `Playtime` field changed in the `V2` version, it will need to be converted for any player who still has the old `V1` data. When a player with old `V1` data joins an experience, you can use helper constructor functions to build a new `V2` data class based on their old data like so:
Verse
```
# Create v1_player_data using existing v1_player_data.
MakeV1PlayerData<constructor>(SourceData:v1_player_data)<transacts> := v1_player_data:
    XP := SourceData.XP
    Rank := SourceData.Rank
    Playtime := SourceData.Playtime

# Create v2_player_data using existing v2_player_data.
MakeV2PlayerData<constructor>(SourceData:v2_player_data)<transacts> := v2_player_data:
    XP := SourceData.XP
    Rank := SourceData.Rank

```

Copy full snippet(21 lines long)
There may be times when you want to force a data reset for players joining your island. This can be done by reassigning a default value for the persistable data in the 'weak_map' for all the players and changing the `Version` field of the class. If you use optional versioned data, you can reset data by setting the optional fields to `false`.
To know if the player's data has already been reset, you can check the `Version` value in the player's persistable data to see if it's the latest.
##  Testing Persistent Data Is Within Limits
If your update can affect the persistent data's total size, you should verify that the persistent data still fits within Verse's persistence system constraints. If you try to update the persistable data and it exceeds the size limits, you will get a Verse runtime error. See[ ](https://dev.epicgames.com/documentation/en-us/fortnite/using-persistable-data-in-verse?revision_hash_id=lwem5P)[Max Persistent Object Size](https://dev.epicgames.com/documentation/en-us/fortnite/using-persistable-data-in-verse#max-persistent-object-size) for more details.
You can check how your updates affect the total size by using the `FitsInPlayerMap()` function.
In the following example, the persistable data contains an array of strings. If that array ever gets too large to store in the `weak_map`, which happens when `FitsInPlayerMap()` fails, the example empties the array and only adds the most recent saved element.
Verse
```
# Construct and return a new player_profile_data with updated quest history.
SetQuestHistory(Src:player_profile_data, NewQuestHistory:[]string)<transacts>:player_profile_data =
    NewData:player_profile_data = player_profile_data:
        MakePlayerProfileData<constructor>(Src)
        QuestHistory := NewQuestHistory

# Set a player's quest history in the PlayerProfileDataMap.
RecordQuestHistory(Agent:agent, QuestHistory:string):void=
    if:
        CheckSaveDataForPlayer[Agent]

```

Copy full snippet(44 lines long)
##  Reacting to Player Joining Your Island
When a new player joins your island, they will not have an entry automatically added to the persistable `weak_map`. You will have to add that entry in Verse.
To do this, you can either check whether a player is already in the `weak_map` whenever you access it, or you can add default data to the `weak_map` whenever a player joins, which you can know by subscribing to the game's `PlayerAddedEvent()` event.
Verse
```
GetPlayspace().PlayerAddedEvent().Subscribe(OnPlayerAdded)

# Later in your file
OnPlayerAdded(Player:player):void=
    if:
        not PlayerProfileDataMap[Player]
        set PlayerProfileDataMap[Player] = player_profile_data{}
```

GetPlayspace().PlayerAddedEvent().Subscribe(OnPlayerAdded) # Later in your file OnPlayerAdded(Player:player):void= if: not PlayerProfileDataMap[Player] set PlayerProfileDataMap[Player] = player_profile_data{}
Copy full snippet(7 lines long)

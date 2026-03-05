# Sample Fortnite.digest.verse for testing
# This is a subset of real device definitions

device_campfire_device = class():
    # Triggers
    TriggerOnEnterRadius():event():void
    TriggerOnExtinguish():event():void
    OnDisabled():event():void
    OnEnabled():event():void
    # Receivers
    ReceiverAddFuel(Amount:int):void
    EnableWhenReceiving():void
    DisableWhenReceiving():void
    # Methods
    AddFuel(Amount:int):void
    Extinguish():void
    Light():void

device_button_device = class():
    # Triggers
    InteractWithAgent():event(such as {Agent}):void
    OnDisabled():event():void
    OnEnabled():event():void
    # Receivers
    EnableWhenReceiving():void
    DisableWhenReceiving():void
    # Methods
    Disable():void
    Enable():void
    Hide():void
    Show():void

player_spawner_device = class():
    # Triggers
    Spawned():event(such as {Agent}):void
    OnDisabled():event():void
    OnEnabled():event():void
    # Receivers
    EnableWhenReceiving():void
    DisableWhenReceiving():void
    # Methods
    SpawnAgent(Agent:agent):void
    Disable():void
    Enable():void

score_manager_device = class():
    # Triggers
    ScoreChanged():event(such as {Agent}):void
    # Receivers  
    AddScoreWhenReceiving(Amount:int):void
    # Methods
    AddScore(Agent:agent, Amount:int):void
    GetScore(Agent:agent):int
    ResetScore(Agent:agent):void

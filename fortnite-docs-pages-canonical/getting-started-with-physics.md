## https://dev.epicgames.com/documentation/en-us/fortnite/getting-started-with-physics

# Getting Started with Physics
Learn tips and get tricks on how to use physics in Unreal Editor for Fortnite.
![Getting Started with Physics](https://dev.epicgames.com/community/api/documentation/image/05db44d6-b940-421e-9e80-870ae16e87cf?resizing_type=fill&width=1920&height=335)
**Physics** in **Unreal Editor for Fortnite (UEFN)** simulates the effects of physical forces in things like collisions, explosions, and the motion of objects. The simulations calculate how fast something accelerates when falling or how one object interacts with another when they collide.
Physics in UEFN**is now in Beta**. You can now publish projects with physics, but be advised that the tools are still undergoing frequent updates which may affect your UEFN islands.
##  What You Can Do
Players can **interact** with physics objects by:

  * **Moving into them** with a physically-based player character.
  * **Striking them** with a Pickaxe or **shooting them** with a weapon, including throwing grenades.
  * **Using a Prop Mover** to move them or rotate them.
  * **Using devices**.

##  Physics-Enabled Devices
The following devices have been tested with physics props and reliably work:
|  |  |  |
---|---|---|---|---
[Air Vent](https://dev.epicgames.com/documentation/en-us/fortnite/using-air-vent-devices-in-fortnite-creative) |  [Barrier](https://dev.epicgames.com/documentation/en-us/fortnite/using-barrier-devices-in-fortnite-creative) |  [Bouncer](https://dev.epicgames.com/documentation/en-us/fortnite/using-bouncer-gallery-devices-in-fortnite-creative) |  [Bouncer Trap](https://dev.epicgames.com/documentation/en-us/fortnite/bouncer-trap) |  [Volume](https://dev.epicgames.com/documentation/en-us/fortnite/using-volume-devices-in-fortnite-creative)
[D-Launcher](https://dev.epicgames.com/documentation/en-us/fortnite/using-dlauncher-devices-in-fortnite-creative) |  [Damage Volume](https://dev.epicgames.com/documentation/en-us/fortnite/using-damage-volume-devices-in-fortnite-creative) |  [Hover Platform](https://dev.epicgames.com/documentation/en-us/fortnite/using-hover-platform-devices-in-fortnite-creative) |  [Pinball Bumper](https://dev.epicgames.com/documentation/en-us/fortnite/using-pinball-bumper-devices-in-fortnite-creative) |  [Pinball Flipper](https://dev.epicgames.com/documentation/en-us/fortnite/using-pinball-flipper-devices-in-fortnite-creative)
[Prop Mover](https://dev.epicgames.com/documentation/en-us/fortnite/using-prop-mover-devices-in-fortnite-creative) |  [Skydive Volume](https://dev.epicgames.com/documentation/en-us/fortnite/using-skydive-volume-devices-in-fortnite-creative) |  [Teleporter](https://dev.epicgames.com/documentation/en-us/fortnite/using-teleporter-devices-in-fortnite-creative) |  [Trigger](https://dev.epicgames.com/documentation/en-us/fortnite/using-trigger-devices-in-fortnite-creative) |  [Water](https://dev.epicgames.com/documentation/en-us/fortnite/using-water-devices-in-fortnite-creative)
|  [Crash Pad](https://dev.epicgames.com/documentation/en-us/fortnite/using-crash-pad-devices-in-fortnite-creative) |  [Explosives](https://dev.epicgames.com/documentation/en-us/fortnite/using-explosive-devices-in-fortnite-creative) |  **
** |
##  Enable Physics
To turn on this Beta feature, you need to enable physics in your project settings:
  1. Open UEFN and select a new or existing project.
  2. Go to **Project** > **Project Settings**.
  3. Under **Beta Access** , check the box next to **Physics**.
[![Physics Beta access](https://dev.epicgames.com/community/api/documentation/image/c083138f-9d7b-48be-8a8c-3ac5512587f2?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/c083138f-9d7b-48be-8a8c-3ac5512587f2?resizing_type=fit) Physics Beta access
Note that physics is not currently supported for any brand templates.

##  Physical Motion
In the game environment, physics objects are subject to the same natural forces described in Newton's second law of motion: **F=ma** , where **Force (F)** is equivalent to **Mass (m)** x **Acceleration (a)**.
####  Force
Force maintains, alters or distorts the **motion** of a body. In video games, this force can be applied to characters and objects by weapons, other moving objects, devices, explosions and more.
####  Mass
Mass is a core property of all matter, and is how much a body of matter **resists** a change in its velocity when a force is applied. The more mass, the less change a force can apply. In a game, objects with physical properties have a set mass. Objects with a large mass are more difficult to move, and cover less distance after interacting with another object.
####  Acceleration
Acceleration is the rate at which the velocity of a body changes as it moves. This is evaluated in terms of **velocity** and **direction**. In a game, this can be the acceleration that a pinball bumper gives an object that interacts with it.
##  Add Physics to Props
All Fortnite props in a physics-enabled project can be set to simulate physics. There are two ways to add physics to props:
###  Using Fortnite Tools
  1. In the toolbar, click the **Selection Mode** dropdown and select **Fortnite Tools**.
[![Fortnite tools](https://dev.epicgames.com/community/api/documentation/image/18d0a48e-3ee6-4147-8287-acae52335839?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/18d0a48e-3ee6-4147-8287-acae52335839?resizing_type=fit) Fortnite tools
  2. Click **Add Physics**.
  3. From the Viewport or the Outliner, select the props you want to add physics to by holding down **Ctrl** and clicking, then click the **Add Physics** button under **Actions**. You should see a red confirmation message appear above the button.
[![Add physics](https://dev.epicgames.com/community/api/documentation/image/ea263a29-9c8f-4203-a204-223964070573?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/ea263a29-9c8f-4203-a204-223964070573?resizing_type=fit) Add physics

Using this method you can also Remove Physics, and Select all physics or non-physics props in your project. To learn more, check out [Fortnite Tools Mode](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-tools-mode-in-fortnite#add-physics).
###  Adding a Physics Component
  1. In the Details panel for the prop, click **+ Add** and choose **Fort Physics**.
[![](https://dev.epicgames.com/community/api/documentation/image/717f1263-31d6-437a-af6f-fbf19c9701c3?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/717f1263-31d6-437a-af6f-fbf19c9701c3?resizing_type=fit)
  2. Check **Simulate Physics** and modify the other settings as you see fit
[![](https://dev.epicgames.com/community/api/documentation/image/2db02ace-2d0b-4fc7-af5a-768acbd2668e?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/2db02ace-2d0b-4fc7-af5a-768acbd2668e?resizing_type=fit)

Learn more about importing a custom prop and adding collision in the [Make a Soccer Game](https://dev.epicgames.com/documentation/en-us/fortnite/make-a-soccer-game) tutorial.
###  User Options
**Mass** lets you determine how heavy the prop is. The heavier the prop, the less impact a force will have on it. In the example below, the crate on the left has a mass of **100** , while the one on the right has a mass of **25**.
**Enable Gravity** determines if the object should have the force of gravity applied to it.
**Start Awake** lets you choose whether the physics prop simulates on game start or when an object comes into contact with it. If unchecked, the physics object will not simulate until a force is applied to it. In the example below, the crates all have **Start Awake** **unchecked** , and only fall after being hit.
**Linear Damping** slows objects down over time. In the example below, the balloon on the left has a Linear Dampening value of **0.5** while the balloon on the right is set to **0.01**.
**Angular Damping** slows down the rotation of objects over time. To learn more about this topic, check out [Physics Damping](https://dev.epicgames.com/documentation/en-us/unreal-engine/physics-damping-in-unreal-engine?application_version=5.5) in the Unreal Engine documentation. In the example below, the left triangle has an Angular Damping value of **0.6** and the right triangle is set to **0.1**.
**Impulse on Hit Multiplier** lets you multiply the force applied to an object when hit. The default is**1** , but it can be set to any value between**-5** and **5**. In the example below, the left soccer ball has a **1.0** multiplier, while the right one is set to **3.0**.
[Soccer Ball](https://sketchfab.com/3d-models/soccer-ball-88590cf1e42e44bfb85ce3b6b1959648) by [tinmanjuggernaut](https://sketchfab.com/tinmanjuggernaut) on Sketchfab, used under the [Sketchfab Standard License](https://sketchfab.com/licenses).
###  Constraints
You can add **Rotation and Translation constraints** to your props to prevent them from moving along certain axes. To learn more about constraints, check out the [Physical Constraint Reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/physics-constraint-reference-in-unreal-engine?application_version=5.5) in the Unreal Engine documentation.
##  FAQ
The following is a non-comprehensive list of questions expected to arise when trying this new feature. If your question doesn't appear in this section, consider checking the latest Fortnite Ecosystem Updates & Release notes for updates.
#####  Can I enable physics on an existing project?
Yes. Physics can be enabled on your existing projects, however, it is recommended that you make a copy of your project before turning on physics. You cannot publish a project that uses an experimental feature. You can disable the physics experimental feature at any time and publish your island.
#####  Can objects animated with Sequencer interact with physics objects?
No. Sequenced objects run on the server's game thread, which has different properties from the physics thread and does not enact the changes required for physical interaction. Essentially, Sequencer is not meant to be used alongside physics. We recommend using the Prop Mover device. Props moved using the Prop Mover can interact with physics objects.
#####  Are all player movement modes available with physics?
The basic player movements, like walking, running, sprinting, sprint jumping, jumping, falling, swimming, crouching, crouch walking, crouch jumping, gliding and skydiving, should work as expected. However, mantling, sliding, ziplining, grind rails, and down but not out (DBNO) are not currently available.
#####  Are all weapons supported?
Most weapons are supported with the physics Beta. As the toolset becomes more robust, you can expect more weapons to work with physics.
#####  Will vehicles work on my physics-enabled island?
Vehicles are not supported, and create instability on your island. We recommend not using them as they may break parts of your game.
#####  How many physics objects can I have in my project?
You should monitor the number of physics objects in your project. Exceeding 50 physically-simulated simple shapes (boxes or spheres) could affect the island performance. Complex physics shapes will impact performance more than simple shapes. You could have different results based on the complexity of your environment and of the objects that simulate.
#####  Does Physics work in Fortnite Creative?
At this time, physics is only available to use with UEFN.
##  Volume Device API
To detect the presence of a physics object, two events have been added to the device: `OnPhysicsEnter` and `OnPhysicsExit`.
The payload for these events is **`Creative_Prop`.**
##  Verse API
The `PropEntersEvent` and `PropExitsEvent` have been added. The snippet below can be used to track when agents enter and exit a volume:
Verse
```

```

volume_device<public> := class<concrete><final>(creative_device_base): PropEntersEvent<public>:listenable(creative_prop) = external {} PropExitsEvent<public>:listenable(creative_prop) = external {}
Copy full snippet(5 lines long)
The `fort_character` class has the ability to get and set linear velocity, get the character’s mass, and apply a linear impulse or a force. The `creative_prop` class can do all of that as well as get and set angular velocity and apply an angular impulse and a torque. See the [fort_character](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/characters/fort_character) and [creative_prop](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/devices/creative_prop) API pages for more information.

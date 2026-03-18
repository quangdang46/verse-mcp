## https://dev.epicgames.com/documentation/en-us/fortnite/using-post-processing-devices-in-fortnite-creative

# Post Process Devices
Add effects to set a mood or enhance your game mechanics.
![Post Process Devices](https://dev.epicgames.com/community/api/documentation/image/96fcd5a8-c77f-4fd6-b813-0754dcbf6bd9?resizing_type=fill&width=1920&height=335)
**Post-processing** refers to customizable filters that you can use to create specific visual effects. Most of these filters primarily affect lighting.The **Post Process** device provides a way to apply these different effects on your island.
You may recognize some of the effects from the **camera filters** in **Island Settings** under the [**World**](https://dev.epicgames.com/documentation/en-us/fortnite/world-settings-in-fortnite-creative) category, but this device provides even more options.
For a Post Process effect to work correctly, make sure the **Camera Filter** option in the World category is set to **Default (none)**.
Apply these effects to a specific player, throughout an entire island, or set up different effects to happen based on user interactions or activation by other devices. You can also set the effect to remain indefinitely, or to turn off after a specified time or based on [event bindings](https://dev.epicgames.com/documentation/en-us/fortnite/using-post-processing-devices-in-fortnite-creative).
You can control transitions between different effects or from no effect to effect and back again by using the **blend** options to set how an effect blends from one state to another.
These effects can be used to simulate or enhance environmental factors or character moods or attitudes.
Using UEFN? Learn more about [post-process effects](https://dev.epicgames.com/documentation/en-us/uefn/intro-to-post-processing-in-unreal-editor-for-fortnite) in our UEFN and Verse documentation.
For help on how to find the **Post Process** device, see [Using Devices](https://dev.epicgames.com/documentation/en-us/fortnite/using-devices-in-fortnite).
If you're using multiple copies of a device on an island, it can be useful to [rename](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary#rename-a-device) them. Choosing names that relate to a device's purpose makes it easier to remember what each one does, and easier to find a specific device when using the [**Event Browser**](https://dev.epicgames.com/documentation/en-us/fortnite/event-browser-in-fortnite-creative).
##  Device Options
Configure this device with the following options.
Default values are **bold**.
Option  |  Value  |  Description
---|---|---
**Enabled During Phase** |  None, **Always** , Pre-Game Only, Gameplay Only, Create Only |  Sets the phase in which the device will be enabled.
**Post Process Effect** |  **None** , Pick an effect |  This is the effect that is applied when the device is enabled. See [Effect Options](https://dev.epicgames.com/documentation/en-us/fortnite/using-post-processing-devices-in-fortnite-creative) below for more info on each effect.
**Effect Duration** |  **Infinite** , Pick a time in seconds |  How long the post process effect will last. **Infinite (0)** continues the effect indefinitely, or you can set a time for when it starts blending out.
**Priority** |  **0** , Pick a priority |  If you're using more than one effect and they overlap, this determines which effect will display. If two or more effects have the same priority, they will attempt to blend together, but they may not blend the same way for every player.
**Starting Strength** |  **1.0** , Pick a value |  How strong the effect you use is at the start. The higher the value, the more intense the effect is. The value you set here clamps to **Blend In Strength**.
**Blend In Strength** |  **0.0** , Pick a value |  How strong the effect is when blended in.
**Blend In Duration** |  **0.0** , Pick a time in seconds |  How long it takes for a blend to go from 0 strength to full blend.
**Blend Out Duration** |  **0.0** , Pick a time in seconds |  How long it takes for a blend to go back to 0 strength.
**Applies to Team** |  **Any** , Pick a team |  Sets which team can activate the device and see the effect.
**Applies to Class** |  **Any** , Pick a class |  Sets which class can activate the device and see the effect.
##  Effect Options
Effect  |  What It Looks Like
---|---
**None** : How things look with no effect applied. |  [![](https://dev.epicgames.com/community/api/documentation/image/d8745fa9-f34b-4c7d-88fd-e24f02ccdc30?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/d8745fa9-f34b-4c7d-88fd-e24f02ccdc30?resizing_type=fit)
**Oak** : Washes out the color and shadows and applies subtle outlines. |  [![](https://dev.epicgames.com/community/api/documentation/image/ae34bb4f-54a5-4fa1-8c2f-4be2c0d8e61e?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/ae34bb4f-54a5-4fa1-8c2f-4be2c0d8e61e?resizing_type=fit)
**Dark** : Makes things look pretty danged dark. Good for creating a night setting. |  [![](https://dev.epicgames.com/community/api/documentation/image/bbd1d54e-58ec-4ceb-9ca3-48ec4549744f?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/bbd1d54e-58ec-4ceb-9ca3-48ec4549744f?resizing_type=fit)
**Film Noir** : Gives everything a washed-out black-and-white effect. |  [![](https://dev.epicgames.com/community/api/documentation/image/c9cdb059-7cb9-4ab0-926b-d15c3b0010f4?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/c9cdb059-7cb9-4ab0-926b-d15c3b0010f4?resizing_type=fit)
**Film Warm** : Warms up the appearance by increasing the yellow. |  [![](https://dev.epicgames.com/community/api/documentation/image/0412257c-4eef-474c-b9c0-c2878ecb98e9?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/0412257c-4eef-474c-b9c0-c2878ecb98e9?resizing_type=fit)
**Happy Place** : Uses a cheerful palette that makes your island scene look fun! |  [![](https://dev.epicgames.com/community/api/documentation/image/5fde843d-d215-42fe-a1ff-c900814ea096?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/5fde843d-d215-42fe-a1ff-c900814ea096?resizing_type=fit)
**Pixelizer** : Pixelizes the image in a way that conjures up old video game consoles from last century. |  [![](https://dev.epicgames.com/community/api/documentation/image/8c373522-2ad6-479c-aea4-4b4abe0e3e21?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/8c373522-2ad6-479c-aea4-4b4abe0e3e21?resizing_type=fit)
**Red** : Gives everything a red hue. |  [![](https://dev.epicgames.com/community/api/documentation/image/416821ab-db22-4257-9668-4c51c6981aba?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/416821ab-db22-4257-9668-4c51c6981aba?resizing_type=fit)
**Sepia** : Gives the scene a reddish-brown hue, like an old Wild West photograph. |  [![](https://dev.epicgames.com/community/api/documentation/image/78cdcc37-210f-4882-af01-329d1b2b1a10?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/78cdcc37-210f-4882-af01-329d1b2b1a10?resizing_type=fit)
**Crazy** : This effect makes your players want to emote like no one's watching! The solarization effect reverses colors in unexpected ways. |  [![](https://dev.epicgames.com/community/api/documentation/image/62d76e03-b864-42e0-bb21-9f9d0779b81c?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/62d76e03-b864-42e0-bb21-9f9d0779b81c?resizing_type=fit)
**Retro** : Outlines images with a glowing line. |  [![](https://dev.epicgames.com/community/api/documentation/image/30c75ad9-61f0-4ce3-843a-bb6172c51871?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/30c75ad9-61f0-4ce3-843a-bb6172c51871?resizing_type=fit)
**Spooky** : Desaturates the colors just enough to create an edgy feeling. |  [![](https://dev.epicgames.com/community/api/documentation/image/5754934b-78eb-481b-b9ea-e505611c2d57?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/5754934b-78eb-481b-b9ea-e505611c2d57?resizing_type=fit)
**Neon Party** : Applies a neon glow to things, but more subtly than the retro effect does. |  [![](https://dev.epicgames.com/community/api/documentation/image/5572f474-6c01-4010-ad53-b8fbcb8c0572?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/5572f474-6c01-4010-ad53-b8fbcb8c0572?resizing_type=fit)
**Horror Movie** : Washes out color, but less so than the low exposure effect. |  [![](https://dev.epicgames.com/community/api/documentation/image/5819fc49-ec50-4e4b-af23-7c7ffbf286d5?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/5819fc49-ec50-4e4b-af23-7c7ffbf286d5?resizing_type=fit)
**Old Cartoon** : Applies outlines similarly to Comic, but in black and white. It also adds a static effect that simulates old film moving through an analog projector. |  [![](https://dev.epicgames.com/community/api/documentation/image/f6139a28-674c-4684-9fce-c193e1a2cbb2?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f6139a28-674c-4684-9fce-c193e1a2cbb2?resizing_type=fit)
**Desolate** : Deepens shadows regardless of time-of-day setting, which creates a feeling of foreboding. |  [![](https://dev.epicgames.com/community/api/documentation/image/ef8417d9-534d-4552-a256-00b281e4ac90?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/ef8417d9-534d-4552-a256-00b281e4ac90?resizing_type=fit)
**Halftone** : A bright effect that uses a texture similar to Comic and Neocomic. |  [![](https://dev.epicgames.com/community/api/documentation/image/1adfe66b-7267-4450-8f5b-a7e6bb6b6538?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/1adfe66b-7267-4450-8f5b-a7e6bb6b6538?resizing_type=fit)
**CCTV** : Shows a low-fidelity image in monochrome as though you're watching over a closed-circuit security camera. |  [![](https://dev.epicgames.com/community/api/documentation/image/c582f26e-3444-4b3d-b320-2e3fabb74552?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/c582f26e-3444-4b3d-b320-2e3fabb74552?resizing_type=fit)
**70s Print** : This effect is reminiscent of a Polaroid snapshot that has faded over decades. |  [![](https://dev.epicgames.com/community/api/documentation/image/860bb1da-cdf8-4761-91ec-c92931b9e9da?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/860bb1da-cdf8-4761-91ec-c92931b9e9da?resizing_type=fit)
**Action Lines** : Dynamic lines radiating outward from the character and action. This effect can convey excitement when a character reacts to something dramatically. |  [![](https://dev.epicgames.com/community/api/documentation/image/0d56178a-e468-4ccd-a8a4-338884966042?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/0d56178a-e468-4ccd-a8a4-338884966042?resizing_type=fit)
**Comic** : Applies an outline to details. |  [![](https://dev.epicgames.com/community/api/documentation/image/21b73972-44f3-4f3e-9e5f-7c6d363abc86?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/21b73972-44f3-4f3e-9e5f-7c6d363abc86?resizing_type=fit)
**Low Exposure** : Washes out much of the color and contrast, the way an underexposed photo would. |  [![](https://dev.epicgames.com/community/api/documentation/image/3583f4ca-f152-4ab9-aa1a-c1cd49f43a03?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/3583f4ca-f152-4ab9-aa1a-c1cd49f43a03?resizing_type=fit)
**Neocomic** : Intensifies contrast, and adds subtle outlines. |  [![](https://dev.epicgames.com/community/api/documentation/image/a0602620-0bc4-43d1-bc2a-d3f55be2f968?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/a0602620-0bc4-43d1-bc2a-d3f55be2f968?resizing_type=fit)
**Nightvision** : Seeing the world through night-vision goggles makes this effect great for stealth games or missions. |  [![](https://dev.epicgames.com/community/api/documentation/image/4ac1f91a-502c-43d1-80d4-ead842cf8930?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/4ac1f91a-502c-43d1-80d4-ead842cf8930?resizing_type=fit)
**Radial Blur** : Creates a blur effect that radiates from a central point. |  [![](https://dev.epicgames.com/community/api/documentation/image/d024f162-e1e5-473c-959c-795edda77c89?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/d024f162-e1e5-473c-959c-795edda77c89?resizing_type=fit)
**Simple Blur** : Makes everything blurry. What the world looks like when some people take off their glasses. |  [![](https://dev.epicgames.com/community/api/documentation/image/a2492c9e-5dcf-45f5-b218-1cf39c2dea0b?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/a2492c9e-5dcf-45f5-b218-1cf39c2dea0b?resizing_type=fit)
**VHSfilter** : What things look like when a VHS tape that was played too many times, this is an effect that conjures a vibe from last century, right down to static on the screen and moving bands of color. |  [![](https://dev.epicgames.com/community/api/documentation/image/38ddce09-c1fc-43e8-a048-1f918457ebbd?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/38ddce09-c1fc-43e8-a048-1f918457ebbd?resizing_type=fit)
**Vignette** : An effect that simulates darkening in a real-world camera lens.Vignetting is mostly noticeable near the edges of the image. |  [![](https://dev.epicgames.com/community/api/documentation/image/21b46672-dd0f-472f-9faa-426579d5ee1e?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/21b46672-dd0f-472f-9faa-426579d5ee1e?resizing_type=fit)
**Heatwave** : A shimmery effect that mimics looking at things in extreme heat such as in a desert area. |  [![](https://dev.epicgames.com/community/api/documentation/image/6b9cef07-f0e4-4bf9-8600-bb41b13eb9db?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/6b9cef07-f0e4-4bf9-8600-bb41b13eb9db?resizing_type=fit)
**Rain** : Shows raindrops as though on the surface of the camera lens. This is useful when you're creating a rain environment, or when the camera is coming out of water. |  [![](https://dev.epicgames.com/community/api/documentation/image/e27175d8-014a-410e-a86e-36a3c7de0375?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/e27175d8-014a-410e-a86e-36a3c7de0375?resizing_type=fit)
**Frost** : An effect like frost on a window pane. It's primarily around the edges. |  [![](https://dev.epicgames.com/community/api/documentation/image/278f9ca2-06eb-44bc-8b37-5ef2969ecd9c?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/278f9ca2-06eb-44bc-8b37-5ef2969ecd9c?resizing_type=fit)
**80s Cartoon** : Applies outline similarly to Comic, but with brighter and flatter colors. |  [![](https://dev.epicgames.com/community/api/documentation/image/273f49cf-153b-4af2-8224-40f165dfabaa?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/273f49cf-153b-4af2-8224-40f165dfabaa?resizing_type=fit)
**Comic Noir** : Applies an outline like Comic or Old Cartoon, but has no static and includes a white border around the entire game screen. |  [![](https://dev.epicgames.com/community/api/documentation/image/a71862f2-b342-40af-bc2a-00e3e220fb92?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/a71862f2-b342-40af-bc2a-00e3e220fb92?resizing_type=fit)
**Heavy Linework** : Applies a black and white comic book filter with heavy linework around assets and characters on screen. |  [![](https://dev.epicgames.com/community/api/documentation/image/1c623eba-dd6c-42b4-9467-8e454c7a84c2?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/1c623eba-dd6c-42b4-9467-8e454c7a84c2?resizing_type=fit)
##  Direct Event Binding
Following are the [direct event binding](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary) options for this device.
###  Functions
A [function](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary) listens for an event on a device then performs an action.
  1. For any function, click the **option** , then **Select Device** to access and select from the **Device** dropdown menu.
  2. Once you've selected a device, click **Select Event** to bind the device to an event that will trigger the function for the device.
  3. If more than one device or event triggers a function, press the **Add** button to add a line and repeat these steps.

Option  |  Description
---|---
**Enable for All When Receiving From** |  Enables the device for all players when an event occurs.
**Disable for All When Receiving From** |  Disables the device for all players when an event occurs.
**Blend In for All When Receiving From** |  Starts the Blend In from current strength to the Blend In Strength value for all players when an event occurs.
**Blend Out for All When Receiving From** |  Starts the Blend Out from current strength to 0 value for all players when an event occurs.
**Reset for All When Receiving From** |  Resets to the initial starting strength for all players when an event occurs. This also ends any ongoing blending.
**Enable for Instigator When Receiving From** |  Enables the device only for the instigating player when an event occurs.
**Disable for Instigator When Receiving From** |  Disables the device only for the instigating player when an event occurs. It also pauses (and hides) any ongoing blending until the device is re-enabled.
**Blend In for Instigator When Receiving From** |  Starts blending in when an event occurs, but only for the instigating player.
**Blend Out for Instigator When Receiving From** |  Starts blending out when an event occurs, but only for the instigating player.
**Reset for Instigator When Receiving From** |  Resets to the initial starting strength when an event occurs, but only for the instigating player.
###  Events
An [event](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-creative-glossary#event) tells another device when to perform a function.
  1. For any event option, click the **option** , then **Select Device** to access and select from the **Device dropdown menu**.
  2. Once you've selected a device, click **Select Function** to bind this event to a function for that device.
  3. If more than one function is triggered by the event, click the **Add** button to add a line and repeat these steps.

Option  |  Description
---|---
**On Blending In Is Send Event To** |  Sends an event to a linked device when blending in is complete.
**On Blending Out Is Send Event To** |  Sends an event to a linked device when blending out is complete.

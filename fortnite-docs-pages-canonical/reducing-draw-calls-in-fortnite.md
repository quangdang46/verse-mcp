## https://dev.epicgames.com/documentation/en-us/fortnite/reducing-draw-calls-in-fortnite

# Reducing Draw Calls
Learn why it’s important to limit the number of draw calls in your projects, and how to do it.
![Reducing Draw Calls](https://dev.epicgames.com/community/api/documentation/image/6c2462f8-cdcd-4ac4-8926-f0336a2c878f?resizing_type=fill&width=1920&height=335)
A**draw call** is the process of gathering and sending data to the GPU (Graphics Processing Unit) to render part of a frame.
Reducing draw calls is one of the best methods for improving the performance of your projects, which is particularly important for lower-end target platforms like mobile.
##  The Draw Call Process
A draw call can contain data for static meshes, skeletal meshes, sprites, UI elements, and more.
Let’s use the example of a **red metal stool** in a UEFN scene. What needs to happen for this simple object to appear in your game?
[![red metal stool](https://dev.epicgames.com/community/api/documentation/image/de7ff644-9417-4794-a7cc-9c97300e1e66?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/de7ff644-9417-4794-a7cc-9c97300e1e66?resizing_type=fit)
###  1. CPU
Think of the CPU (Central Processing Unit) as the project manager. It looks at the red metal stool and gathers all the instructions:
  * **Transform data**. Where is the chair? (location, rotation and scale)
  * **Mesh data**. What is the shape? (specific static mesh asset)
  * **Material data**. What does the surface look like? (material instance, which includes the red color and the metal shine)
  * **Visibility**. Is the player looking at this stool? If yes, the CPU proceeds.

All of these instructions are placed into a manifest, and are now ready to be sent.
###  2. Command
The CPU sends a specialized command to the GPU:
  * “Here is a section of memory that contains a stool shape. Use the red metal shader and place it at these coordinates. Go!” **This is the draw call**.
  * The CPU stops what it’s doing to send this message, so if there are 1000 stools that aren’t instanced, the CPU needs to send 1000 individual messages, which creates a **bottleneck**.

###  3. GPU
The GPU is a very fast painter that waits for instructions to get going. Once it receives the manifest, it does the following:
  * **Rasterization**. It flattens the 3D coordinates to the 2D screen grid.
  * **Pixel Shading**. It runs the material code, calculating how the light in the scene hits the stool’s material. If there is a light nearby, it calculates the**specular highlight**(the white shine on the metal).
  * **Output**. It writes the final colored pixels to the **frame buffer** , which is then sent to your monitor.

If you are running the game at 60 FPS (frames per second) and there are 3,000 draw calls per frame, your GPU is handling 180,000 manifests every second, which is millions of manifests a minute!
The next section explains how to view the amount of draw calls happening in your project.
##  Viewing Draw Calls
You can view draw calls in the editor using **Spatial Profiler** , or by using **Runtime Performance Stats** in-game.
###  Spatial Profiler
To see draw calls with the **Spatial Profiler** :
  1. In the editor toolbar, select **Tools** > **Spatial Profiler**.
[![spatial profiler tool](https://dev.epicgames.com/community/api/documentation/image/7dbb3f37-0456-4885-bd73-008c4f7b5d4d?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/7dbb3f37-0456-4885-bd73-008c4f7b5d4d?resizing_type=fit)
  2. With an active session, select your player from the**Add source to Live Session (1)** dropdown.
[![spatial profiler UI](https://dev.epicgames.com/community/api/documentation/image/c2e3424a-7f60-4a83-9049-358c6150f2fe?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/c2e3424a-7f60-4a83-9049-358c6150f2fe?resizing_type=fit)
  3. In the **Metrics** list under **Rendering** , select **Draw Call Count (2)**.
  4. Click the**Record (3)** button to collect data.
  5. Hover over the bars in the graph **(4)** to see the exact number of draw calls.
  6. Click **Stop (5)** when finished.

You can **save** and **load** sessions, which is particularly useful if you are trying to track improvements over time.
####  Tips
  * Collect data in Play mode. Edit mode could give different results because the on-screen elements are different.
  * Sample from as many platforms and settings as possible. Draw call counts differ vastly between Nanite-enabled sessions and sessions where Nanite is disabled.

Check out the [Spatial Profiler documentation](https://dev.epicgames.com/documentation/en-us/fortnite/spatial-profiler-in-unreal-editor-for-fortnite) for more information.
###  Runtime Performance Stats
To view draw calls in the game client, use the **Runtime Performance Stats** overlay:
  1. Launch a session in your project.
  2. Select **Settings** > **Game UI** > **(Creative) Runtime Performance Stats**.
[![performance stats setting](https://dev.epicgames.com/community/api/documentation/image/f0970fb3-258d-4ee0-9d51-b051b9d0a20e?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f0970fb3-258d-4ee0-9d51-b051b9d0a20e?resizing_type=fit)

The draw calls, along with other performance metrics, are now displayed at the top of your play session.
[![performance metrics](https://dev.epicgames.com/community/api/documentation/image/9985e4da-9830-471e-b5bf-8b514b409ba3?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/9985e4da-9830-471e-b5bf-8b514b409ba3?resizing_type=fit)
##  Reducing Draw Calls
Reducing the amount of draw calls improves performance.
Imagine having to get 50 apples from your car to your kitchen (an apple is a draw call in this analogy): would it be faster to do 50 trips, bringing one apple at a time, or one trip with all 50?
**You can reduce draw calls by** :
  * Reducing material counts
  * Merging certain assets
  * Using instanced static meshes (ISMs)
  * Adjusting LODs and HLODs
  * Using Nanite

###  Reducing Material Count
Minimizing the number of material slots on a mesh is one of the best ways to reduce draw calls. Having multiple material slots can negatively affect real-time rendering of the asset.
If 10 meshes with 5 materials each are visible at the same time, this results in 50 unique draw calls. Increase this to 100 meshes and you now need 500 unique draw calls.
The key mesh below has two material slots: one for the metal of the key, and one for the gem.
[![key two materials](https://dev.epicgames.com/community/api/documentation/image/f8d4f52b-ce4e-43ab-89d5-2f470a50b894?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/f8d4f52b-ce4e-43ab-89d5-2f470a50b894?resizing_type=fit)
This key requires two draw calls in order to appear on screen. The highlighted sections each account for one draw call.
[![key materials highlighted](https://dev.epicgames.com/community/api/documentation/image/8c820908-fefe-4c10-a9d0-84390a412a0b?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/8c820908-fefe-4c10-a9d0-84390a412a0b?resizing_type=fit)
Ideally, you should have 1 material per mesh.
[![key one material](https://dev.epicgames.com/community/api/documentation/image/221db4d3-40bc-448f-a18c-4c6ee3648944?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/221db4d3-40bc-448f-a18c-4c6ee3648944?resizing_type=fit)
UEFN uses auto-instancing to merge identical meshes with identical materials into fewer draw calls.
###  Merging Assets
Combine meshes in your asset creation software to reduce draw calls.
For example, importing a desk covered in items as individual objects with unique materials creates a draw call for each item, which means 11 or 12 calls in this case. Combining these objects into a single mesh with one shared material reduces this to a single draw call.
[![merged assets](https://dev.epicgames.com/community/api/documentation/image/b6eb461e-2256-4b6c-900f-5d211c5231f7?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/b6eb461e-2256-4b6c-900f-5d211c5231f7?resizing_type=fit)
This method does have a drawback: to vary the placement of objects, you must create a new version of the entire mesh, which can affect your memory budget. Balance runtime performance and memory budget by assessing the asset needs. If a player will interact with specific objects, keep those separate and combine the rest.
###  Using Instanced Static Meshes (ISMs)
For multiple instances of the same mesh, use **instanced static meshes (ISMs)**. ISMs guarantee that meshes are part of the same draw call. Learn more about ISMs and HISMs by reading the [Instanced Static Mesh Component](https://dev.epicgames.com/documentation/en-us/unreal-engine/instanced-static-mesh-component-in-unreal-engine?application_version=5.5) page of the Unreal Engine documentation.
You can also use the [Foliage tool](https://dev.epicgames.com/documentation/en-us/fortnite/foliage-mode-in-unreal-editor-for-fortnite) to paint instanced meshes into your scene.
Alternatively, you can use the [Scatter tool](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-tools-mode-in-fortnite#scatter) from the Fortnite Tools mode to create an ISM blueprint within your scene and provide you with placement tools.
###  Adjusting LODs and HLODs
Use the [Level of Detail (LOD)](https://dev.epicgames.com/documentation/en-us/fortnite/setting-the-level-of-detail-in-unreal-editor-for-fortnite) system to reduce vertex counts and draw calls for distant objects. Create and import custom LODs where you have combined material slots for higher LODs. This allows you to assign custom materials to higher LODs, which is useful for hero assets that need material flexibility close to the camera but not at a distance.
Auto-generated LODs maintain the material count through all levels, so you must import custom LODs to use this technique.
If streaming is enabled, use the [Hierarchical Level of Detail (HLOD)](https://dev.epicgames.com/documentation/en-us/fortnite/streaming-and-hlods-in-unreal-editor-for-fortnite) system to combine the meshes, materials, and textures of groups of distant objects.
###  Using Nanite
[Nanite](https://dev.epicgames.com/documentation/en-us/unreal-engine/nanite-in-unreal-engine) uses one draw call per material for all Nanite-enabled meshes in the scene, which can drastically reduce the number of draw calls.
To optimize for Nanite, reduce the number of unique materials in your scene. Because UEFN projects run on many platforms, you should follow standard optimization steps. Nanite-enabled sessions will receive additional benefits.
Nanite is only available on [supported platforms](https://dev.epicgames.com/documentation/en-us/unreal-engine/nanite-virtualized-geometry-in-unreal-engine?application_version=5.5#supported-platforms).

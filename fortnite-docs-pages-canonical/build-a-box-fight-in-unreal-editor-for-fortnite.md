## https://dev.epicgames.com/documentation/en-us/fortnite/build-a-box-fight-in-unreal-editor-for-fortnite



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
  4. Box Fight


# Box Fight
Learn to make this classic 1v1 building elimination game from start to finish using Unreal Editor for Fortnite! 
![Box Fight](https://dev.epicgames.com/community/api/documentation/image/e4a5c800-abac-4180-964b-56aaf6c55df3?resizing_type=fill&width=1920&height=335)
On this page
[Box Fight](https://dev.epicgames.com/documentation/en-us/fortnite-creative/design-a-1v1-box-fight-in-fortnite-creative) is a fast-paced shooting game where players try to eliminate each other in a very limited space. Barriers separate the players before the game starts, allowing each player to quickly build defenses before the barriers drop.
This tutorial provides a step-by-step guide to create a box fight game in **Unreal Editor for Fortnite (UEFN)**.
If this is your first time using UEFN, take a few minutes to familiarize yourself with basic [Controls](https://dev.epicgames.com/documentation/en-us/fortnite/guide-to-uefn-controls-for-creative-users-in-unreal-editor-for-fortnite) and [Outliner Tips](https://dev.epicgames.com/documentation/en-us/uefn/outliner-tips-and-tricks-in-unreal-editor-for-fortnite) in UEFN.
UEFN provides a way for you to playtest your level at any point in the production process. Click the **Launch Session** button located on the upper left side of your viewer:
[![launch session](https://dev.epicgames.com/community/api/documentation/image/fde9a011-3769-48e0-b2dd-a043c00f8cdf?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/fde9a011-3769-48e0-b2dd-a043c00f8cdf?resizing_type=fit)
Wait for the [FN client](https://www.fortnite.com/en-US/creative/docs/fortnite-creative-glossary#game-client) to load your level, then start testing.
##  Overview 
Here is an overview of the steps you'll need to recreate this island in the ideal sequence:
  * [![1. Set Up the Box Fight Game](https://dev.epicgames.com/community/api/documentation/image/21c8fa8b-c1da-4e10-a1f0-2cc64e5799bf?resizing_type=fit&width=640&height=640) 1. Set Up the Box Fight Game Create a new project and modify the Island Settings. ](https://dev.epicgames.com/documentation/en-us/fortnite/box-fight-1-set-up-the-game-in-unreal-editor-for-fortnite)
  * [![2. Build the Level](https://dev.epicgames.com/community/api/documentation/image/e0166bde-040d-4c52-988b-dec743b26ca3?resizing_type=fit&width=640&height=640) 2. Build the Level Build the area for your box fight, including a basement underneath the main playing area. ](https://dev.epicgames.com/documentation/en-us/fortnite/box-fight-2-build-the-level-in-unreal-editor-for-fortnite)
  * [![3. Add Devices](https://dev.epicgames.com/community/api/documentation/image/25334b81-f62c-4349-93f3-4c3428578ed6?resizing_type=fit&width=640&height=640) 3. Add Devices Add and configure the devices that make up the box fight. ](https://dev.epicgames.com/documentation/en-us/fortnite/box-fight-3-add-devices-in-unreal-editor-for-fortnite)
  * [![4. Link Devices](https://dev.epicgames.com/community/api/documentation/image/1997e8ec-6bde-4085-8494-ca4a777b6703?resizing_type=fit&width=640&height=640) 4. Link Devices Set up direct event binding or set up a Verse script to link the devices. ](https://dev.epicgames.com/documentation/en-us/fortnite/box-fight-4-link-devices-in-unreal-editor-for-fortnite)


You cannot skip a section if you want to build a working box fight game. Each step is essential to get to the final product!
  * [ gameplay](https://dev.epicgames.com/community/search?query=gameplay)
  * [ devices](https://dev.epicgames.com/community/search?query=devices)
  * [ box fight](https://dev.epicgames.com/community/search?query=box%20fight)
  * [ 1v1](https://dev.epicgames.com/community/search?query=1v1)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [ Overview ](https://dev.epicgames.com/documentation/en-us/fortnite/build-a-box-fight-in-unreal-editor-for-fortnite#overview)


Related documents
[ Island Settings in UEFN  ![Island Settings in UEFN](https://dev.epicgames.com/community/api/documentation/image/7ebe4678-7f4a-4b98-a445-73e75b54ec0a?resizing_type=fit&width=160&height=92) ](https://dev.epicgames.com/documentation/en-us/fortnite/island-settings-in-unreal-editor-for-fortnite)[ Outliner Tips and Tricks  ![Outliner Tips and Tricks](https://dev.epicgames.com/community/api/documentation/image/3f92777c-b562-408c-99bd-250f67a8db43?resizing_type=fit&width=160&height=92) ](https://dev.epicgames.com/documentation/en-us/fortnite/outliner-tips-and-tricks-in-unreal-editor-for-fortnite)[ Car Racing  ![Car Racing](https://dev.epicgames.com/community/api/documentation/image/05e312be-d514-4940-948a-7c5b008f6939?resizing_type=fit&width=160&height=92) ](https://dev.epicgames.com/documentation/en-us/fortnite/build-a-carracing-game-in-unreal-editor-for-fortnite)




---

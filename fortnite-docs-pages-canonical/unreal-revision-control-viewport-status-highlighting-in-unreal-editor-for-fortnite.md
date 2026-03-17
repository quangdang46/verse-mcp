## https://dev.epicgames.com/documentation/en-us/fortnite/unreal-revision-control-viewport-status-highlighting-in-unreal-editor-for-fortnite



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
  4. Unreal Revision Control Viewport Status Highlighting


# Unreal Revision Control Viewport Status Highlighting
Use status highlighting to see which assets are being edited by your colleagues. 
![Unreal Revision Control Viewport Status Highlighting](https://dev.epicgames.com/community/api/documentation/image/811b105d-b3ca-43c9-9930-578875c1bc75?resizing_type=fill&width=1920&height=335)
On this page
As a developer working with your collaborators, you want instant insight into how your project is evolving at a given time. Revision Control Status Highlighting provides a lens, directly in the viewport, into what your team members are working on, whether certain actors have newer versions, or which actors have been added or modified by you.
[![An example of status highlighting in red, green, and blue.](https://dev.epicgames.com/community/api/documentation/image/47eea085-e5fb-4856-8385-4f8618d9c5ed?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/47eea085-e5fb-4856-8385-4f8618d9c5ed?resizing_type=fit)
##  Default settings 
When you open a level, you will see a red highlight (the same color as the corresponding revision control icon) on actors if they are checked out by one of your team members. Hovering on the actor will display a tooltip with the username of the team member who has this actor checked out.
[![An example of an asset checked out my a teammate.](https://dev.epicgames.com/community/api/documentation/image/3084e8be-1dbc-43fd-a692-ec1bab2c7ae1?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/3084e8be-1dbc-43fd-a692-ec1bab2c7ae1?resizing_type=fit)
Yellow highlights indicate actors that have been updated in revisions you haven’t yet synced, giving you a sneak peek at what will change when you sync to the latest. When you see yellow highlights, you can either sync to the latest or simply avoid working on that asset to prevent losing any potential changes.
##  My changes 
If you want to visualize your own local changes within the viewport, you can turn on "✔ **Checked Out by Me** " or “+ **Newly Added** ” within the **Show Revision Control** settings dropdown to highlight actors you’ve checked out or those you’ve added but not yet submitted to revision control.
[![Status Highlights options.](https://dev.epicgames.com/community/api/documentation/image/3b09170c-b3b6-4789-b67b-bd55f034ee81?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/3b09170c-b3b6-4789-b67b-bd55f034ee81?resizing_type=fit)
##  Highlight visibility 
If all of these highlight colors are too distracting for you, you can disable/reduce their visibility in one of a few ways:
  1. Disable **GameView**(shortcut G)
  2. Turn off the status URC highlights by unchecking the checkbox or clicking the**Hide All** menu item.
  3. Select the actor you are interested in, as we do not render the highlight when you have the actor selected.
  4. Lower the **Opacity** input of the Status Highlights menu to make it less opaque within the viewport.


##  Filter actors in the outliner by URC status 
Highlighting and filtering by revision control status aren't just reserved for the viewport. You can also filter the outliner by Revision Control status. This can simplify reverting actors you do not want to submit or help with inspecting actors you’ve modified to make sure you’re happy with the changes.
[![Filter actors by URC status.](https://dev.epicgames.com/community/api/documentation/image/2d30cb6c-24aa-4807-9078-6724dcfd8792?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/2d30cb6c-24aa-4807-9078-6724dcfd8792?resizing_type=fit)
  * [ assets](https://dev.epicgames.com/community/search?query=assets)
  * [ projects](https://dev.epicgames.com/community/search?query=projects)
  * [ version control](https://dev.epicgames.com/community/search?query=version%20control)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [ Default settings ](https://dev.epicgames.com/documentation/en-us/fortnite/unreal-revision-control-viewport-status-highlighting-in-unreal-editor-for-fortnite#default-settings)
  * [ My changes ](https://dev.epicgames.com/documentation/en-us/fortnite/unreal-revision-control-viewport-status-highlighting-in-unreal-editor-for-fortnite#my-changes)
  * [ Highlight visibility ](https://dev.epicgames.com/documentation/en-us/fortnite/unreal-revision-control-viewport-status-highlighting-in-unreal-editor-for-fortnite#highlight-visibility)
  * [ Filter actors in the outliner by URC status ](https://dev.epicgames.com/documentation/en-us/fortnite/unreal-revision-control-viewport-status-highlighting-in-unreal-editor-for-fortnite#filter-actors-in-the-outliner-by-urc-status)






---

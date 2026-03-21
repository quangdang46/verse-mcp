## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block



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
  4. text_block class


# text_block class
Learn technical details about the text_block class. 
On this page
Text block widget. Displays text to the user.
|   
---|---  
Verse `using` statement | `using { /Fortnite.com/UI }`  
## Inheritance Hierarchy
This class is derived from the following hierarchy, starting with `widget`:
Name | Description  
---|---  
[`widget`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget) |  Base class for all UI elements drawn on the `player`'s screen.  
[`text_base`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base) |  Base widget for text widget.  
## Members
This class has both data members and functions.
### Data
Data Member Name | Type | Description  
---|---|---  
`AutoWrap` | `?logic` |  Whether the text should be automatically wrapped.  
`DefaultJustification` | `text_justification` |  The justification to display to the user. Used only during initialization of the widget and not modified by SetJustification.  
`DefaultOverflowPolicy` | `text_overflow_policy` |  The policy that determine what happens when the text is longer than its allowed length. Used only during initialization of the widget and not modified by SetOverflowPolicy.  
`DefaultShadowColor` | `color` |  The color of the shadow. Used only during initialization of the widget and not modified by SetShadowColor.  
`DefaultShadowOffset` | `?vector2` |  The direction the shadow is cast. Used only during initialization of the widget and not modified by SetShadowOffset.  
`DefaultShadowOpacity` | `float` |  The opacity of the shadow. Used only during initialization of the widget and not modified by SetShadowOpacity.  
`DefaultText` | `message` |  The text to display to the user. Used only during initialization of the widget and not modified by SetText.  
`DefaultTextColor` | `color` |  The color of the displayed text. Used only during initialization of the widget and not modified by SetTextColor.  
`DefaultTextOpacity` | `float` |  The opacity of the displayed text. Used only during initialization of the widget and not modified by SetTextOpacity.  
`DefaultTextSize` | `float` |  The size of the displayed text. Used only during initialization of the widget and not modified by SetTextSize.  
`WrappingPolicy` | `?text_wrapping_policy` |  The wrapping policy to determine where the line can be broken.  
`WrapWidth` | `?float` |  Whether text wraps onto a new line when it's length exceeds this width; if this value is zero or negative, no wrapping occurs.  
### Functions
Function Name | Description  
---|---  
[`GetJustification`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/getjustification) |  Gets the text justification in the widget.  
[`GetOverflowPolicy`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/getoverflowpolicy) |  Gets the policy that determine what happens when the text is longer than its allowed length.  
[`GetParentWidget`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget/getparentwidget) |  Returns the `widget`'s parent `widget`. Fails if no parent exists, such as if this `widget` is not in the `player_ui` or is itself the root `widget`.  
[`GetRootWidget`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget/getrootwidget) |  Returns the `widget` that added this `widget` to the `player_ui`. The root `widget` will return itself. Fails if this `widget` is not in the `player_ui`.  
[`GetShadowColor`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block/getshadowcolor) |  Gets the color of the shadow.  
[`GetShadowOffset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block/getshadowoffset) |  Gets the direction the shadow is cast.  
[`GetShadowOpacity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block/getshadowopacity) |  Gets the opacity of the shadow.  
[`GetText`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/gettext) |  Gets the text currently in the widget.  
[`GetTextColor`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/gettextcolor) |  Gets the color of the displayed text.  
[`GetTextOpacity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/gettextopacity) |  Gets the opacity of the displayed text.  
[`GetTextSize`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/gettextsize) |  Gets the size of the displayed text.  
[`GetVisibility`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget/getvisibility) |  Returns the current `widget_visibility` state.  
[`IsEnabled`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget/isenabled) |  `true` if this `widget` can be modified interactively by the player.  
[`SetEnabled`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget/setenabled) |  Enables or disables whether the `player` can interact with this `widget`.  
[`SetJustification`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/setjustification) |  Sets the text justification in the widget.  
[`SetOverflowPolicy`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/setoverflowpolicy) |  Sets the policy that determine what happens when the text is longer than its allowed length.  
[`SetShadowColor`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block/setshadowcolor) |  Sets the color of the shadow.  
[`SetShadowOffset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block/setshadowoffset) |  Sets the direction the shadow is cast.  
[`SetShadowOpacity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block/setshadowopacity) |  Sets the opacity of the shadow.  
[`SetText`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/settext) |  Sets the text displayed in the widget.  
[`SetTextColor`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/settextcolor) |  Sets the color of the displayed text.  
[`SetTextOpacity`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/settextopacity) |  Sets the opacity of the displayed text.  
[`SetTextSize`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/text_base/settextsize) |  Sets the size of the displayed text.  
[`SetVisibility`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/temporary/ui/widget/setvisibility) |  Shows or hides the `widget` without removing itself from the containing `player_ui`. See `widget_visibility` for details.  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ class](https://dev.epicgames.com/community/search?query=class)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Inheritance Hierarchy](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block#inheritancehierarchy)
  * [Members](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block#members)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block#data)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/fortnitedotcom/ui/text_block#functions)





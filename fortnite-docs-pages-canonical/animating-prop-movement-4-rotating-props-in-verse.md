## https://dev.epicgames.com/documentation/en-us/fortnite/animating-prop-movement-4-rotating-props-in-verse

# 4. Rotating Props
Learn how to rotate obstacles on your course Fall Guys course.
![4. Rotating Props](https://dev.epicgames.com/community/api/documentation/image/cd3adcbf-804b-4550-b0b0-0f74a96deec7?resizing_type=fill&width=1920&height=335)
Another platforming obstacle common to the genre is objects that rotate, such as a spinning platform you have to constantly move on, or a bar that moves back and forth that you have to jump over.
The second component of an object’s transform is its rotation, and you can manipulate an object to rotate around an axis. There are several different ways you can use rotation to make unique platforming challenges, and you’ll learn how to code them in this section.
##  Making Props that Rotate
Follow these steps to build code that rotates your props:
  1. Create a new Verse class named `rotating_prop` that inherits from `movable_prop` using **Verse Explorer**. Add the `<concrete>` specifier to this class to expose its properties to UEFN.
Verse
```

```

# A prop that rotates by an additional rotation or rotates to match # a Creative prop&#39;s rotation. rotating_prop&lt;public&gt; := class&lt;concrete&gt;(movable_prop):
Copy full snippet(3 lines long)
  2. Add the `using { /Fortnite.com/Devices/CreativeAnimation }` and `using { /UnrealEngine.com/Temporary/SpatialMath }` statements to the top of the file to import these modules. You’ll need these to animate your prop. The tooltips used in this section are also included here.
Verse
```
     using { /Fortnite.com/Devices }
     using { /Fortnite.com/Devices/CreativeAnimation }
     using { /Verse.org/Simulation }
     using { /UnrealEngine.com/Temporary/SpatialMath }

     AdditionalRotationTip<localizes>:message = "The rotation to apply to the RootProp."
     ShouldRotateForeverTip<localizes>:message = "Whether the RootProp should rotate forever."
     MatchRotationTargetTip<localizes>:message = "The optional prop whose rotation the RootProp should rotate to. Use this if you do not want to set an Additional Rotation."

     # A prop that rotates by an additional rotation or rotates to match

```

Copy full snippet(12 lines long)
  3. At the top of the `rotating_prop` class definition, add the following fields.
    1. An editable `rotation` named `AdditionalRotation`. This is the rotation to apply to the prop. After `Move()` completes, the prop’s rotation will be offset by this value.
Verse
```

```

# The additional rotation to apply to the RootProp. @editable {ToolTip := AdditionalRotationTip} AdditionalRotation:rotation = rotation{}
Copy full snippet(3 lines long)
    2. An editable `logic` named `ShouldRotateForever`. This specifies whether the prop should keep rotating without resetting.
Verse
```

```

# Whether the RootProp should rotate forever. @editable {ToolTip := ShouldRotateForeverTip} ShouldRotateForever:logic = true
Copy full snippet(3 lines long)
    3. An editable optional `creative_prop` named `MatchRotationTarget`. If you want your prop to rotate to match another prop’s rotation, you can set this value rather than using the `AdditionalRotation`.
Verse
```

```

# The optional prop whose rotation RootProp should rotate to match. Use this if you # do not want to set an additional rotation. @editable {ToolTip := MatchRotationTargetTip} MatchRotationTarget:?creative_prop = false
Copy full snippet(4 lines long)
    4. A variable `rotation` named `TargetRotation`. This is the rotation the prop is currently rotating toward.
Verse
```

```

# The rotation the prop is currently rotating toward. var TargetRotation:rotation = rotation{}
Copy full snippet(2 lines long)
  4. Your final class definition should look like this:
Verse
```
     using { /Fortnite.com/Devices }
     using { /Fortnite.com/Devices/CreativeAnimation }
     using { /Verse.org/Simulation }
     using { /UnrealEngine.com/Temporary/SpatialMath }

     AdditionalRotationTip<localizes>:message = "The rotation to apply to the RootProp."
     ShouldRotateForeverTip<localizes>:message = "Whether the RootProp should rotate forever."
     MatchRotationTargetTip<localizes>:message = "The optional prop whose rotation the RootProp should rotate to. Use this if you do not want to set an Additional Rotation."

     # A prop that rotates by an additional rotation or rotates to match

```

Copy full snippet(28 lines long)
  5. Since you already set up the `Move()` function that moves your prop in `movable_prop`, you can override it in this class. Override the `Move()` function in your `rotating_prop` class. In `Move()`, first, check if the `MatchRotationTarget` is set and save it in a variable `RotationToMatch`. If so, set the `TargetRotation` to the `RotationToMatch`. Otherwise, set it to the `AdditionalRotation`.
Verse
```

```

# Rotate the RootProp by applying the TargetRotation, or toward the MoveTarget if one is set. Move&lt;override&gt;()&lt;suspends&gt;:void= # Set the TargetRotation to the MoveTarget&#39;s rotation if the MoveTarget is set. if: RotationToMatch := MatchRotationTarget?.GetTransform().Rotation then: set TargetRotation = RotationToMatch else: set TargetRotation = AdditionalRotation
Copy full snippet(10 lines long)
  6. As with `translating_prop`, specify the animation mode for your animation to play. Initialize a new `animation_mode` variable named `AnimationMode` to `animation_mode.OneShot`. This means your animation will stop once your object reaches its target. If the prop should not rotate forever or not move once and stop, set the animation mode to ping pong. Using ping pong will let you make objects that oscillate back and forth, like the bar on a metronome or a bridge that raises and lowers.
Verse
```

```

# Set the default animation mode to play. # The OneShot animation mode will play the animation once. var AnimationMode:animation_mode := animation_mode.OneShot # If the RootProp should not reset and not stop when it finishes rotating, # set the animation mode to PingPong. if: not ShouldRotateForever? and not MoveOnceAndStop? then: set AnimationMode = animation_mode.PingPong
Copy full snippet(10 lines long)
If you set `ShouldReset` to **false** and `ShouldRotateForever` to **true** , your prop should keep its position after each animation while continuing to loop `Move()`.
  7. Get the rotation for your root prop to rotate toward in a new variable named `RotateByMoveRotation` by calling `RotateBy()` on the `StartingTransform`, passing the `TargetRotation`.Then call `MoveToEase()`. Your complete `Move()` function should look like this.
Verse
```
     # Rotate the RootProp by applying the TargetRotation, or toward the MoveTarget if one is set.
     Move<override>()<suspends>:void=

         # Set the TargetRotation to the MoveTarget's rotation if the MoveTarget is set.
         if:
             RotationToMatch := MatchRotationTarget?.GetTransform().Rotation
         then:
             set TargetRotation = RotationToMatch
         else:
             set TargetRotation = AdditionalRotation

```

Copy full snippet(24 lines long)
  8. In your `prop_animator` device class, add a new editable array of `rotating_prop` named `RotatingProps`. Add another `for` expression to `OnBegin()` that loops through all the rotating props and calls `Setup()` on them. Your updated `prop_animator` class should look like this:
Verse
```
     using { /Fortnite.com/Devices }
     using { /Verse.org/Simulation }
     using { /UnrealEngine.com/Temporary/Diagnostics }

     TranslatingPropsTip<localizes>:message = "The props that translate (move) using animation."
     RotatingPropsTip<localizes>:message = "The props that rotate using animation."

     # Coordinates moving props through animation by calling each prop's Setup() method.
     prop_animator := class(creative_device):

```

Copy full snippet(29 lines long)
  9. Save your code and compile it.

##  Linking Props to Devices
Back in the editor, delete some of the props after the translating props section to create another gap. Add a **FG01 Turntable base** and a **FG01 SpinningBar Double S** to your level. Name the base **RotatingBase** and the bar **Spinning Bar**. Position the bar above the base, and place both props above the gap.
[![Rotating Props Setup](https://dev.epicgames.com/community/api/documentation/image/20ffcd21-a17f-4fc9-acd5-3490e4d9232b?resizing_type=fit)](https://dev.epicgames.com/community/api/documentation/image/20ffcd21-a17f-4fc9-acd5-3490e4d9232b?resizing_type=fit)
_Setup of the two rotating props. Both the spinning bar and rotating base spin in the same direction at different speeds._
Select your **prop animator** in the **Outliner** , and add an array element to `RotatingProps` for each of your rotating props. Assign each prop with the following values:
Option  |  Value  |  Explanation
---|---|---
**Additional Rotation** |  Z, 90.0 |  This prop wsill make a 90-degree rotation around the Z-axis each time.
**RootProp** |  Assign to prop you’re animating |  This is the prop you’re animating.
**Move Duration** |  2.0, 3.0 |  Assign one of the props a shorter duration so that they rotate at different speeds.
**Move Ease Type** |  Linear |  This will animate your props at a consistent speed.
Push your changes, then check out your props! Try varying the different values to get different rotations, and try rotating in each of the different dimensions to create different types of obstacles.
##  Next Up
In the next section, you’ll combine movement and rotation to create props that can do both!
  * [![5. Scaling Props](https://dev.epicgames.com/community/api/documentation/image/eb32ad64-f177-436e-9aff-093b15e3ea5b?resizing_type=fit&width=640&height=640) 5. Scaling Props Learn how to manipulate object scales with Verse so they grow and shrink. ](https://dev.epicgames.com/documentation/en-us/fortnite/animating-prop-movement-5-scaling-props-in-verse)

##  Complete Code
Here is the complete code built in this section:
###  rotating_prop.verse
Verse
```
using { /Fortnite.com/Devices }
using { /Fortnite.com/Devices/CreativeAnimation }
using { /Verse.org/Simulation }
using { /UnrealEngine.com/Temporary/SpatialMath }

AdditionalRotationTip<localizes>:message = "The rotation to apply to the RootProp."
ShouldRotateForeverTip<localizes>:message = "Whether the RootProp should rotate forever."
MatchRotationTargetTip<localizes>:message = "The optional prop whose rotation the RootProp should rotate to. Use this if you do not want to set an Additional Rotation."

# A prop that rotates by an additional rotation or rotates to match

```

Copy full snippet(54 lines long)
###  prop_animator.verse
Verse
```
using { /Fortnite.com/Devices }
using { /Verse.org/Simulation }
using { /UnrealEngine.com/Temporary/Diagnostics }

TranslatingPropsTip<localizes>:message = "The props that translate (move) using animation."
RotatingPropsTip<localizes>:message = "The props that rotate using animation."

# Coordinates moving props through animation by calling each prop's Setup() method.
prop_animator := class(creative_device):

```

Copy full snippet(30 lines long)

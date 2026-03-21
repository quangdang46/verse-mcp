## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse



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
  4. Verse module


# Verse module
Learn technical details about the Verse module. 
On this page
  * [`Verse.org`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg)
  * **`Verse`**


## Classes and Structs
Name | Description  
---|---  
[`classifiable_subset(element_type)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/classifiable_subset/classifiable_subset\(element_type\)) |  A `classifiable_subset` is a container that holds a set of elements. A classifiable_subset can hold multiple elements of the same type.  
[`diagnostic`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/diagnostic) |  An opaque diagnostic message that only shows up in diagnostic logs. The format of the diagnostic may change at any time without warning and may not be inspected by Verse code.  
[`event(t)`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event/event\(t\)) |  A _recurring_ , successively signaled parametric `event` with a `payload` allowing a simple mechanism to coordinate between concurrent tasks.  
[`locale`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/locale) |  Used for message localization.  
[`message`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/message) |  A localizable text message.  
## Interfaces
Name | Description  
---|---  
[`cancelable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cancelable) |  Implemented by classes that allow users to cancel an operation. For example, calling `subscribable.Subscribe` with a callback returns a `cancelable` object. Calling `Cancel` on the return object unsubscribes the callback.  
[`disposable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/disposable) |  Implemented by classes whose instances have limited lifetimes.  
[`enableable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/enableable) |  Implemented by classes whose instances can be enabled and disabled.  
[`invalidatable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/invalidatable) |  Implemented by classes whose instances can become invalid at runtime.  
[`showable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/showable) |  Implemented by classes whose instances can change visibility to be shown or hidden.  
## Functions
Name | Description  
---|---  
[`operator'='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorequals) |   
[`operator'<>'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorlessgreater) |   
[`prefix'-'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/prefixminus) |   
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus) |   
[`operator'-'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorminus) |   
[`operator'*'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorstar) |   
[`operator'/'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorslash) |   
[`operator'+='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplusequals) |   
[`operator'-='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorminusequals) |   
[`operator'*='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorstarequals) |   
[`Abs`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/abs) |   
[`operator'>'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater) |   
[`operator'>='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreaterequals) |   
[`operator'<'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorless) |   
[`operator'<='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorlessequals) |   
[`Ceil`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/ceil) |   
[`Floor`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/floor) |   
[`prefix'-'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/prefixminus-1) |   
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus-1) |   
[`operator'-'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorminus-1) |   
[`operator'*'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorstar-1) |   
[`operator'/'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorslash-1) |   
[`operator'+='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplusequals-1) |   
[`operator'-='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorminusequals-1) |   
[`operator'*='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorstarequals-1) |   
[`operator'/='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator/equals) |   
[`Abs`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/abs-1) |   
[`operator'*'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorstar-2) |   
[`operator'*'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorstar-3) |   
[`operator'>'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreater-1) |   
[`operator'>='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorgreaterequals-1) |   
[`operator'<'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorless-1) |   
[`operator'<='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorlessequals-1) |   
[`operator'?'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorquestionmark) |   
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus-2) |   
[`operator'+='`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplusequals-2) |   
[`operator'()'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator\(\)) |   
[`operator'()'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator\(\)-1) |   
[`operator'()'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator\(\)-2) |   
[`ConcatenateMaps`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/concatenatemaps) |   
[`operator'()'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator\(\)-3) |   
[`operator'()'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operator\(\)-4) |   
[`weak_map`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/weak_map) |   
[`operator'?'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorquestionmark-1) |   
[`FitsInPlayerMap`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/fitsinplayermap) |   
[`Print`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/print) |  Writes `Message` to a dedicated `Print` log while displaying it in `Color` on the client screen for `Duration` seconds. By default, `Color` is `NamedColors.White` and `Duration` is `2.0` seconds.  
[`Print`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/print-1) |  Writes `Message` to a dedicated `Print` log while displaying it in `Color` on the client screen for `Duration` seconds. By default, `Color` is `NamedColors.White` and `Duration` is `2.0` seconds.  
[`Print`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/print-2) |  Writes `Message` to a dedicated `Print` log while displaying it in `Color` on the client screen for `Duration` seconds. By default, `Color` is `NamedColors.White` and `Duration` is `2.0` seconds.  
[`Concatenate`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/concatenate) |  Makes a flattened `array` by concatenating the elements of `Arrays`.  
[`classifiable_subset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/classifiable_subset) |  A `classifiable_subset` is a container that holds a set of elements. A classifiable_subset can hold multiple elements of the same type.  
[`MakeClassifiableSubset`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/makeclassifiablesubset) |  Constructs a `classifiable_subset` containing the `InElements`.  
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus-3) |  Returns a new set that is the union of all elements in `InSetL` set and `InSetR`.  
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus-4) |  Concatenates two diagnostic messages.  
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus-5) |  Concatenates a diagnostic message with a normal string, yielding a diagnostic message.  
[`operator'+'`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/operatorplus-6) |  Concatenates a normal string with a diagnostic message, yielding a diagnostic message.  
[`ToDiagnostic`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/todiagnostic) |  Converts any Verse value into an opaque diagnostic message.  
[`Err`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/err) |  Halts the Verse runtime with error `Message`.  
[`event`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event) |  A _recurring_ , successively signaled parametric `event` with a `payload` allowing a simple mechanism to coordinate between concurrent tasks.  
[`event`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/event-1) |  A _recurring_ , successively signaled event allowing a simple mechanism to coordinate between concurrent tasks.  
[`Ceil`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/ceil-1) |  Returns the smallest `int` that is greater than or equal to `Val`. Fails if `not IsFinite(Val)`.  
[`Floor`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/floor-1) |  Returns the largest `int` that is less than or equal to `Val`. Fails if `not IsFinite(Val)`.  
[`Round`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/round) |  Returns `Val` rounded to the nearest `int`. When the fractional part of `Val` is `0.5`, rounds to the nearest _even_ `int` (per the IEEE-754 default rounding mode). Fails if `not IsFinite(Val)`.  
[`Int`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/int) |  Returns the `int` that equals `Val` without the fractional part. Fails if `not IsFinite(val)`.  
[`ToString`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/tostring) |  Makes a `string` representation of `Val`.  
[`GetSecondsSinceEpoch`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/getsecondssinceepoch) |  Returns the number of seconds since January 1, 1970 UTC, ignoring leap seconds. I.e, this function implements Unix time. This function always returns the same value within the same transaction.  
[`ToString`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/tostring-1) |  Makes a printable `string` representation of `Val`.  
[`listenable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/listenable) |  A parametric interface combining `awaitable` and `subscribable`.  
[`listenable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/listenable-1) |  A parameterless interface combining `awaitable` and `subscribable`.  
[`Localize`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/localize) |  Makes a `string` by localizing `Message` based on the current `locale`.  
[`Clamp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp) |  Constrains the value of `Val` between `A` and `B`. Robustly handles different argument orderings. Returns the median of `Val`, `A`, and `B`, such that comparisons with `NaN` operate as if `NaN > +Inf`.  
[`Clamp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/clamp-1) |  Constrains the value of `Val` between `A` and `B`. Robustly handles different argument orderings. Returns the median of `Val`, `A`, and `B`.  
[`Min`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/min) |  Returns the minimum of `X` and `Y`.  
[`Max`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/max) |  Returns the maximum of `X` and `Y`.  
[`Min`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/min-1) |  Returns the minimum of `X` and `Y` unless either are `NaN`. Returns `NaN` if either `X` or `Y` are `NaN`.  
[`Max`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/max-1) |  Returns the maximum of `X` and `Y` unless either are `NaN`. Returns `NaN` if either `X` or `Y` are `NaN`.  
[`Sqrt`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/sqrt) |  Returns the square root of `X` if `X >= 0.0`. Returns `NaN` if `X < 0.0`.  
[`Sin`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/sin) |  Returns the sine of `X`, where `X` is interpreted as a value in radians, if `IsFinite[X]`. Returns `NaN` if `not IsFinite[X]`.  
[`Cos`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cos) |  Returns the cosine of `X`, where `X` is interpreted as a value in radians, if `IsFinite[X]`. Returns `NaN` if `not IsFinite[X]`.  
[`Tan`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/tan) |  Returns the tangent of `X`, where `X` is interpreted as a value in radians, if `IsFinite[X]`. Returns `NaN` if `not IsFinite[X]`.  
[`ArcSin`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arcsin) |  Returns the inverse sine (arcsine) of `X` as a value in radians if `-1.0 <= X <= 1.0`.  
[`ArcCos`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arccos) |  Returns the inverse cosine (arccosine) of `X` as a value in radians if `-1.0 <= X <= 1.0`.  
[`ArcTan`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arctan) |  Returns the inverse tangent (arctangent) of `X` as a value in radians such that:`-PiFloat/2.0 <= ArcTan(x) <= PiFloat/2.0`.  
[`ArcTan`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arctan-1) |  Returns the angle in radians at the origin between a ray pointing to `(X, Y)` and the positive `X` axis such that `-PiFloat < ArcTan(Y, X) <= PiFloat`. Returns `0.0` if `X=0.0 and Y=0.0`.  
[`Sinh`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/sinh) |  Returns the hyperbolic sine of `X`.  
[`Cosh`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/cosh) |  Returns the hyperbolic cosine of `X`.  
[`Tanh`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/tanh) |  Returns the hyperbolic tangent of `X`.  
[`ArSinh`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arsinh) |  Returns the inverse hyperbolic sine of `X` if `IsFinite(X)`.  
[`ArCosh`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/arcosh) |  Returns the inverse hyperbolic cosine of `X` if `1.0 <= X`.  
[`ArTanh`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/artanh) |  Returns the inverse hyperbolic tangent of `X` if `IsFinite(X)`.  
[`Pow`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/pow) |  Returns `A` to the power of `B`.  
[`Quotient`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/quotient) |  Returns the quotient `X/Y` as defined by Euclidean division, i.e.:
  * `Quotient[X/Y] = Floor[X/Y]` when `Y > 0`
  * `Quotient[X/Y] = Ceil[X/Y]` when `Y < 0`
  * `Quotient[X/Y] * Y + Mod[X,Y] = X` Fails if `Y = 0`.

  
[`Mod`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/mod) |  Returns the remainder of `X/Y` as defined by Euclidean division, i.e.:
  * `Mod[X,Y] = X - Quotient(X/Y)*Y`
  * `0 <= Mod[X,Y] < Abs(Y)` Fails if `Y=0`.

  
[`Exp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/exp) |  Returns the natural exponent of `X`.  
[`Ln`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/ln) |  Returns the natural logarithm of `X`.  
[`Log`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/log) |  Returns the base `B` logarithm of `X`.  
[`Lerp`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/lerp) |  Used to linearly interpolate/extrapolate between `From` (when `Parameter = 0.0`) and `To` (when `Parameter = 1.0`). Expects that all arguments are finite. Returns `From*(1 - Parameter) + To*Parameter`.  
[`Sgn`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/sgn) |  Returns the sign of `Val`:
  * `1` if `Val > 0`
  * `0` if `Val = 0`
  * `-1` if `Val < 0`

  
[`Sgn`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/sgn-1) |  Returns the sign of `Val`:
  * `1.0` if `Val > 0.0`
  * `0.0` if `Val = 0.0`
  * `-1.0` if `Val < 0.0`
  * `NaN` if `Val = NaN`

  
[`IsAlmostEqual`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/isalmostequal) |  Succeeds if `Val1` and `Val2` are within `AbsoluteTolerance` of each other.  
[`result`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/result) |  Implemented by classes that provide a result for an operation, which can fail or be successful  
[`MakeSuccess`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/makesuccess) |   
[`MakeError`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/makeerror) |   
[`signalable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/signalable) |  A parametric interface implemented by events with a `payload` that can be signaled. Can be used with `awaitable`, `subscribable`, or both (see: `listenable`).  
[`Join`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/join) |  Makes a `string` by concatenating `Separator` between the elements of `Strings`.  
[`ToString`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/tostring-2) |  Returns `String` without modification.  
[`ToString`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/tostring-3) |  Makes a `string` from `Character`.  
[`subscribable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/subscribable) |  A parametric interface implemented by events with a `payload` that can be subscribed to. Matched with `signalable.`  
[`subscribable`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse/subscribable-1) |  A parameterless interface implemented by events that can be subscribed to.  
## Data
Name | Description  
---|---  
`Inf` |   
`NaN` |   
---|---  
`PiFloat` |   
---|---  
  * [ api](https://dev.epicgames.com/community/search?query=api)
  * [ module](https://dev.epicgames.com/community/search?query=module)


* * *
[Developer Forums](https://forums.unrealengine.com/categories?tag=fortnite)
[Learning Library](https://dev.epicgames.com/community/fortnite/learning)
On this page
  * [Classes and Structs](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse#classesandstructs)
  * [Interfaces](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse#interfaces)
  * [Functions](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse#functions)
  * [Data](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/versedotorg/verse#data)





## https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/webapi

# WebAPI module
Learn technical details about the WebAPI module.
  * [`UnrealEngine.com`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom)
  * **`WebAPI`**

## Classes and Structs
Name | Description
---|---
[`client_id`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/webapi/client_id) |  Usage: Licensed users create a derived version of `client_id` in their module. The Verse class path for your derived `client_id` is then used as the configuration key in your backend service to map to your endpoint. WARNING: do not make your derived `client_id` class public. This object type is your private key to your backend. Example: my_client_id := class(client_id) MyClient := MakeClient(my_client_id)
[`client`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/webapi/client) |
[`response`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/webapi/response) |
[`body_response`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/webapi/body_response) |
## Functions
Name | Description
---|---
[`MakeClient`](https://dev.epicgames.com/documentation/en-us/fortnite/verse-api/unrealenginedotcom/webapi/makeclient) |

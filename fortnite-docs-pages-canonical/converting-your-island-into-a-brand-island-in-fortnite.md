## https://dev.epicgames.com/documentation/en-us/fortnite/converting-your-island-into-a-brand-island-in-fortnite

# Converting Your Island Into a Brand Island
Learn how to convert your island into a brand island.
![Converting Your Island Into a Brand Island](https://dev.epicgames.com/community/api/documentation/image/c7d918b4-b5cc-42ad-b3a8-a26820c5b81a?resizing_type=fill&width=1920&height=335)
You can convert your existing island into a [brand island](https://dev.epicgames.com/documentation/en-us/fortnite/fortnite-glossary#brand-island) in Unreal Editor for Fortnite (UEFN)! Think your island’s gameplay is perfect for reintroducing to players with the flair of a specific brand? This page covers conditions, process, and further questions for converting islands.
##  Conditions
To convert an island, the following conditions apply:
  * You must already be a member of the Fortnite Developer Program.
  * You must already have signed the IP Partner Licensing Agreement.
  * Your island must pass moderation and comply with the brand rules for the brand game collection that you want to convert to.
  * Your island must be created in UEFN, and you must be the owner of it.
  * Once your island is converted and published, it must stay as the new brand island for at least 30 days before you can revert it back to its former state or into another brand island.

Epic reserves the right to turn off this feature at any point. If that happens, Epic Games will provide instructions for how to revert your island.
##  Convert Your Island
To convert your island into a particular brand island:
  1. In UEFN, open the project that you want to convert, and go to **Menu** > **Project** > **Project Settings**.
  2. In the **Project Settings** tab, go to **Brand Selection** , and update the **Selected Brand** dropdown to the brand that you want to convert your island into. In the following pop-up, confirm that you want to proceed.
  3. Reload the project. In UEFN, the **Content Browser** should now show the game feature set and assets for the brand.
  4. Add or remove assets to match the newly selected brand so your island can pass validation.
If you are converting to or from LEGO®, Fall Guys, or Rocket Racing there are extra steps here. See [How do I convert my island to a LEGO, Rocket Racing, or Fall Guys island?](https://dev.epicgames.com/documentation/en-us/fortnite/converting-your-island-into-a-brand-island-in-fortnite#how-do-i-convert-my-island-to-a-lego-rocket-racing-or-fall-guys-island)
  5. In the navigation bar, navigate to **Launch Session** , and click the three vertical dots menu icon ( ⋮ ). Under **Sync** , click **Validate Project**.
  6. Push the updated project for moderation and publishing.

For example, if you want to convert from The Walking Dead Universe to Squid Game, remove the Walker NPC spawner and add Squid Game assets like the Llama piggybank.
##  Q&A
###  Can I convert my island again?
Yes. Once the mandatory 30 days have passed after converting and publishing your island, you can convert it again to a different brand or to use no brand at all.
When converting into a different brand, make sure that your island now adheres to the rules for that brand.
###  How is my converted island moderated?
Once you’ve adjusted your island to a brand and pushed your project for moderation, the island is moderated against the brand rules for the target brand, including link and project metadata. Check the brand rules for tagging, media, thumbnails, and other moderation elements.
###  What happens to engagement payouts when I convert my island?
When you convert your island to an IP brand island and publish your island, your Engagement Payouts will be adjusted to match the [brand’s payout scheme](https://create.fortnite.com/news/ip-partner-fee-schedule). On the actual day you convert and publish your island, the payout will match the payout scheme that your island belongs to for the majority of the day, measured in total minutes.
###  What happens to my island in Discover?
Your island goes live in the particular brand’s Discover section as soon as it goes live after moderation. If you apply for conversion within a publishing hold, the most recently published version of the island remains in Discover until the publishing hold lifts.
###  How do I convert my island to a LEGO, Rocket Racing, or Fall Guys island?
Converting to these brand islands requires a manual setup of specific project components. To convert away from any of the brands listed below, refer to the [Converting away from a Brand Island](https://dev.epicgames.com/documentation/en-us/fortnite/converting-your-island-into-a-brand-island-in-fortnite#converting-away-from-a-brand-island) section.
####  LEGO® Islands
To convert to a **LEGO** Island, delete the **IslandSettings0** device in the **Outliner** and replace it with the **Device Experience Settings Lime** blueprint from the **Content Browser**.
####  Rocket Racing Islands
To convert to a **Rocket Racing** Island, add the following assets from the **Content Browser** :
  * RR Track
  * RR Checkpoint(s)
  * RR Competitive Race Manager
  * 12 RR Start Positions

####  Fall Guys Islands
To convert to a **Fall Guys** Island, delete the **IslandSettings0** device in the **Outliner** and replace it with the **IslandSettingsFallGuys** blueprint from the **Content Browser**.
####  Converting away from a Brand Island
If you need to convert **away** from any of the branded island types above:
  * In the **Outliner** , add an **IslandSettings** blueprint from the**Content Browser**.

###  Where should I report any issues related to Island Conversion?
If you experience an issue with converting your island, please make a post on the forums and include a description. Describe the actions you took, and use visuals if possible.
Our community team regularly reviews the forums and escalates issues to the appropriate team to triage and start resolving them. Thank you for raising these issues as they happen and helping us improve these tools!

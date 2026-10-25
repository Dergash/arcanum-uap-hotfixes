# Version 1.1.0

## [Roseborough/Ring of Brodgar dialogue is inaccessible for Virgil and Z'an Al'urin](https://github.com/Dergash/arcanum-uap-hotfixes/issues/5)
Commits: [f7d968d](https://github.com/Dergash/arcanum-uap-hotfixes/commit/f7d968d383f4a34a84bc9491c580ad06176bc294)

Both Virgil and Z'an Al'uring have voiced travelogue about Ring of Brodgar.  
At certain point during development the Ring of Brodgar was located in the area of Ancient Ruins (1755W 1480S).  
Final game has the Ring of Brodgar made part of Roseborough, but the dialogue condition was never amended to reflect that change.
Combined with `ia` engine bug, it made those dialogues are inaccessible.

Now you can ask Virgil and Z'an Al'urin about current location when in Roseborough to hear this restored dialogue.

## [Torian Kel has inaccessible dialogue for Hardin's Pass](https://github.com/Dergash/arcanum-uap-hotfixes/issues/6)
Commits: [93c98d9](https://github.com/Dergash/arcanum-uap-hotfixes/commit/93c98d9a352251545a3e5e70aeffd5bc9a2a43aa)

Torian Kel's voiced travelogue for Hardin's Pass was previously inaccessible because landing point in `mes/gamearea.mes` was a tiny bit outside of the Hardin's Pass town map.  

I've moved the landing point a bit to restore his dialogue there.  

## [Murgo has inaccessible dialogue for Torin Quarry](https://github.com/Dergash/arcanum-uap-hotfixes/issues/8)
Commits: [ad0b585](https://github.com/Dergash/arcanum-uap-hotfixes/commit/ad0b585c41d2bbc084eaa29b650ce5e89d72b480)

The travelogue for Murgo was inaccessible because Torin Quarry has lacked a Town Map. Apparently it was planned at some point, as the list of Town Maps in vanilla game does include the record for Torin Quarry. 

I've generated one (a small one, two sectors) to restore his dialogue.

## [Magnus offensive introduction makes him permanently unavailable as a companion](https://github.com/Dergash/arcanum-uap-hotfixes/issues/4)
Commits: [c7ff030](https://github.com/Dergash/arcanum-uap-hotfixes/commit/c7ff030c4a6cd885406dfb0e69260d56d9265abd)

If Magnus reaction is low during introduction dialogue and you use polite branch, but you exit before asking his name, he is permanently lost as a companion - talking with him again only leads to Exit option.

Raising reaction afterwards (i.e. via Beauty increase) doesn't help.

This was happening because of two things:
- couple of exit dialogue options were lacking condition `me 1` checks
- another exit option was not seeting local variable that would allow to resume conversation later

Now conversation should no longer lead to dead end.

## [Dumb characters can't get Spot Traps expert training with Vegards MoltenFlow](https://github.com/Dergash/arcanum-uap-hotfixes/issues/12)
Commits: [a6043ab](https://github.com/Dergash/arcanum-uap-hotfixes/commit/a6043ab97671f0fd2761ea4bb552d52e6fe8ab6f)

A `$` symbol was missing from the payment line, now he trains dumb characters successfully.

Thanks [@barbarbrick](https://www.flickr.com/people/163554572@N05/)!

## [Great Helm erroneously gives AC:18 instead of stated AC:3 DR:7](https://github.com/Dergash/arcanum-uap-hotfixes/issues/9)
Commits: [9356e8e](https://github.com/Dergash/arcanum-uap-hotfixes/commit/9356e8e3fcf69b9e431ea7d5d98499cd890292f4)

When deciding between the wrong description and the wrong item parameters, I've decided that the fault lies with actual parameters, AC:18 has to be wrong.

It seems the intended progression of helms in vanilla game was:
- Helmet (ac:2 dr:5 PE:-1)
- Great Helm (ac:3 dr:7 PE:-1)
- Helm of Vision (ac:3 dr:7, crafted from Great Helm)
- Goggled Helmet (ac:6 dr:9 PE:+3, crafted from Helm of Vision)
  - or Miner's Helmet (ac: 15, light bonus, crafted from Helm of Vision)

But as it was, Great Helm was much more useful than Helm of Vision for all characters and a little bit more useful that Miner's Helmet for melee experts.

I've changed Great Helm prototype to provide AC:3 DR:7 as per item description.

## [Morning Star Perfume doesn't work](https://github.com/Dergash/arcanum-uap-hotfixes/issues/10)
Commits: [00cc4cf](https://github.com/Dergash/arcanum-uap-hotfixes/commit/00cc4cfb99328349a2275e10ed5014ea74bf8603)

It seems that this item which is purchased from Tailors was supposed to provide you with a temporary +1 bonus to Beauty, but the item prototype was missing a necessary spell.

Now it should be usable.

## [Sarah Toon offensive dialogue erroneously shows retrieved deed option](https://github.com/Dergash/arcanum-uap-hotfixes/issues/3)
Commits: [2f21291](https://github.com/Dergash/arcanum-uap-hotfixes/commit/2f2129193bd708bb23a91b63190d5f28780cb8f8)

If you initailly has a negative reaction with Sarah Toon, an option to tell you that you gave the deed to the mines to her brother will erroneously appear, even if you haven't done so.

I've added missing conditions to prevent that from happening.

# Version 1.0.0

## Magnus dialogue on Isle of Despair
Commits: [f129e79](https://github.com/Dergash/arcanum-uap-hotfixes/commit/f129e798a7d6af28a7afa74369fd84900dc8ead3)

Magnus has two fully voiced dialogues about Isle of Despair, but only the second one is accessible, and only when you already have arrived there.
This dialogue implies that you already asked him about it, and indeed there was supposed to be another dialogue where you can ask him about the Island.

I've enabled this missing dialogue for Story State 5, so now you should be able to ask Magnus about Isle of Despair after receiving the quest to reach it from Gilbert Bates but before actually travelling there

## Magnus dialogue on search for Qintarra
Commits: [44c5894](https://github.com/Dergash/arcanum-uap-hotfixes/commit/44c5894fc940a12cefef1051af79a1b596d31203)

As with Isle of Despair, Magnus has two dialogues on Qintarra, one is accessible when you already found it and another when you are searching for it.

I've enabled this missing dialogue for Story State 8, so now you should be able to ask Mask if he knows the location of Qintarra after speaking with Loghaire.

Note: This one can be considered a "mod" as while the response from Magnus is fully voiced, there is no player line for it in the original game so I had to come up with my own.

## Magnus dialogue about the Wheel Clan
Commits: [048c365](https://github.com/Dergash/arcanum-uap-hotfixes/commit/048c36543f91af3e723a0edffec1df5ff63805c5)

Again, there are two fully voiced dialogues from Magnus about the Wheel Clan, but only one is accessible when you already found its location.

I've enabled this missing dialogue for Story State 7, so now you should be able to ask Magnus about it after speaking with Thorvald Two Stones.

Note: This one can be considered a "mod" as while the response from Magnus is fully voiced, there is no player line for it in the original game so I had to come up with my own.

## [Broken dialogue about Isle of Despair for Virgil, Vollinger, Sogg, Geoffrey and Tollo](https://github.com/Dergash/arcanum-uap-hotfixes/issues/1)
Commits: [99888d0](https://github.com/Dergash/arcanum-uap-hotfixes/commit/99888d09c6ae3dda794a32b7dc36074a6c505e18)

Due to what I believe to be an [engine bug](https://github.com/alexbatalov/arcanum-ce/issues/167) in how `ia` condition checks are performed by the game engine, dialogues where you can ask your followers about Isle of Despair are unavailable in the game.

Followers affected:
- Virgil (voiced)
- Vollinger
- Sogg
- Geoffrey (voiced)
- Tollo

They all have a dialogue about Isle of Despair, but it does not work because `ia` which checks against id in `gamearea.mes` implicitly also requires that coordinates in this file also should have a Town Map. If there is no Town Map, the check fails.

Isle of Despair does have a town map, but the landing point as defined in `gamearea.mes` are slightly outside of the Town Maps, hence why all those dialogues weren't working.

To enable broken dialogues, I've adjusted coordinates slightly in `gamearea.mes`, moving the landing point from the ship closer to the camp gates, inside the Town Map.

Note: This one can be considered a "mod" as you land closer to the camp than in Vanilla, though the initial landing is still next to the ship as it handled by ship captain `.scr` file and not by `gamearea.mes`.

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

## Broken dialogue about Isle of Despair for Virgil, Vollinger, Sogg, Geoffrey and Tollo
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

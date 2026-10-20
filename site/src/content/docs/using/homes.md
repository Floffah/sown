---
title: Personal homes and player data
sidebar:
  order: 3
---

Your personal home is a customisable world stored on your device. You can use it offline without a dedicated game server, or host it for other players to visit.

Local and hosted worlds use the same world representation. Your home can contain scenes, objects, scripts, and assets like any other Sown project.

## What belongs where

| Information | Responsibility |
| --- | --- |
| Home source and local assets | Your local project |
| Persistent identity | Your cryptographic identity and its authorised keys |
| Profile, preferences, and portable player data | Your Personal Data Store |
| A world's authoritative gameplay state | That world's server |

You keep your identity when you travel between worlds. World servers control gameplay progress, and their operators set the rules for accepting portable information such as avatars. Personal Data Stores have no authority over a world's inventory, currency, or achievements.

See [Personal Data Stores](../../identity/personal-data/) for provider choice and data portability.

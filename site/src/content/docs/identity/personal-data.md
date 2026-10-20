---
title: Personal Data Stores
sidebar:
  order: 3
---

Your Personal Data Store (PDS) holds your profile details, preferences, avatar configuration, and portable player data. Choose a hosted provider or run a compatible service yourself.

Clients and worlds need authorisation to access private data. Publishing your identity record or PDS address does not make private account data public.

## Provider migration

You keep your DID when you move your personal data to another provider:

1. Establish the replacement PDS.
2. Transfer or restore portable data.
3. Authorise an identity record update identifying the replacement provider.
4. Make that signed update available through identity resolution.

Your identity belongs to you, and keeping it does not require the old provider's approval.

## Identity and data are separate

You cannot recover deleted data just by proving control of your identity. Keep backups or exports of information you need if your provider becomes unavailable.

Your PDS stores portable personal information. World servers have authority over their own gameplay state; their operators set the rules for accepting portable claims. An exported profile is not proof of an inventory item, achievement, or currency balance inside another operator's world.

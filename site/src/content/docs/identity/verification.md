---
title: Verification and privacy
sidebar:
  order: 4
---

World servers need proof that connecting clients control authorised credentials. DIDs identify players; they are not authentication secrets.

## Verifier responsibilities

Check credential authority, intended recipient, purpose, expiry, and applicable freshness or revocation information. Bind authentication proofs to the intended server and session so that a proof captured in one context does not authorise another.

World servers have no access to players' private keys or PDS passwords and no unrestricted access to their accounts. Credentials presented to a world are limited to that world and purpose.

A valid identity proof establishes control of an authorised credential. It does not establish the real-world identity of the person using it.

## Untrusted infrastructure

Resolvers can withhold updates or return old signed records, and providers can become unavailable. Malicious world servers can misrepresent their participants or ignore authentication rules. Verify claims at the boundary where they are used.

During network partitions, a verifier can miss revocations. Even when a signed record is available and its signature is valid, the verifier still needs to check its freshness.

## Privacy

Persistent identifiers can link activity across worlds. Keep private PDS information separate from public resolution records, and request only the data needed for the interaction. World servers have authorised access to specific information, rather than unrestricted access to players' accounts.

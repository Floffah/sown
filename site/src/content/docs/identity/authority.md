---
title: Identifiers and authority
sidebar:
  order: 2
---

Your decentralised identifier (DID) identifies your persistent player identity. It is independent of your username, display name, profile, or Personal Data Store address. You keep the same identity when you change providers or rotate an ordinary authentication key.

## Cryptographic control

You sign authorised operations with a private key. Other parties verify those signatures with the corresponding public key. Keep private keys secret. Knowing your DID or copying your display name does not prove control over your identity.

Identity ownership, everyday authentication, administration, and recovery are distinct responsibilities. Providers have no authority to change identity ownership simply because they host the data.

## Signed records

An identity record links the DID to information that can change, such as its PDS location, authorised authentication keys, and revision information. Recipients use authorised signatures to verify changes without trusting the service distributing the record.

Resolvers distribute records; recipients verify their authority. If someone modifies signed content, its signature no longer verifies.

## Validity and freshness

A valid signature proves that a record was authorised, even if a newer record supersedes it. For example, an older record can still have a valid signature after a key it names is revoked.

Treat freshness and revocation checks as part of authentication. Cached or client-supplied records require the same verification as records returned by a resolver.

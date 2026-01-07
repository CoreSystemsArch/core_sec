# CORE_SEC: The Entropy Challenge

> "Mathematics is the only place where truth is absolute."

## The Premise
We believe that modern security relies too heavily on hardware crutches (TPMs, Enclaves). True security should be mathematically inherent in the software architecture itself.

**CoreSec** is a proof-of-concept vault written in Rust. It utilizes military-grade AES-256-GCM encryption, PBKDF2 key derivation with aggressive iteration counts, and strict memory zeroing.

It is designed to be a black box. No backdoors. No cloud recovery. Just entropy.

## The Challenge
We have encrypted a flag within this repository. We claim that without the password, this ciphertext is indistinguishable from random noise and mathematically impossible to break within a human lifetime.

**Prove us wrong.**

### The Artifact
The following hex string contains the encrypted flag.

    f19f1c81f7ac23e2200c7cbeaf53dd2c43b8284a1228d56e5a7a48f6726440359b2c446ec722f85df9476893de070a57342aebf5515e0d88ce978c9b744b94c1f421b611e1d95bdebeaf08256fd177566a203271291a7275ca72

### The Objective
Decrypt the hex string and find the flag.
The flag follows the format: `CORE_SEC_FLAG_{...}`

If you find it, open an Issue in this repository with the plaintext flag and your method.
We don't offer a cash bounty. We offer something rarer: **The admission that we were wrong.**

### Build & Verify
You can inspect the source code (`src/main.rs`) to verify there are no tricks.
To run the tool yourself:

    cargo run

---
*Powered by CoreSystems. Welcome to the Void.*
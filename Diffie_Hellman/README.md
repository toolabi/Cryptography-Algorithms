# Diffie-Hellman Key Exchange

## Overview
The Diffie-Hellman Key Exchange is a cryptographic protocol that allows two parties to securely establish a shared secret over an insecure communication channel. This shared secret can then be used to encrypt subsequent communications using symmetric encryption algorithms.

## How It Works
The Diffie-Hellman Key Exchange relies on the mathematical properties of modular arithmetic and discrete logarithms. Here's a step-by-step explanation:

1. **Public Parameters**: Both parties agree on two public values:
   - A large prime number `p`.
   - A primitive root modulo `p`, denoted as `g`.

2. **Private Secrets**:
   - Each party selects a private secret:
     - Alice chooses a secret `x`.
     - Bob chooses a secret `y`.

3. **Public Keys**:
   - Each party computes their public key:
     - Alice computes `X = g^x mod p`.
     - Bob computes `Y = g^y mod p`.

4. **Exchange**:
   - Alice sends `X` to Bob.
   - Bob sends `Y` to Alice.

5. **Shared Secret**:
   - Both parties compute the shared secret using the other party's public key and their own private secret:
     - Alice computes `K = Y^x mod p`.
     - Bob computes `K = X^y mod p`.

   Since `(g^y)^x mod p` is equal to `(g^x)^y mod p`, both parties arrive at the same shared secret `K`.

## Why It Is Used
The Diffie-Hellman Key Exchange is widely used because:
- It enables secure key exchange over an insecure channel without requiring prior shared secrets.
- It forms the foundation for many secure communication protocols, such as TLS (Transport Layer Security).
- It ensures forward secrecy when used with ephemeral keys (e.g., in Elliptic Curve Diffie-Hellman).

## Where It Is Used
The Diffie-Hellman Key Exchange is used in:
- **Secure Web Browsing**: Establishing secure HTTPS connections.
- **Virtual Private Networks (VPNs)**: Securely exchanging keys for encrypted communication.
- **Messaging Applications**: End-to-end encryption in apps like Signal and WhatsApp.
- **Wireless Security**: Protocols like WPA3 for Wi-Fi networks.

## Example Implementation
This repository contains a simple implementation of the Diffie-Hellman Key Exchange in Rust. The code demonstrates how two parties (Alice and Bob) can compute a shared secret using the protocol.

### Running the Code
To run the example:
1. Ensure you have Rust installed.
2. Clone this repository and navigate to the project directory.
3. Run the following command:
   ```sh
   cargo run
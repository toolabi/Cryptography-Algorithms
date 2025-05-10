# AES (Advanced Encryption Standard)

## Overview
AES (Advanced Encryption Standard) is a symmetric encryption algorithm widely used for securing sensitive data. It was established as a standard by the U.S. National Institute of Standards and Technology (NIST) in 2001. AES is known for its efficiency, security, and versatility, making it one of the most popular encryption methods in the world.

## How It Works
AES operates on blocks of data using a fixed block size of 128 bits. It supports key sizes of 128, 192, or 256 bits, providing varying levels of security. The algorithm consists of several rounds of transformations, including:
1. **SubBytes**: A non-linear substitution step.
2. **ShiftRows**: A transposition step.
3. **MixColumns**: A mixing operation combining the data within each column.
4. **AddRoundKey**: A step where the data is XORed with a round key derived from the encryption key.

The number of rounds depends on the key size:
- 10 rounds for 128-bit keys.
- 12 rounds for 192-bit keys.
- 14 rounds for 256-bit keys.

AES decryption reverses these steps to retrieve the original plaintext.

## Why AES Is Used
AES is widely used because of its:
- **Security**: Resistant to known cryptographic attacks.
- **Performance**: Efficient on both hardware and software implementations.
- **Flexibility**: Supports multiple key sizes and is suitable for various applications.

## Where AES Is Used
AES is used in a wide range of applications, including:
- **Data Encryption**: Protecting sensitive data in storage and transmission.
- **Secure Communications**: Used in protocols like HTTPS, TLS, and VPNs.
- **File Encryption**: Securing files and archives.
- **Disk Encryption**: Protecting data on hard drives and SSDs.
- **Wireless Security**: Used in Wi-Fi encryption standards like WPA2 and WPA3.

## Getting Started
This project demonstrates a basic implementation of AES. To run the project, use the following command:

```sh
cargo run
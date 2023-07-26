### Password store
```
+----------------+         +----------------+
| Master Password | -----> |  KDF (PBKDF2)  | --> Encryption Key
+----------------+         +----------------+
                                |
                                v
+--------------------+     +---------+
| Plaintext Password | --> | AES-GCM | --> Encrypted Password
+--------------------+     +---------+
                                |
                                v
                            +---------+
                            | Storage |
                            +---------+
```
1. Derive encryption key from master password: The first step is to derive an encryption key from the master password provided by the user. This is done using a key derivation function (KDF). We're using PBKDF2.
2. Encrypt plaintext password: Once the encryption key has been derived, it can be used to encrypt the plaintext password using the AES-GCM encryption algorithm. AES-GCM is a symmetric encryption algorithm, which means that the same key is used for both encryption and decryption. The algorithm takes the plaintext password, the encryption key, and other parameters such as a nonce or initialization vector (IV) as input and produces the encrypted password as output. 
3. Store encrypted password: The encrypted password can then be stored in a file.
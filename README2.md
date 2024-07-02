# Euclid
Zk-snarkification of EU ID data. The goal is for European citizens holding a new EU ID card to anonymously prove part of their identity through selective disclosure and zero-knowledge proofs. For that, there are a few parts that need to be implemented. In this repository, we'll provide the code for the _proving_ part of the process. The verification can be done in different programming languages or environments. 

Here's how things work : 

### 1. Retrieval of MRZ and SOD

Plenty of data is stored on the chips of modern cards. There is the machine-readable zone (MRZ) and the security object (SOD) holding the signature metadata (when it was signed, what the public key is etc.).

![Machine-readable zone of a US passport](https://upload.wikimedia.org/wikipedia/commons/7/7e/Mrp_image.gif) 

One of the easiest (wrt UX) ways to retrieve it is by making use of the card's NFC antenna, which can be read by most smartphones. 

### Proof generation 

A Circom circuit is written and, depending on its inputs, will generate the necessary files for proving parts of the data. This will be done on the smartphone as well, for ease-of-use, as well as to avoid introducing any security issue regarding the transmission of sensitive data (MRZ). 

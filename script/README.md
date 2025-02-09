# ZK attestation for AI-generated-contents

## Introduction
- As generative AI is evolving, AI-generated contents is also increasing.
  - In the future, AI themself or the owner/creator of AI may hold and manage the IP of AI-generated contents.

- By attesting a ZKP to AI-generated contents (i.e. images), IP of these AI-generated contents can be protected. 
  - In this project, AI-generated images are used as a AI-generated contents.

- SP1 / Succinct would be used for generating a ZKP, which is attested to AI-generated contents

<br>

## Installation
- 1/ Compile program
```shell
cd program
cargo prove build
```

- 2/ Executing program without generating a proof.
```shell
cd script
RUST_LOG=info cargo run --release -- --execute
```

- 3/ Executing program with generating a proof.
```shell
cd script
RUST_LOG=info cargo run --release -- --prove
```


<br>

## NOTE
- Currently, this project has been in progress.

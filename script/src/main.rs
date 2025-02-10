//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

// use env_logger;
// use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};


/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("zk-attestation-for-ai-generated-contents-program");
//const ELF: &[u8] = include_elf!("chess-program"); /// @dev - "chess-program" is referenced from the program/Cargo.toml

#[derive(Serialize, Deserialize)]
struct ImageData {
    image_src: String,
    image_alt: String,
    //image_salt: String,
    height: u32,
    width: u32,
}

/**
 * @notice - The test of the main() in the main.rs of the /program.
 */
fn main() {
    // Setup logging.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // Create an input stream (image data, which is generated by AI)
    let image_data: ImageData = ImageData {
        image_src: "/exampleImage.png".to_string(),
        image_alt: "Example Logo".to_string(),
        //image_salt: "52d8eebd2b69894c91e0f1ab0f8db93f61855f6abf70224f7a46ca214b3cac84".to_string(),
        height: 450,
        width: 150,
    };

    let image_salt: String = "52d8eebd2b69894c91e0f1ab0f8db93f61855f6abf70224f7a46ca214b3cac84".to_string();  // Shuld be the "private" input (Not to be commited as a public value)

    // The input stream that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&image_data);
    stdin.write(&image_salt); // Shuld be the "private" input (Not to be commited as a public value)

    println!("Image Source: {}", image_data.image_src);
    println!("Image Alt: {}", image_data.image_alt);
    println!("Image Salt: {}", image_salt);
    println!("Image Height: {}", image_data.height);
    println!("Image Width: {}", image_data.width);

    // Create a `ProverClient` method.
    let client = ProverClient::from_env();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap(); // [Error]: thread 'main' panicked
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, &stdin).run().unwrap();
    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}

use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize_and_increment() {
    let program_id = solana_counter::id();
    let payer = Keypair::new();
    let (counter, _bump) = Pubkey::find_program_address(
        &[solana_counter::constants::COUNTER_SEED],
        &program_id,
    );

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/solana_counter.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_counter::instruction::Initialize {}.data(),
        solana_counter::accounts::Initialize {
            payer: payer.pubkey(),
            counter,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "initialize failed: {:?}", res.err());

    let data = svm.get_account(&counter).unwrap().data;
    let state = solana_counter::state::Counter::try_deserialize(&mut data.as_ref()).unwrap();
    assert_eq!(state.count, 0, "count should be 0 after initialize");

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_counter::instruction::Increment {}.data(),
        solana_counter::accounts::Increment {
            counter,
            user: payer.pubkey(),
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "increment failed: {:?}", res.err());

    let data = svm.get_account(&counter).unwrap().data;
    let state = solana_counter::state::Counter::try_deserialize(&mut data.as_ref()).unwrap();
    assert_eq!(state.count, 1, "count should be 1 after increment");
}

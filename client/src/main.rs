use anchor_lang::{
    prelude::Pubkey,
    solana_program::{instruction::Instruction, system_program},
    AccountDeserialize, InstructionData, ToAccountMetas,
};
use solana_keypair::read_keypair_file;
use solana_message::{Message, VersionedMessage};
use solana_rpc_client::rpc_client::RpcClient;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

fn main() {
    let rpc_url = std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let client = RpcClient::new(rpc_url);

    let home = std::env::var("HOME").expect("HOME not set");
    let wallet_path = std::env::var("ANCHOR_WALLET")
        .unwrap_or_else(|_| format!("{}/workspaces/ws-personal/wallet.json", home));
    let payer = read_keypair_file(&wallet_path).expect("failed to read wallet keypair");

    let program_id = solana_counter::id();
    let (counter, _bump) = Pubkey::find_program_address(
        &[solana_counter::constants::COUNTER_SEED],
        &program_id,
    );

    println!("program:  {}", program_id);
    println!("counter:  {}", counter);
    println!("payer:    {}", payer.pubkey());

    println!("\n[initialize]");
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
    let blockhash = client.get_latest_blockhash().expect("failed to get blockhash");
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    let sig = client.send_and_confirm_transaction(&tx).expect("initialize failed");
    println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);

    println!("\n[increment]");
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_counter::instruction::Increment {}.data(),
        solana_counter::accounts::Increment {
            counter,
            user: payer.pubkey(),
        }
        .to_account_metas(None),
    );
    let blockhash = client.get_latest_blockhash().expect("failed to get blockhash");
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
    let sig = client.send_and_confirm_transaction(&tx).expect("increment failed");
    println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);

    println!("\n[read state]");
    let account = client.get_account(&counter).expect("counter account not found");
    let state =
        solana_counter::state::Counter::try_deserialize(&mut account.data.as_ref()).unwrap();
    println!("count = {}", state.count);
}

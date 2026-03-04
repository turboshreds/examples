pub mod shredstream {
    tonic::include_proto!("shredstream");
}

use crate::shredstream::{
    SubscribeEntriesRequest, shredstream_proxy_client::ShredstreamProxyClient,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("Please provide a URL");
        return Ok(());
    }

    let url = args[1].clone();
    let mut client = ShredstreamProxyClient::connect(url).await.unwrap();
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await
        .unwrap()
        .into_inner();

    while let Some(slot_entry) = stream.message().await.unwrap() {
        let entries =
            match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                Ok(e) => e,
                Err(e) => {
                    println!("Deserialization failed with err: {e}");
                    continue;
                }
            };

        for e in entries {
            for tx in e.transactions {
                println!("Got tx https://solscan.io/tx/{}", tx.signatures[0]);
            }
        }

        // println!(
        //     "slot {}, entries: {}, transactions: {}",
        //     slot_entry.slot,
        //     entries.len(),
        //     entries.iter().map(|e| e.transactions.len()).sum::<usize>()
        // );
    }
    Ok(())
}

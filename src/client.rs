use std::io;

use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    // Get user input
    println!("How much do you want to request?");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let trimmed = input_text.trim();

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "123".to_owned(),
        to_addr: "456".to_owned(),
        amount: trimmed.parse::<u32>().unwrap(),
    });

    let response = client.send_payment(request).await?;

    println!("Response: {:?}", response);

    Ok(())
}

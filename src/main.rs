mod image_cell_abi;
mod send_tx;
mod transfer;

#[tokio::main]
async fn main() {
    transfer::transfer().await.ok();
    send_tx::send_tx().await.ok();
}

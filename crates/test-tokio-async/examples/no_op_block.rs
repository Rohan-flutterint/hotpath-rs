#[tokio::main(flavor = "current_thread")]
#[hotpath::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hotpath::measure_block!("custom_block", {
        println!("custom_block output");
    });

    Ok(())
}

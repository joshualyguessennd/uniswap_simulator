mod get_pool;
use get_pool::get_pool_data;
use clap::Parser;

fn get_subgraph_url(chain: &str) -> &'static str {
    match chain {
        "ethereum" => "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3",
        "polygon" => "https://api.thegraph.com/subgraphs/name/ianlapham/uniswap-v3-polygon",
        "celo" => "https://api.thegraph.com/subgraphs/name/jesse-sawa/uniswap-celo",
        "optimism" => "https://api.thegraph.com/subgraphs/name/ianlapham/optimism-post-regenesis",
        "arbitrum" => "https://api.thegraph.com/subgraphs/name/ianlapham/arbitrum-minimal",
        "bnb" => "https://api.thegraph.com/subgraphs/name/ianlapham/uniswap-v3-bsc",
        _ => panic!("Unsupported chain"),
    }
}

#[derive(clap::Parser)]
struct Opts {
    #[clap(short, long)]
    chain: String,

    /// The Uniswap V3 token pair address
    #[clap(short, long)]
    pair: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let chain = &opts.chain;
    let pair = &opts.pair;

    let subgraph_url = get_subgraph_url(chain);
    let pool_data = get_pool_data(subgraph_url, pair);

    match pool_data {
        Some(data) => {
            // Process the data as needed
            println!("Pool data: {:?}", data);
        },
        None => {
            println!("No data found for pool {}", pair);
        }
    }
}

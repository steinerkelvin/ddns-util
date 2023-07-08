mod cli;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use clap::Parser;

const IPV4_DETECT_URL: &str = "https://api.ipify.org/";
const IPV6_DETECT_URL: &str = "https://api6.ipify.org/";

async fn detect_ipv4() -> anyhow::Result<Ipv4Addr> {
    let res = reqwest::get(IPV4_DETECT_URL).await?.text().await?;
    let ip = Ipv4Addr::from_str(&res)?;
    Ok(ip)
}

async fn detect_ipv6() -> anyhow::Result<Ipv6Addr> {
    let res = reqwest::get(IPV6_DETECT_URL).await?.text().await?;
    let ip = Ipv6Addr::from_str(&res)?;
    Ok(ip)
}

async fn dynv6_update(
    hostname: &str,
    token: &str,
    _ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
) -> anyhow::Result<()> {
    // TODO: IPv4
    if let Some(ipv6) = ipv6 {
        const BASE_URL: &str = "http://dynv6.com/api/update";
        let url = format!(
            "{}?hostname={}&token={}&ipv6={}",
            BASE_URL, hostname, token, ipv6,
        );
        // eprintln!("Dynv6 update url: {}", url);
        let res = reqwest::get(url).await?;
        if !res.status().is_success() {
            anyhow::bail!("Dynv6 update failed: {}. \"{}\"", res.status(), res.text().await?);
        }
    } 
    Ok(())
}

async fn run(args: cli::Cli) -> anyhow::Result<()> {
    let token_var = "DYNV6_TOKEN";
    let token = std::env::var(token_var).expect("Missing token environment variable");
    let hostname_var = "HOSTNAME";
    let hostname = std::env::var(hostname_var).expect("Missing hostname environment variable");

    let ipv4 = match args.detect_ipv4 {
        cli::DetectIpv4Option::Auto => Some(detect_ipv4().await?),
    };
    let ipv6 = match args.detect_ipv6 {
        cli::DetectIpv6Option::Auto => Some(detect_ipv6().await?),
    };

    println!("ipv4: {:?}", ipv4);
    println!("ipv6: {:?}", ipv6);

    dynv6_update(&hostname, &token, None, ipv6).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI arguments with `clap`
    let args = cli::Cli::parse();
    run(args).await
}

mod cli;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

const IPV4_DETECT_URL: &str = "https://api.ipify.org/";
const IPV6_DETECT_URL: &str = "https://api6.ipify.org/";

const DYNV6_BASE_URL: &str = "http://dynv6.com/api/update";

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
    domain: &str,
    token: &str,
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
) -> anyhow::Result<()> {
    if let Some(ipv4) = ipv4 {
        eprintln!(
            "Updating Dynv6 domain '{}' with IPv4: '{}'",
            domain, ipv4
        );
        let url = format!(
            "{}?hostname={}&token={}&ipv4={}",
            DYNV6_BASE_URL, domain, token, ipv4,
        );
        let res = reqwest::get(url).await?;
        if !res.status().is_success() {
            anyhow::bail!(
                "Dynv6 update failed: {}. \"{}\"",
                res.status(),
                res.text().await?
            );
        }
        eprintln!("Dynv6 update successful: {}", res.text().await?);
    }
    if let Some(ipv6) = ipv6 {
        eprintln!(
            "Updating Dynv6 domain '{}' with IPv6: '{}'",
            domain, ipv6
        );
        let url = format!(
            "{}?hostname={}&token={}&ipv6={}",
            DYNV6_BASE_URL, domain, token, ipv6,
        );
        let res = reqwest::get(url).await?;
        if !res.status().is_success() {
            anyhow::bail!(
                "Dynv6 update failed: {}. \"{}\"",
                res.status(),
                res.text().await?
            );
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
#[error("Missing configuration (environment variable: {env:?})")]
struct MissingConfigError {
    env: Option<String>,
}

fn get_env_var(var_name: &str) -> anyhow::Result<String> {
    let val = std::env::var(var_name);
    match val {
        Ok(val) => Ok(val),
        Err(err) => Err(anyhow::Error::new(MissingConfigError {
            env: Some(var_name.to_string()),
        })
        .context(err)),
    }
}

async fn run(args: cli::Cli) -> anyhow::Result<()> {
    let token_var = "DYNV6_TOKEN";
    let token = get_env_var(token_var)?;
    let domain_var = "DOMAIN";
    let domain = get_env_var(domain_var)?;

    let ipv4 = match args.detect_ipv4 {
        cli::DetectIpv4Option::Auto => Some(detect_ipv4().await?),
        cli::DetectIpv4Option::Nope => None,
    };
    let ipv6 = match args.detect_ipv6 {
        cli::DetectIpv6Option::Auto => Some(detect_ipv6().await?),
        cli::DetectIpv6Option::Nope => None,
    };

    dynv6_update(&domain, &token, ipv4, ipv6).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI arguments with `clap`
    let args = cli::Cli::parse();
    run(args).await
}

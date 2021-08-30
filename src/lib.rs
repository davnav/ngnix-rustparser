
#[macro_use]
extern crate pest_derive;

use pest::Parser;
// use nginx_rustparser::*;

#[derive(Parser)]
#[grammar = "config.pest"]
pub struct NginxConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_block() {
        let bk1 = NginxConfig::parse(Rule::BLOCK, "http {
            include conf;
            server{
                listen 80;
            } }").expect("can not read the file");

        println!("{:?}",bk1);
    }
}

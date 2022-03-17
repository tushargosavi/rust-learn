extern crate regex;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::io::{self, BufRead};

#[derive(Debug, Serialize, Deserialize)]
struct EsRecord {
    date: String,
    action: String,
    host: String,
    remote: String,
    remote_addr: String,
    master: String,
    master_addr: String,
}

fn to_iso_date(s: &str) -> String {
    let parts : Vec<&str> = s.split(" ").collect();
    format!("2022-01-{} {}", parts[1], parts[2])
}

fn parse_add_remove_line(line: &str) -> Vec<EsRecord> {
    lazy_static! {
        static ref re: Regex = Regex::new(
            r"^(... \d{2} \d{2}:\d{2}:\d{2}).*\[(.*)\] (added|removed) \{(.*) reason:.*\[master (.*)\}.* committed.*"
        )
        .unwrap();
        static ref setsre: Regex =
            Regex::new(r"\{(.*)\}\{(.*)\}\{(.*)\}\{(.*)\}\{(.*)\}(.*)").unwrap();
    }

    let mut result = vec![];

    if let Some(cap) = re.captures(line) {
        if cap.len() < 5 {
            return result;
        }
        let date = &cap[1];
        let host = &cap[2];
        let action = &cap[3];
        let sets_lines = &cap[4];
        let master_part = &cap[5];

        let mut master_host = String::from("");
        let mut master_addr = String::from("");
        if let Some(cap) = setsre.captures(master_part) {
            master_host = (&cap[1]).to_owned();
            master_addr = (&cap[5]).to_owned();
        }

        //println!("rest {}", sets_lines);
        for line in sets_lines.split(",") {
            if let Some(cap) = setsre.captures(line) {
                for i in 1..cap.len() {
                    let dest = &cap[1];
                    let dest_addr = &cap[5];
                    result.push(EsRecord {
                        date: to_iso_date(date),
                        action: action.to_owned(),
                        host: host.to_owned(),
                        remote: dest.to_owned(),
                        remote_addr: dest_addr.to_owned(),
                        master: master_host.to_owned(),
                        master_addr: master_addr.to_owned(),
                    })
                }
            }
        }
    }
    result
}

fn parse_connect_failed(line: &str) -> Vec<EsRecord> {
    lazy_static! {
        static ref re: Regex = Regex::new(
            r"^(... \d{2} \d{2}:\d{2}:\d{2}).*\[(.*)\] failed to connect to node \{(.*)\}\{.*\}\{.*\}\{.*\}\{(.*)\}\{.*\}.*"
        )
        .unwrap();
    }
    let mut result = vec![];
    if let Some(cap) = re.captures(line) {
        result.push(EsRecord {
            date: to_iso_date(&cap[1]),
            action: String::from("connect_error"),
            host: (&cap[2]).to_owned(),
            remote: (&cap[3]).to_owned(),
            remote_addr: (&cap[4]).to_owned(),
            master: String::from(""),
            master_addr: String::from(""),
        });
        /*
        for i in 1..cap.len() {
            println!("index={} {}", i, &cap[i]);
        }
        */
    }
    result
}

fn parse_send_failed(line: &str) -> Vec<EsRecord> {
    lazy_static! {
        static ref re: Regex = Regex::new(
            r"^(... \d{2} \d{2}:\d{2}:\d{2}).*\[(.*)\] send message failed .*localAddress=(.*), remoteAddress=(.*)/(.*)\}\].*"
        )
        .unwrap();
    }
    let mut result = vec![];
    if let Some(cap) = re.captures(line) {
        result.push(EsRecord {
            date: to_iso_date(&cap[1]),
            action: String::from("send_fail"),
            host: (&cap[2]).to_owned(),
            remote: (&cap[4]).to_owned(),
            remote_addr: (&cap[5]).to_owned(),
            master: String::from(""),
            master_addr: String::from(""),
        });
        /*
        for i in 1..cap.len() {
            println!("index={} {}", i, &cap[i]);
        }
        */
    }
    result
}

fn parse_line(line: &str) -> Vec<EsRecord> {
    if line.contains("failed to connect to node") {
        parse_connect_failed(line)
    } else if line.contains("added") || line.contains("removed") {
        parse_add_remove_line(line)
    } else if line.contains("send message failed") {
        parse_send_failed(line)
    } else {
        vec![]
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                //println!("processing {}", line);
                let result = parse_line(line.as_str());
                for ref r in result {
                    println!("{}", serde_json::to_string(r).unwrap());
                }
            }
            Err(e) => break,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn sample_run() {
        let log_lines = vec![
       "Jan 28 12:58:13 adDLclusterp48.corp.cat.com docker[31330]: [2022-01-28T12:58:13,871][INFO ][o.e.c.s.ClusterApplierService] [host48-4] removed {{host43-3}{M2ki3lsCTTWEfK7VlNVkQg}{5gnwv9erS46Suk3VMdIh-w}{elasticsearch-host43-d}{10.50.21.72:9300}{rack_id=host43, box_type=warm},{host43-2}{2GjnExSmRhC6e9jTd8YL8g}{Kmn5MdiJStOd98dui71MRw}{elasticsearch-host43-c}{10.50.21.78:9300}{rack_id=host43, box_type=warm},{host43-4}{UKGsWuudSs-fswPRQiJOpg}{whwBtiGPR9yA5_Zdk65PCw}{elasticsearch-host43-e}{10.50.21.120:9300}{rack_id=host43, box_type=warm},{host44-0}{CZeAZi1yTVuoPbSol9L7Jg}{9deiGX2USpKxF94TfpLA0g}{elasticsearch-host44-a}{10.50.7.59:9300}{rack_id=host44, box_type=hot},}, reason: apply cluster state (from master [master {host1-2}{FbttKZFfQPuntpoHABK1cg}{Qr3iqwKoS_6YfGlk8CYJQg}{elasticsearch-host1-c}{10.50.56.65:9300}{rack_id=host1, box_type=warm} committed version [1214]])",
       "Jan 28 12:59:45 adDLclusterp48.corp.cat.com docker[31330]: [2022-01-28T12:59:45,079][INFO ][o.e.c.s.ClusterApplierService] [host48-4] added {{host43-4}{UKGsWuudSs-fswPRQiJOpg}{sN1iShARQqiIcNsYB2gADw}{elasticsearch-host43-e}{10.50.21.111:9300}{rack_id=host43, box_type=warm},{host43-1}{iKFc-YkmR4ir3Gy38jcOpw}{OL1JofHkRWeBpdwaiGmPDA}{elasticsearch-host43-b}{10.50.21.104:9300}{rack_id=host43, box_type=warm},{host44-0}{CZeAZi1yTVuoPbSol9L7Jg}{7xWeY_1BQFu-db91ajfzPA}{elasticsearch-host44-a}{10.50.7.54:9300}{rack_id=host44, box_type=hot},{host43-2}{2GjnExSmRhC6e9jTd8YL8g}{LOrw7pXOTVSdZk1ebjluPw}{elasticsearch-host43-c}{10.50.21.97:9300}{rack_id=host43, box_type=warm},{host43-3}{M2ki3lsCTTWEfK7VlNVkQg}{rucjLsSRR1K-Z9tlQvXNKg}{elasticsearch-host43-d}{10.50.21.70:9300}{rack_id=host43, box_type=warm},}, reason: apply cluster state (from master [master {host1-2}{FbttKZFfQPuntpoHABK1cg}{Qr3iqwKoS_6YfGlk8CYJQg}{elasticsearch-host1-c}{10.50.56.65:9300}{rack_id=host1, box_type=warm} committed version [1215]])",
       "Jan 28 23:07:17 adDLclusterp02.corp.cat.com docker[18620]: [2022-01-28T23:07:17,218][WARN ][o.e.c.NodeConnectionsService] [host2-1] failed to connect to node {host75-2}{GIdeTxH9R1C0cCMKvCl7JA}{U-KdqGn7SXG_bhf7BKTa9w}{elasticsearch-host75-c}{10.50.12.200:9300}{rack_id=host75, box_type=warm} (tried [7] times)",
       "Jan 28 23:05:30 adDLclusterp02.corp.cat.com docker[16652]: [2022-01-28T23:05:30,125][WARN ][o.e.t.OutboundHandler    ] [host2-0] send message failed [channel: Netty4TcpChannel{localAddress=0.0.0.0/0.0.0.0:37940, remoteAddress=elasticsearch-host75-c/10.50.12.200:9300}]"
    ];

        for line in log_lines {
            let result = parse_line(line);
            for ref r in result {
                println!("{}", serde_json::to_string(r).unwrap());
            }
        }
    }
}

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers : u64,
    ignore : bool,
    auth_server : Option<String>
}

fn main() {
    let config = ServerConfig{
        workers : 100,
        ignore : true,
        auth_server : None,
    };

    {
        println!("To and from YAML");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("serialized {}", serialized);
        let deserialized : ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        println!("de {:?}", deserialized);
    }

    {
        println!("To and from Json");
        let serialized = serde_json::to_string(&config).unwrap();
        println!("json {}", serialized);
        let deserialized : ServerConfig = serde_json::from_str(&serialized).unwrap();
        println!("de {:?}", deserialized);
    }
}
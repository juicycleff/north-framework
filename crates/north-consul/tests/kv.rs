extern crate north_consul;
use north_consul::kv::KVPair;
use north_consul::{Client, Config};

#[ignore]
#[tokio::test]
async fn kv_test() {
    use north_consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = client.list("", None).await.unwrap();
    assert!(r.0.is_empty());
    
    let pair = KVPair {
        Key: String::from("testkey"),
        Value: String::from("testvalue"),
        ..Default::default()
    };
    
    assert!(client.put(&pair, None).await.unwrap().0);
    
    let b64val = client.get("testkey", None).await.unwrap().0.unwrap().Value;
    let bytes = base64::decode(b64val).unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), "testvalue");
    
    let r = client.list("t", None).await.unwrap();
    assert!(!r.0.is_empty());
    
    client.delete("testkey", None).await.unwrap();
    
    let r = client.list("", None).await.unwrap();
    assert!(r.0.is_empty());
}
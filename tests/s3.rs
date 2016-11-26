#![cfg(feature = "s3")]
extern crate env_logger;
extern crate rusoto;

#[macro_use]
extern crate log;

use std::io::Read;
use std::fs::File;
use std::env::var;
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Client, ListObjectsRequest, HeadObjectRequest, CopyObjectRequest,
                 GetObjectRequest, PutObjectRequest, DeleteObjectRequest};

fn test_bucket() -> String {
    match var("S3_TEST_BUCKET") {
        Ok(val) => val.to_owned(),
        Err(_) => "rusototester".to_owned(),
    }
}

fn test_bucket_region() -> Region {
    match var("S3_TEST_BUCKET_REGION") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => "us-west-2".parse().unwrap(),
    }
}

#[test]
fn object_lifecycle_test() {
    let client = S3Client::new(DefaultCredentialsProvider::new().unwrap(),
                               test_bucket_region());

    let filename = "some_file";

    // PUT an object
    let mut f = File::open("tests/sample-data/no_credentials").unwrap();
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: test_bucket(),
                key: filename.to_owned(),
                body: Some(contents),
                ..Default::default()
            };
            client.put_object(&req);
        }
    }


    // HEAD the object that was PUT
    let size_req = HeadObjectRequest {
        bucket: test_bucket(),
        key: filename.to_owned(),
        ..Default::default()
    };

    println!("{:#?}", client.head_object(&size_req).unwrap());

    // GET the object
    let get_req = GetObjectRequest {
        bucket: test_bucket(),
        key: filename.to_owned(),
        ..Default::default()
    };

    println!("{:#?}", client.get_object(&get_req).unwrap());

    // copy the object to change its settings
    let req = CopyObjectRequest {
        bucket: test_bucket(),
        key: filename.to_owned(),
        copy_source: format!("{}/{}", test_bucket(), filename),
        cache_control: Some("max-age=123".to_owned()),
        content_type: Some("application/json".to_owned()),
        metadata_directive: Some("REPLACE".to_owned()),
        ..Default::default()
    };

    println!("{:#?}", req);

    println!("{:#?}", client.copy_object(&req));

    // DELETE the object
    let del_req = DeleteObjectRequest {
        bucket: test_bucket(),
        key: filename.to_owned(),
        ..Default::default()
    };

    //println!("{:#?}", client.delete_object(&del_req).unwrap());
}

#[test]
fn should_list_buckets() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = S3Client::new(credentials, Region::UsEast1);

    client.list_buckets().unwrap();
}

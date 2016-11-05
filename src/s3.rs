<<<<<<< f9400e99118d70fe56e4c33eb118623fafed9655
//! The AWS S3 API.

#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
#![allow(unused_variables, unused_mut)]

use std::fmt;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::io::Read;
use std::num::ParseIntError;
use std::str::{FromStr, ParseBoolError};
use std::str;

use hyper::client::{Client, RedirectPolicy};
use md5;
use rusoto_credential::{
    ProvideAwsCredentials,
    AwsCredentials,
    CredentialsError,
};
use rustc_serialize::base64::{ToBase64, STANDARD};
use xml::*;
use regex::Regex;

use param::{Params, ServiceParams};
use region::Region;
use signature::SignedRequest;
use xmlutil::*;
use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError};
use region;

#[derive(Debug, Default)]
pub struct S3Error {
    pub message: String
}

impl S3Error {
    fn new<S>(message: S) -> S3Error where S: Into<String> {
        S3Error { message: message.into() }
    }
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for S3Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<HttpDispatchError> for S3Error {
    fn from(err: HttpDispatchError) -> S3Error {
        S3Error { message: err.description().to_owned() }
    }
}

impl From<CredentialsError> for S3Error {
//! AWS S3
 
include!(concat!(env!("OUT_DIR"), "/s3.rs"));

#[cfg(test)]
mod test {
    use s3::{S3Client, HeadObjectRequest, GetObjectRequest};
    use super::super::{Region, SignedRequest};
    use super::super::mock::*;

    extern crate env_logger;

    #[test]
    // sample response from the S3 documentation
    // tests the model generation and deserialization end-to-end
    fn should_parse_sample_list_buckets_response() {
        let mock = MockRequestDispatcher::with_status(200)
            .with_body(r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01">
                <Owner>
                <ID>bcaf1ffd86f461ca5fb16fd081034f</ID>
                <DisplayName>webfile</DisplayName>
                </Owner>
                <Buckets>
                <Bucket>
                        <Name>quotes</Name>
                        <CreationDate>2006-02-03T16:45:09.000Z</CreationDate>
                </Bucket>
                <Bucket>
                        <Name>samples</Name>
                        <CreationDate>2006-02-03T16:41:58.000Z</CreationDate>
                </Bucket>
                </Buckets>
            </ListAllMyBucketsResult>
            "#)
            .with_request_checker(|request: &SignedRequest| {
                assert_eq!(request.method, "GET");
                assert_eq!(request.path, "/");
                assert_eq!(request.params.get("Action"),
                           Some(&"ListBuckets".to_string()));
                assert_eq!(request.payload, None);
            });

        let client = S3Client::with_request_dispatcher(mock, MockCredentialsProvider, Region::UsEast1);
        let result = client.list_buckets().unwrap();

        let owner = result.owner.unwrap();
        assert_eq!(owner.display_name, Some("webfile".to_string()));
        assert_eq!(owner.id,
                   Some("bcaf1ffd86f461ca5fb16fd081034f".to_string()));

        let buckets = result.buckets.unwrap();
        assert_eq!(buckets.len(), 2);

        let bucket1 = buckets.get(0).unwrap();
        assert_eq!(bucket1.name, Some("quotes".to_string()));
        assert_eq!(bucket1.creation_date,
                   Some("2006-02-03T16:45:09.000Z".to_string()));
    }

    #[test]
    fn should_parse_headers() {
        let mock = MockRequestDispatcher::with_status(200)
            .with_body("")
            .with_header("x-amz-expiration".to_string(), "foo".to_string())
            .with_header("x-amz-restore".to_string(), "bar".to_string());

        let client = S3Client::with_request_dispatcher(mock, MockCredentialsProvider, Region::UsEast1);
        let request = HeadObjectRequest::default();
        let result = client.head_object(&request).unwrap();

        assert_eq!(result.expiration, Some("foo".to_string()));
        assert_eq!(result.restore, Some("bar".to_string()));
    }

    #[test]
    fn should_serialize_complicated_request() {
        initialize_logger();

        let request = GetObjectRequest {
            bucket: "bucket".to_string(),
            if_match: sstr("if_match"),
            if_modified_since: sstr("if_modified_since"),
            if_none_match: sstr("if_none_match"),
            if_unmodified_since: sstr("if_unmodified_since"),
            key: "key".to_string(),
            range: sstr("range"),
            request_payer: sstr("request_payer"),
            response_cache_control: sstr("response_cache_control"),
            response_content_disposition: sstr("response_content_disposition"),
            response_content_encoding: sstr("response_content_encoding"),
            response_content_language: sstr("response_content_language"),
            response_content_type: sstr("response_content_type"),
            response_expires: sstr("response_expires"),
            sse_customer_algorithm: sstr("sse_customer_algorithm"),
            sse_customer_key: sstr("sse_customer_key"),
            sse_customer_key_md5: sstr("sse_customer_key_md5"),
            version_id: sstr("version_id")
        };

        let mock = MockRequestDispatcher::with_status(200)
            .with_body("")
            .with_request_checker(|request: &SignedRequest| {
                debug!("{:#?}", request);
                assert_eq!(request.method, "GET");
                assert_eq!(request.path, "/bucket/key");
                assert_eq!(request.params.get("Action"), sstr("GetObject").as_ref());
                assert_eq!(request.params.get("response-content-type"), sstr("response_content_type").as_ref());
                assert!(request.headers.get("range").unwrap().contains(&Vec::from("range")));

                assert_eq!(request.payload, None);
            });

        let client = S3Client::with_request_dispatcher(mock, MockCredentialsProvider, Region::UsEast1);
        let _ = client.get_object(&request).unwrap();
    }

    /// returns Some(String)
    fn sstr(value: &'static str) -> Option<String> {
        Some(value.to_string())
    }

    fn initialize_logger() {
        let _ = env_logger::init();
    }

}

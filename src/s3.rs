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
	use s3::S3Client;
	use super::super::{Region, DefaultCredentialsProvider, SignedRequest};
	use super::super::mock::*;

	#[test]
	// sample response from the S3 documentation
	// tests the model generation and deserialization end-to-end
	fn should_parse_sample_list_buckets_response() {
		let credentials = DefaultCredentialsProvider::new().unwrap();
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
			.with_request_checker(
				|request: &SignedRequest| {
					assert_eq!(request.method, "GET");
					assert_eq!(request.path, "/");
					assert_eq!(request.params.get("Action"), Some(&"ListBuckets".to_string()));
					assert_eq!(request.payload, None);
				}
			);

	    let client = S3Client::with_request_dispatcher(mock, credentials, Region::UsEast1);
	    let result = client.list_buckets().unwrap();

	    let owner = result.owner.unwrap();
	    assert_eq!(owner.display_name, Some("webfile".to_string()));
	    assert_eq!(owner.i_d, Some("bcaf1ffd86f461ca5fb16fd081034f".to_string()));

	    let buckets = result.buckets.unwrap();
	    assert_eq!(buckets.len(), 2);

	    let bucket1 = buckets.get(0).unwrap();
	    assert_eq!(bucket1.name, Some("quotes".to_string()));
	    assert_eq!(bucket1.creation_date, Some("2006-02-03T16:45:09.000Z".to_string()));

	    let bucket2 = buckets.get(1).unwrap();
	    assert_eq!(bucket2.name, Some("samples".to_string()));
	    assert_eq!(bucket2.creation_date, Some("2006-02-03T16:41:58.000Z".to_string()));
	}
}

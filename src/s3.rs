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

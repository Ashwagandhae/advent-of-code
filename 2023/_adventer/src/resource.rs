use anyhow::Result;
use itertools::Itertools;
use reqwest;
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::fs;

const YEAR: i32 = 2023;

#[derive(Debug, Clone, Deserialize)]
pub struct Secrets {
    pub session: String,
}

const SECRETS: OnceCell<Secrets> = OnceCell::new();

fn secrets(secrets: &OnceCell<Secrets>) -> &Secrets {
    secrets.get_or_init(|| {
        let secrets: Secrets =
            serde_json::from_str(&std::fs::read_to_string("./secrets.json").unwrap()).unwrap();
        secrets
    })
}

pub fn print_input(day: i32) -> Result<()> {
    println!("{}", get_input(day)?);
    Ok(())
}

pub fn print_output(name: ProblemName) -> Result<()> {
    println!("{}", get_output(name)?);
    Ok(())
}

pub fn print_example_input(name: ProblemName) -> Result<()> {
    println!("{}", get_example_input(name)?);
    Ok(())
}

pub fn print_example_output(name: ProblemName) -> Result<()> {
    println!("{}", get_example_output(name)?);
    Ok(())
}

pub fn get_input(day: i32) -> Result<String> {
    let data = get_data(DataId::Text(day))?;
    Ok(data)
}
#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize, Deserialize)]
pub enum ProblemPart {
    #[clap(alias = "0")]
    Zero,
    #[clap(alias = "1")]
    One,
}

impl std::fmt::Display for ProblemPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemPart::Zero => write!(f, "0"),
            ProblemPart::One => write!(f, "1"),
        }
    }
}

/// First tries to get cached data. If the provided function fails, it will try to get fresh data and try the function again.
pub fn get_fresh_data_if_needed<T>(
    id: DataId,
    fun: impl Fn(&str) -> T,
    success: impl Fn(&T) -> bool,
) -> Result<T> {
    let (data, fresh) = get_data_with_freshness(id)?;
    let result = fun(&data);
    match (success(&result), fresh) {
        (false, false) => {
            uncache_data(id);
            let data = get_data(id).unwrap();
            Ok(fun(&data))
        }
        _ => Ok(result),
    }
}

pub fn get_output(name: ProblemName) -> Result<String> {
    let result = get_fresh_data_if_needed(
        DataId::Html(name.day),
        |data| find_output(data, name.part),
        |result| match result {
            Err(FindOutputError::NoOutputYet(_)) => false,
            _ => true,
        },
    )?;
    Ok(result?)
}

pub fn get_example_input(name: ProblemName) -> Result<String> {
    let result = get_fresh_data_if_needed(
        DataId::Html(name.day),
        |data| find_example_input(data, name.part),
        |result| match result {
            Err(FindExampleError::IndexTooHigh(ProblemPart::One)) => false,
            _ => true,
        },
    )?;
    Ok(result?)
}

pub fn get_example_output(name: ProblemName) -> Result<String> {
    let result = get_fresh_data_if_needed(
        DataId::Html(name.day),
        |data| find_example_output(data, name.part),
        |result| match result {
            Err(FindExampleError::IndexTooHigh(ProblemPart::One)) => false,
            _ => true,
        },
    )?;
    Ok(result?)
}
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum FindOutputError {
    #[error("no output for {0} yet")]
    NoOutputYet(ProblemPart),
}

fn find_output(html: &str, index: ProblemPart) -> Result<String, FindOutputError> {
    let document = Html::parse_document(html);
    let main = Selector::parse("main > article.day-desc + p").unwrap();
    let Some(para) = document.select(&main).nth(match index {
        ProblemPart::Zero => 0,
        ProblemPart::One => 1,
    }) else { return Err(FindOutputError::NoOutputYet(index))};
    let select = Selector::parse("code").unwrap();
    let code = para.select(&select).next().unwrap();
    Ok(code.text().join("").trim().to_string())
}

#[derive(Debug, Clone, Error)]
pub enum FindExampleError {
    #[error("advent of code didn't have a problem at {0}")]
    IndexTooHigh(ProblemPart),
}
fn find_example_input(html: &str, index: ProblemPart) -> Result<String, FindExampleError> {
    let document = Html::parse_document(html);
    let main = Selector::parse("main > article.day-desc").unwrap();
    let Some(article) = document.select(&main).nth(match index {
        ProblemPart::Zero => 0,
        ProblemPart::One => 1,
    }) else { return Err(FindExampleError::IndexTooHigh(index))};
    let select = Selector::parse("p, pre").unwrap();
    let iter = article.select(&select);
    Ok(iter
        .tuple_windows()
        .find(|(el_1, el_2)| {
            let text = el_1.text().join("");
            let formatted_text = text.trim().to_lowercase();

            el_1.value().name() == "p"
                && el_2.value().name() == "pre"
                && (formatted_text.contains("example") || formatted_text.contains("consider"))
                && formatted_text.ends_with(":")
        })
        .map(|(_, el)| el.text().join("").to_string())
        .unwrap()
        .trim()
        .to_string())
}

fn find_example_output(html: &str, index: ProblemPart) -> Result<String, FindExampleError> {
    let document = Html::parse_document(html);
    let main = Selector::parse("main > article.day-desc").unwrap();
    let Some(article) = document.select(&main).nth(match index {
        ProblemPart::Zero => 0,
        ProblemPart::One => 1,
    }) else { return Err(FindExampleError::IndexTooHigh(index))};
    let select = Selector::parse("p").unwrap();
    // get second to last para
    let paras = article.select(&select).collect_vec();
    let second_to_last = paras[paras.len() - 2];
    // get last <code> > <em> in that para
    let select = Selector::parse("code > em").unwrap();
    let mut codes = second_to_last.select(&select).collect_vec();
    let last = codes.pop().unwrap();
    Ok(last.text().join("").trim().to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum DataId {
    Text(i32),
    Html(i32),
}

impl DataId {
    fn extension(&self) -> &str {
        match self {
            DataId::Text(_) => "txt",
            DataId::Html(_) => "html",
        }
    }
    fn day(&self) -> i32 {
        match self {
            DataId::Text(day) => *day,
            DataId::Html(day) => *day,
        }
    }
    fn fetch_url(&self) -> String {
        match self {
            DataId::Text(day) => format!("https://adventofcode.com/{}/day/{}/input", YEAR, day),
            DataId::Html(day) => format!("https://adventofcode.com/{}/day/{}", YEAR, day),
        }
    }
}

fn get_data_with_freshness(id: DataId) -> Result<(String, bool)> {
    if let Some(data) = get_data_from_cache(id) {
        Ok((data, false))
    } else {
        // fetch from advent of code
        let data = fetch_data(id)?;
        // cache
        cache_data(id, &data);
        Ok((data, true))
    }
}

fn get_data(id: DataId) -> Result<String> {
    get_data_with_freshness(id).map(|(data, _)| data)
}

fn get_data_from_cache(id: DataId) -> Option<String> {
    let path = format!("./_cache/{}.{}", id.day(), id.extension());
    if !std::path::Path::new(&path).exists() {
        return None;
    }
    Some(fs::read_to_string(path).unwrap())
}

fn cache_data(id: DataId, input: &str) {
    let path = format!("./_cache/{}.{}", id.day(), id.extension());
    fs::write(path, input).unwrap();
}

fn uncache_data(id: DataId) {
    let path = format!("./_cache/{}.{}", id.day(), id.extension());
    fs::remove_file(path).unwrap();
}

pub fn fetch_data(id: DataId) -> Result<String> {
    let url = id.fetch_url();
    let client = http_client(&secrets(&SECRETS).session, "text/html")?;
    let res = client.get(&url).send()?;
    // check status
    if res.status() != 200 {
        anyhow::bail!("fetching data failed: {}", res.status());
    }
    let body = res.text()?;
    Ok(body.trim().to_string())
}

use http::{
    header::{CONTENT_TYPE, COOKIE, USER_AGENT},
    HeaderMap, HeaderValue,
};
use reqwest::blocking::Client as HttpClient;
use reqwest::redirect::Policy;

use crate::ProblemName;
fn http_client(session_cookie: &str, content_type: &str) -> Result<HttpClient> {
    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim()))?;
    let content_type_header = HeaderValue::from_str(content_type).unwrap();
    let user_agent = format!("adventer 1.0");
    let user_agent_header = HeaderValue::from_str(&user_agent).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_type_header);
    headers.insert(USER_AGENT, user_agent_header);

    Ok(HttpClient::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()?)
}

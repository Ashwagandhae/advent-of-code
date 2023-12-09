use crate::problem::{ProblemDay, ProblemPart};
use anyhow::Result;
use itertools::Itertools;
use reqwest;
use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

pub const YEAR: i32 = 2023;

#[derive(Debug, Clone, Deserialize)]
pub struct Secrets {
    pub session: String,
}

static SECRETS: OnceLock<Secrets> = OnceLock::new();

fn secrets(secrets: &OnceLock<Secrets>) -> &Secrets {
    secrets.get_or_init(|| {
        let secrets: Secrets =
            serde_json::from_str(&std::fs::read_to_string("./secrets.json").expect(
                "failed to get secrets.json, make a ./secrets.json file with your session cookie",
            ))
            .expect("failed to parse secrets.json");
        secrets
    })
}

pub fn print_input(day: ProblemDay) -> Result<()> {
    println!("{}", get_input(day)?);
    Ok(())
}

pub fn print_output(name: Problem) -> Result<()> {
    println!("{}", get_output(name)?);
    Ok(())
}

pub fn print_example_input(name: Problem) -> Result<()> {
    println!("{}", get_example_input(name)?);
    Ok(())
}

pub fn print_example_output(name: Problem) -> Result<()> {
    println!("{}", get_example_output(name)?);
    Ok(())
}

pub fn get_input(day: ProblemDay) -> Result<String> {
    let data = get_data(DataId::InputText(day))?;
    Ok(data)
}

pub fn get_output(name: Problem) -> Result<String> {
    let data = get_data(DataId::OutputText(name))?;
    Ok(data)
}

pub fn get_example_input(name: Problem) -> Result<String> {
    let data = get_data(DataId::ExampleInputText(name))?;
    Ok(data)
}

pub fn get_example_output(name: Problem) -> Result<String> {
    let data = get_data(DataId::ExampleOutputText(name))?;
    Ok(data)
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
            uncache_data(id)?;
            let data = get_data(id)?;
            Ok(fun(&data))
        }
        _ => Ok(result),
    }
}

pub fn get_output_from_html(name: Problem) -> Result<String> {
    let result = get_fresh_data_if_needed(
        DataId::ProblemHtml(name.day),
        |data| find_output(data, name.part),
        |result| !matches!(result, Err(FindOutputError::NoOutputYet(_))),
    )?;
    Ok(result?)
}

pub fn get_example_input_from_html(name: Problem) -> Result<String> {
    let result = get_fresh_data_if_needed(
        DataId::ProblemHtml(name.day),
        |data| find_example_input(data, name.part),
        |result| {
            !matches!(
                result,
                Err(FindExampleError::IndexTooHigh(ProblemPart::One))
            )
        },
    )?;
    Ok(result?)
}

pub fn get_example_output_from_html(name: Problem) -> Result<String> {
    let result = get_fresh_data_if_needed(
        DataId::ProblemHtml(name.day),
        |data| find_example_output(data, name.part),
        |result| {
            !matches!(
                result,
                Err(FindExampleError::IndexTooHigh(ProblemPart::One))
            )
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
    #[error("no output (code) element in para (p)")]
    NoOutputElementInPara,
}

fn find_output(html: &str, index: ProblemPart) -> Result<String, FindOutputError> {
    let document = Html::parse_document(html);
    let main = Selector::parse("main > article.day-desc + p").unwrap();
    let Some(para) = document.select(&main).nth(match index {
        ProblemPart::Zero => 0,
        ProblemPart::One => 1,
    }) else { return Err(FindOutputError::NoOutputYet(index))};
    let select = Selector::parse("code").unwrap();
    let Some(code) = para.select(&select).next() else {return Err(FindOutputError::NoOutputElementInPara)};
    Ok(code.text().join("").trim().to_string())
}

#[derive(Debug, Clone, Error)]
pub enum FindExampleError {
    #[error("advent of code didn't have a problem at {0}")]
    IndexTooHigh(ProblemPart),
    #[error("no example input (pre) element in article (article.day-desc)")]
    NoExampleElementInArticle,
    #[error("no second to last para (p) in article (article.day-desc)")]
    NoSecondToLastPara,
    #[error("no last output (code) element in last para (p)")]
    NoLastCodeElementInPara,
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
    let Some(example_input_text) = iter
    .tuple_windows()
    .find(|(el_1, el_2)| {
        let text = el_1.text().join("");
        let formatted_text = text.trim().to_lowercase();

        el_1.value().name() == "p"
            && el_2.value().name() == "pre"
            && (formatted_text.contains("example") || formatted_text.contains("consider"))
            && formatted_text.ends_with(':')
    })
    .map(|(_, el)| el.text().join("")) else { return Err(FindExampleError::NoExampleElementInArticle)};
    Ok(example_input_text.trim().to_string())
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
    let Some(second_to_last) = paras.get(paras.len() - 2) else { return Err(FindExampleError::NoSecondToLastPara)};
    // get last <code> > <em> in that para
    let select = Selector::parse("code > em").unwrap();
    let mut codes = second_to_last.select(&select).collect_vec();
    let Some(last) = codes.pop() else { return Err(FindExampleError::NoLastCodeElementInPara)};
    Ok(last.text().join("").trim().to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum DataId {
    OutputText(Problem),
    ExampleOutputText(Problem),
    ExampleInputText(Problem),
    InputText(ProblemDay),
    ProblemHtml(ProblemDay),
}

impl DataId {
    fn cache_file_name(&self) -> String {
        match self {
            DataId::ProblemHtml(day) => format!("{}.html", day),
            DataId::InputText(day) => format!("{}i.txt", day),
            DataId::OutputText(problem) => format!("{}o.txt", problem.to_string()),
            DataId::ExampleInputText(problem) => format!("{}ei.txt", problem.to_string()),
            DataId::ExampleOutputText(problem) => format!("{}eo.txt", problem.to_string()),
        }
    }

    fn get(&self) -> Result<String> {
        match self {
            DataId::ProblemHtml(day) => {
                fetch_data(&format!("https://adventofcode.com/{}/day/{}", YEAR, day))
            }
            DataId::InputText(day) => fetch_data(&format!(
                "https://adventofcode.com/{}/day/{}/input",
                YEAR, day
            )),
            DataId::OutputText(problem) => get_output_from_html(*problem),
            DataId::ExampleInputText(problem) => get_example_input_from_html(*problem),
            DataId::ExampleOutputText(problem) => get_example_output_from_html(*problem),
        }
    }
}

fn get_data_with_freshness(id: DataId) -> Result<(String, bool)> {
    if let Some(data) = get_data_from_cache(id)? {
        Ok((data, false))
    } else {
        // fetch from advent of code
        let data = id.get()?;
        // cache
        cache_data(id, &data)?;
        Ok((data, true))
    }
}

fn get_data(id: DataId) -> Result<String> {
    get_data_with_freshness(id).map(|(data, _)| data)
}

fn get_data_from_cache(id: DataId) -> Result<Option<String>> {
    let path = format!("./_cache/{}", id.cache_file_name());
    if !std::path::Path::new(&path).exists() {
        return Ok(None);
    }
    Ok(Some(fs::read_to_string(path)?))
}

fn cache_data(id: DataId, input: &str) -> Result<()> {
    let path = format!("./_cache/{}", id.cache_file_name());
    Ok(fs::write(path, input)?)
}

fn uncache_data(id: DataId) -> Result<()> {
    let path = format!("./_cache/{}", id.cache_file_name());
    Ok(fs::remove_file(path)?)
}

pub fn fetch_data(url: &str) -> Result<String> {
    let client = http_client(&secrets(&SECRETS).session, "text/html")?;
    let res = client.get(url).send()?;
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

use crate::problem::Problem;
pub fn http_client(session_cookie: &str, content_type: &str) -> Result<HttpClient> {
    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim()))?;
    let content_type_header = HeaderValue::from_str(content_type)?;
    let user_agent = "adventer 1.0".to_string();
    let user_agent_header = HeaderValue::from_str(&user_agent)?;

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_type_header);
    headers.insert(USER_AGENT, user_agent_header);

    Ok(HttpClient::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()?)
}

#[derive(Debug)]
pub enum SubmissionOutcome {
    Correct,
    Incorrect,
    Wait,
    WrongLevel,
}

pub fn submit_answer(problem: Problem, answer: &str) -> Result<SubmissionOutcome> {
    let outcome = submit_answer_html(problem, answer)?;
    if outcome.contains("That's the right answer") {
        cache_data(DataId::OutputText(problem), answer.trim())?;
        Ok(SubmissionOutcome::Correct)
    } else if outcome.contains("That's not the right answer") {
        Ok(SubmissionOutcome::Incorrect)
    } else if outcome.contains("You gave an answer too recently") {
        Ok(SubmissionOutcome::Wait)
    } else if outcome.contains("You don't seem to be solving the right level") {
        Ok(SubmissionOutcome::WrongLevel)
    } else {
        anyhow::bail!("Unknown outcome: {}", outcome);
    }
}

fn submit_answer_html(problem: Problem, answer: &str) -> Result<String> {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/answer",
        YEAR, problem.day
    );
    let content_type = "application/x-www-form-urlencoded";
    let response = http_client(&secrets(&SECRETS).session, content_type)?
        .post(url)
        .body(format!(
            "level={}&answer={}",
            problem.part.advent_display(),
            answer
        ))
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())?;

    Ok(response)
}

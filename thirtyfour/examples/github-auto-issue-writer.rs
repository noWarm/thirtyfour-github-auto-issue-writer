//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515 (also confirm that the version is suitable with your browser)
//!
//! Run as follows:
//!
//!     cargo run --example github-auto-issue-writer

use thirtyfour::prelude::*;
use tokio;
use crate::tokio::time::Duration;
use std::fs;
use csv;
use csv::Error;
use csv::Reader;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Record {
    github: String,
    comment: String,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // login to github
    driver.goto("https://github.com/login").await?;
    let elem_form = driver.find(By::ClassName("auth-form-body")).await?;

    let login_field_text = elem_form.find(By::Id("login_field")).await?;
    login_field_text.send_keys("username").await?;
    
    let password_text = elem_form.find(By::Id("password")).await?;
    password_text.send_keys("password").await?;

    let elem_button = driver.find(By::ClassName("btn")).await?;
    elem_button.click().await?;
    tokio::time::sleep(Duration::from_secs(10000)).await;
    
    // csv file to string
    let foo: String = fs::read_to_string("homework_comments.csv")?.parse()?;
    let mut reader = csv::Reader::from_reader(foo.as_bytes());

    // iterate through each student comments
    for record in reader.deserialize() {
        let record: Record = record?;
        let var_url = record.github;        // student github id
        let var_comment = record.comment;   // comments for student's assignment
        
        // Goto the Repository Link
        driver.goto(format!("https://github.com/<insert classroom organization>/<insert-homework-name>-{}/issues/new", var_url)).await?;
        let elem_form = driver.find(By::Id("new_issue")).await?;
    
        let issue_title_text = elem_form.find(By::Id("issue_title")).await?;
        issue_title_text.send_keys("comments and feedback kub").await?;
        
        // Post the feedback as an issue
        let issue_body_text = elem_form.find(By::Id("issue_body")).await?;
        issue_body_text.send_keys(var_comment).await?;
        
        // Click the submit button
        let elem_button = elem_form.find(By::ClassName("btn")).await?;
        elem_button.click().await?;
    }


    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;
    Ok(())
}

use std::io::{BufRead, Write};
use std::iter::once;

use crate::infrastructure::repository_impl::RepositoryImpl;
use crate::interfaces::controller::Controller;
use crate::usecases::repository::{self, Repository};
use crate::usecases::service::{self, Service};
use crate::usecases::service_impl::ServiceImpl;

use clap::Parser;
mod commands;
pub use commands::Cli;
use commands::*;

// コンテストの url または id から、 id に変換する
fn to_contest_id(contest_id_or_url: String) -> String {
    if contest_id_or_url.starts_with("http") {
        contest_id_or_url.split("/").last().unwrap().to_string()
    } else {
        contest_id_or_url
    }
}

// ターミナルプロンプト
pub struct Shell<'s, S: Service> {
    prompt: String,
    controller: Controller<'s, S>,
}

impl<'s, S: Service> Shell<'s, S> {
    pub fn new(service: &'s S, prompt: String, cli: &Cli) -> Self {
        let contest_id = to_contest_id(cli.contest.clone());

        let mut prompt_context = tera::Context::new();
        prompt_context.insert("contest_id", &contest_id);
        let mut tera = tera::Tera::default();
        Self {
            prompt: tera.render_str(&prompt, &prompt_context).unwrap(),
            controller: Controller::new(service),
        }
    }
    fn print_prompt(&self) {
        print!("{}", self.prompt);
    }
    pub fn run(&self) -> i32 {
        let repository = RepositoryImpl::new();
        let service = ServiceImpl::new(&repository);

        let mut stdin_iter = std::io::stdin().lock().lines();
        loop {
            self.print_prompt();
            std::io::stdout().flush().unwrap();

            match stdin_iter.next() {
                Some(r) => {
                    match ShellCommand::try_parse_from(
                        once("").chain(r.unwrap().split_whitespace()),
                    ) {
                        Ok(shell) => match shell.command {
                            Command::Exit(exit_args) => {
                                if exit_args.code == 0 {
                                    println!("bye");
                                }
                                return exit_args.code;
                            }
                            Command::Login(login_args) => {
                                self.controller.login(login_args);
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                }
                None => {
                    break;
                }
            }
        }
        0
    }
}

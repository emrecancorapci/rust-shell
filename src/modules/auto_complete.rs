use std::collections::HashSet;

use crate::shell::core::{QueryResult, ShellAutoComplete, ShellCommandProvider};

pub struct AutoComplete {
    query: String,
    is_tab_clicked: bool,
    tab_state: TabState,
}

impl ShellAutoComplete for AutoComplete {
    fn new() -> Self {
        Self {
            query: String::new(),
            is_tab_clicked: false,
            tab_state: TabState::Idle,
        }
    }
    fn reset(&mut self) {
        self.query.clear();
        self.is_tab_clicked = false;
        self.tab_state = TabState::Idle;
    }
    fn query_command<U, CommandProvider: ShellCommandProvider<U>>(
        &mut self,
        command: &str,
    ) -> Result<QueryResult, std::io::Error> {
        if self.query.is_empty() {
            self.query = command.trim().to_string();
        }

        let builtin_commands = CommandProvider::get_commands();
        let mut result = HashSet::new();
        let mut search_result = CommandProvider::search_command(&self.query)?;

        result.extend(search_result.iter().map(|s| s.to_string()));
        result.extend(
            builtin_commands
                .iter()
                .filter(|c| c.starts_with(&self.query))
                .map(|s| s.to_string()),
        );

        if search_result.len() == 0 {
            return Ok(QueryResult::NoMatch);
        }

        if self.tab_state == TabState::Idle {
            self.tab_state = TabState::ClickedOnce;

            if let Some(res) = search_result.iter().find(|s| s == &&self.query) {
                return Ok(QueryResult::ExactMatch(res.clone()));
            }

            return match search_result.len() {
                1 => Ok(QueryResult::SingleMatch(search_result.pop().unwrap())),
                _ => {
                    let common_prefix = search_result.iter().fold(String::new(), |acc, s| {
                        if acc.is_empty() {
                            s.clone()
                        } else {
                            find_common_prefix(&[acc, s.clone()])
                        }
                    });

                    if common_prefix.len() > self.query.len() {
                        Ok(QueryResult::CommonPrefix(common_prefix))
                    } else {
                        Ok(QueryResult::Bell)
                    }
                }
            };
        }

        if self.tab_state == TabState::ClickedOnce {
            self.tab_state = TabState::ClickedMultipleTimes;

            return match search_result.len() {
                1 => Ok(QueryResult::SingleMatch(search_result.pop().unwrap())),
                _ => Ok(QueryResult::MultipleMatches(search_result)),
            };
        }

        Ok(QueryResult::Bell)
    }
}

#[derive(PartialEq, Eq)]
enum TabState {
    Idle,
    ClickedOnce,
    ClickedMultipleTimes,
}

fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();

    for s in strings.iter().skip(1) {
        let mut new_prefix = String::new();

        for (c1, c2) in prefix.chars().zip(s.chars()) {
            if c1 == c2 {
                new_prefix.push(c1);
            } else {
                break;
            }
        }

        prefix = new_prefix;

        if prefix.is_empty() {
            break;
        }
    }

    prefix
}

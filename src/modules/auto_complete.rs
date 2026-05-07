use std::collections::BTreeSet;

use crate::shell::core::{QueryResult, ShellAutoComplete, ShellCommandProvider};

pub struct AutoComplete {
    query: String,
    tab_state: TabState,
}

impl ShellAutoComplete for AutoComplete {
    fn new() -> Self {
        Self {
            query: String::new(),
            tab_state: TabState::Idle,
        }
    }
    fn reset(&mut self) {
        self.query.clear();
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
        let search_result = CommandProvider::search_command(&self.query)?;
        let mut commands_btree = search_result.iter().cloned().collect::<BTreeSet<_>>();

        commands_btree.extend(
            builtin_commands
                .iter()
                .filter(|c| c.starts_with(&self.query))
                .map(|s| s.to_string()),
        );

        let mut result = commands_btree.into_iter().collect::<Vec<_>>();

        if result.is_empty() {
            return Ok(QueryResult::NoMatch);
        }

        if self.tab_state == TabState::Idle {
            self.tab_state = TabState::ClickedOnce;

            if let Some(res) = result.iter().find(|s| s == &&self.query) {
                return Ok(QueryResult::ExactMatch(res.clone()));
            }

            return match result.len() {
                1 => Ok(QueryResult::SingleMatch(result.pop().unwrap())),
                _ => {
                    let common_prefix = result.iter().fold(String::new(), |acc, s| {
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
        } else {
            self.tab_state = TabState::ClickedMultipleTimes;

            return match result.len() {
                1 => Ok(QueryResult::SingleMatch(result.pop().unwrap())),
                _ => Ok(QueryResult::MultipleMatches(result)),
            };
        }
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

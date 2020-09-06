use crate::*;
use std::thread;
use std::time::Duration;
use clokwerk::{Scheduler, Interval};
use reqwest;
use lettre::{Message, Mailbox};




pub struct Supervisor {
    pub config: Config,
    pub scheduler: Scheduler
}

impl Supervisor {
    /// Creates a supervisor from the config, which creates a Scheduler from each
    /// stored endpoint,
    pub fn from_config(config: Config) -> Self {
        let mut scheduler = Scheduler::new();
        config.endpoint.iter()
                       .for_each(|e|{
                            let mut t = Task::from_endpoint(e);
                            t = t.add_mail_settings(&config.mail);
                            let seconds: u32 = e.interval.unwrap_or_default() as u32;
                            // Add the interval to the scheduler and move the Task there
                            scheduler.every(Interval::Seconds(seconds))
                                     .run(move || {
                                        t.run()
                                     });
                       });
        // Return a fresh Supervisor : )
        Self {
            config,
            scheduler
        }
    }

    pub fn run_loop(mut self) {
        loop {
            self.scheduler.run_pending();
            thread::sleep(Duration::from_millis(1000));
        }
    }
}


#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub url: String,
    interval : usize,
    pub state: State,
    pub mail: Option<Mail>,
    pub contact_mail: Mailbox
}


impl Task {
    pub fn from_endpoint(endpoint: &Endpoint) -> Self {
        Self {
            name: endpoint.name.clone().unwrap_or("Endpoint".to_string()),
            url: endpoint.url.clone(),
            interval: endpoint.interval.unwrap_or_default(),
            state: State::Unchecked,
            mail: None,
            contact_mail: endpoint.contact_mail.clone()
        }
    }

    pub fn add_mail_settings(self, mail: &Mail) -> Self {
        Self {
            name: self.name,
            url: self.url,
            interval: self.interval,
            state: self.state,
            mail: Some(mail.clone()),
            contact_mail: self.contact_mail
        }
    }

    pub fn send_email(&mut self, mail_type: MailType, reason: Option<HttpResponse>) {
        match &self.mail {
            Some(mail) => {
                let email = match mail_type {
                    MailType::ReachableAgain => {
                        // Build Body
                        let body = match reason {
                            Some(r) => format!("{} ({}) is reachable again after {} seconds\n\nReason:\n{:#?}", 
                                self.name, 
                                self.url,
                                self.since_seconds(),
                                r),
                            None => format!("{} ({}) is reachable again after {} seconds\n\nReason was unknown", 
                                self.name, 
                                self.url,
                                self.since_seconds()),
                        };
                        // Build Message
                        Message::builder()
                            .to(self.contact_mail.clone())
                            .from(mail.sender_mail.clone())
                            .subject(format!("[{}] {} reachable again!", env!("CARGO_PKG_NAME"), self.name))
                            .body(body)
                            .unwrap()
                    },
                    MailType::Unreachable => {
                        // Build Body
                        let body = match reason {
                            Some(r) => {
                                match self.since_seconds() {
                                    0 => {
                                        format!("{} ({}) has just become unreachable\n\nReason:\n{:#?}", 
                                            self.name, 
                                            self.url,
                                            r)
                                    },
                                    _ => {
                                        format!("{} ({}) has been unreachable for {} seconds\n\nReason:\n{:#?}", 
                                            self.name, 
                                            self.url, 
                                            self.since_seconds(),
                                            r)
                                    }
                                }
                            },
                            None => {
                                match self.since_seconds() {
                                    0 => {
                                        format!("{} ({}) has just become unreachable\n\nReason unknown", 
                                            self.name, 
                                            self.url)
                                    },
                                    _ => {
                                        format!("{} ({}) has been unreachable for {} seconds\n\nReason unknown", 
                                            self.name, 
                                            self.url, 
                                            self.since_seconds())
                                    }
                                }
                            },
                        };
                        // Build Message
                        Message::builder()
                            .to(self.contact_mail.clone())
                            .from(mail.sender_mail.clone())
                            .subject(format!("[{}] {} unreachable!", env!("CARGO_PKG_NAME"), self.name))
                            .body(body)
                            .unwrap()
                    }
                };
                
                match mail.send(email) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e)
                }
            },
            None => {
                eprintln!("Error: No mail settings in the Task \"{}\". Somebody used the API the wrong way.", self.name)
            }
        }
    }

    pub fn run(&mut self) {
        match reqwest::blocking::get(&self.url) {
            Ok(r) => {
                let status = r.status();
                if status.is_success() || status.is_redirection() {
                    self.was_reachable();
                }else{
                    self.was_not_reachable(format!("{}", status));
                }
                println!("Successful request {}: {} (since {} seconds)", self.name, r.status(), self.since_seconds())
            },
            Err(e) => {
                self.was_not_reachable(format!("{}", e));
                println!("Error requesting {}: {} (since {} seconds)", self.name, e, self.since_seconds());
            }
        };
    }

    pub fn since_seconds(&self) -> usize {
        match self.state {
            State::Reachable(rounds) => {
                rounds * self.interval
            },
            State::Unreachable(rounds, _) => {
                rounds * self.interval
            },
            _ => 0
        }
    }

    pub fn was_reachable(&mut self){
        let state = match self.state {
            State::Reachable(rounds) => State::Reachable(rounds + 1),
            State::Unchecked => State::Reachable(1),
            _ => {
                // Send email after coming back from Unreachable State
                self.send_email(MailType::ReachableAgain, None);
                State::Reachable(1)
            },
        };

        self.state = state;
    }

    pub fn was_not_reachable(&mut self, response: HttpResponse) {
        let state = match self.state {
            State::Unreachable(rounds, _) => State::Unreachable(rounds + 1, response.to_string()),
            _ => {
                self.send_email(MailType::Unreachable, Some(response.to_string()));
                State::Unreachable(1, response.to_string())
            }
        };

        self.state = state;
    }
}


pub type Rounds = usize;
pub type HttpResponse = String;


#[derive(Debug, Clone)]
pub enum State {
    Unchecked,
    Unknown,
    Reachable(Rounds),
    Unreachable(Rounds, HttpResponse)
}

#[derive(Debug, Clone)]
pub enum MailType {
    ReachableAgain,
    Unreachable
}
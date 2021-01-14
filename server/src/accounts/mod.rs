use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;

use netlib::net::tcp::TcpStream;
use netlib::{Interest, PollReactor, Reaction, Reactor, Result};

use crate::connections::Connections;

mod signin;
mod signup;

// -----------------------------------------------------------------------------
//     - Create account reactors -
// -----------------------------------------------------------------------------
pub fn accounts() -> impl Reactor<Input = TcpStream> {
    let acc = Accounts::new();
    acc.chain(AccountRouter::new())
}

// -----------------------------------------------------------------------------
//     - Accounts -
// -----------------------------------------------------------------------------
struct Accounts<T: AsRawFd + Read + Write>(Connections<T>);

impl<T: AsRawFd + Read + Write> Accounts<T> {
    fn new() -> Self {
        Self(Connections::<T>::new())
    }
}

impl<T: AsRawFd + Read + Write> Reactor for Accounts<T> {
    type Input = PollReactor<T>;
    type Output = AccountAction<T>;

    fn react(&mut self, reaction: Reaction<Self::Input>) -> Reaction<Self::Output> {
        match reaction {
            // -----------------------------------------------------------------------------
            //     - New connection -
            // -----------------------------------------------------------------------------
            Reaction::Value(con) => {
                // self.0.insert(con.id, (con, DefaultCodec::new()));
                Reaction::Value(AccountAction::SignIn(con))
                // Reaction::Continue
            }
            // -----------------------------------------------------------------------------
            //     - Connection event -
            // -----------------------------------------------------------------------------
            Reaction::Event(ev) if self.0.contains_key(&ev.owner) => {
                // let msg = r#"
// 1) Sign in
// 2) Sign up
// "#;

                // if let Some(data) = self.0.recv(ev.owner).pop() {
                //     return match data.as_str() {
                //         "1" => {
                //             let (con, _) = self.0.remove(&ev.owner).unwrap();
                //             Reaction::Value(AccountAction::SignIn(con))
                //         }
                //         "2" => {
                //             let (con, _) = self.0.remove(&ev.owner).unwrap();
                //             Reaction::Value(AccountAction::SignUp(con))
                //         }
                //         _ => Reaction::Continue,
                //     };
                // }

                Reaction::Continue
            }
            Reaction::Event(ev) => Reaction::Event(ev),
            Reaction::Continue => Reaction::Continue,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Account router -
//     Allows us to choose between two different
//     paths for a reaction
// -----------------------------------------------------------------------------
enum AccountAction<T: AsRawFd + Read + Write> {
    SignIn(PollReactor<T>),
    SignUp(PollReactor<T>),
}

struct AccountRouter<T: AsRawFd + Read + Write> {
    signin: signin::SignIn<T>,
    signup: signup::SignUp<T>,
}

impl<T: AsRawFd + Read + Write> AccountRouter<T> {
    fn new() -> Self {
        Self {
            signin: signin::SignIn::new(),
            signup: signup::SignUp::new(),
        }
    }
}

impl<T: AsRawFd + Read + Write> Reactor for AccountRouter<T> {
    type Input = AccountAction<T>;
    type Output = PollReactor<T>;

    fn react(&mut self, reaction: Reaction<Self::Input>) -> Reaction<Self::Output> {
        match reaction {
            Reaction::Event(ev) => {
                if let Reaction::Continue = self.signin.react(Reaction::Event(ev)) {
                    Reaction::Continue
                } else {
                    return self.signup.react(Reaction::Event(ev));
                }
            }
            Reaction::Value(val) => match val {
                AccountAction::SignIn(val) => self.signin.react(Reaction::Value(val)),
                AccountAction::SignUp(val) => self.signup.react(Reaction::Value(val)),
            },
            Reaction::Continue => Reaction::Continue,
        }
    }
}

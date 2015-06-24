#[macro_use(handler)]
extern crate chatbot;

use chatbot::Chatbot;
use chatbot::adapter::{IrcAdapter, IrcConfig};
use chatbot::handler::GithubIssueLinker;

fn main() {
    let name = "noobit";
    let mut bot = Chatbot::new(name);

    let echo = handler!("EchoHandler", r"echo .+", |_, msg| {
        Some(msg.to_owned())
    });
    let ping = handler!("PingHandler", r"ping", |_, _| Some("pong".to_owned()));

    let irc = IrcAdapter::new(IrcConfig {
        nickname: Some(format!("{}", name)),
        alt_nicks: Some(vec![format!("{}_", name), format!("{}__", name)]),
        server: Some(format!("irc.freenode.net")),
        channels: Some(vec![format!("#whatme")]),
        .. Default::default()
    });

    bot.add_handler(echo);
    bot.add_addressed_handler(ping);
    bot.add_handler(GithubIssueLinker::new());
    bot.add_adapter(irc);

    bot.run();
}

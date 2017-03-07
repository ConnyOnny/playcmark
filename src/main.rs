extern crate pulldown_cmark;
use pulldown_cmark::*;

use std::io;
use std::io::Read;

use std::env::temp_dir;
use std::fs::File;
use std::io::Write;

use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let parser = Parser::new_ext(&buffer, Options::all());
    let parser2 = Parser::new_ext(&buffer, Options::all());
    let mut events_debugstr = String::new();
    for event in parser {
        let s = format!("{:?}", event);
        events_debugstr.push_str(&s);
        events_debugstr.push_str("<br>\n");
    }
    let mut html_version = String::new();
    html::push_html(&mut html_version, parser2);
    let mut path = temp_dir();
    path.push("playcmark_temp.html");
    let mut file = File::create(&path).unwrap();
    write!(file,
"<!DOCTYPE HTML>
<head>
<style>
body {{
    background: #aaa;
}}
</style>
</head>
<body>
<div style=\"position:absolute; top: 0.5em; left: 2%; width:46%; border:1px solid black; background: white; padding: 0.5em\">
{}
</div>
<div style=\"position:absolute; top: 0.5em; right: 2%; width: 46%; background: #8f8; border: 1px solid black; font-family: monospace; padding: 0.5em\">
{}
</div>
</body>
"
    , html_version, events_debugstr).unwrap();
    Command::new("firefox").arg(path).exec();
}

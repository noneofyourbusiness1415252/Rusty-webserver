#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;
use napi::{Result, *};
use regex::*;
use std::{io::*, *};
#[napi(ts_args_type = "paths: Record<string, (...args: any[]) => string>")]
fn _serve(env: Env, paths: collections::HashMap<String, JsFunction>) -> Result<()> {
  for stream in std::net::TcpListener::bind("0.0.0.0:80")?.incoming() {
    let mut path = vec![];
    let mut stream = stream?;
    let mut reader = BufReader::new(stream.try_clone()?);
    reader.read_until(32, &mut vec![])?;
    reader.read_until(32, &mut path)?;
    let mut vars = vec![];
    let mut path = String::from_utf8_lossy(&path).trim_end().to_string();
    for pattern in paths.keys() {
      let regex = Regex::new(&escape(pattern).replace(r"\*", "(.*)")).unwrap();
      if regex.is_match(&path) {
        for capture in regex.captures(&path).unwrap().iter().skip(1) {
          vars.push(env.create_string(capture.unwrap().as_str())?)
        }
        path = pattern.to_owned();
        break;
      }
    }
    let ext = path.rsplit_once(".").map(|x| x.1);
    stream.write(
      match paths.get(&path) {
        Some(func) => format!(
          "HTTP/1.1 200\r\nContent-Type:{};charset=utf-8\r\nCache-Control:{}\r\nX-Content-Type-Options:nosniff\r\n\r\n{}",
          match ext {
		    Some(ext) => match ext {"html" => "text/html", "css" => "text/css", "js" => "text/javascript", _ => "application/octet-stream"}
			_ => "text/html",
		  },
	      match ext {
		    Some("html")|None => "no-cache",
            _ => "max-age=31536000,immutable"
		  },
          func
            .call(None, &vars)?
            .coerce_to_string()?
            .into_utf8()?
            .as_str()?,
        ),
        _ => "HTTP/1.1 404\r\n\r\n".to_owned(),
      }
      .as_bytes(),
    )?;
  }
  Ok(())
}

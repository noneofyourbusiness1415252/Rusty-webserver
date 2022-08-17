#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;
use napi::{Result, *};
use regex::*;
use std::{collections::*, io::*, *};
#[napi(ts_args_type = "paths: Record<string, (...args: any[]) => string>")]
fn _serve(env: Env, paths: collections::HashMap<String, JsFunction>) -> Result<()> {
  let mut statics = HashMap::new();
  env::set_current_dir("static")?;
  add_all_files(".", &mut statics)?;
  for stream in std::net::TcpListener::bind("0.0.0.0:0")?.incoming() {
    let mut path = vec![];
    let mut stream = stream?;
    let mut reader = BufReader::new(stream.try_clone()?);
    reader.read_until(b'/', &mut vec![])?;
    reader.read_until(b' ', &mut path)?;
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
          func.call(None, &vars)?.coerce_to_string()?.into_utf8()?.as_str()?,
        ),
        _ => match statics.get(&path) {
          Some(content) => format!(
          "HTTP/1.1 200\r\nContent-Type:{};charset=utf-8\r\nCache-Control:{}\r\nX-Content-Type-Options:nosniff\r\n\r\n{}",
          match ext {
		        Some(ext) => match ext {"html" => "text/html", "css" => "text/css", "js" => "text/javascript", _ => "application/octet-stream"}
			      _ => "text/html",
		      },
	        match ext {
		        Some("html")|None => "no-cache",
            _ => "max-age=31536000,immutable"
		      },
          content
        ),
      _ => "HTTP/1.1 404\r\n\r\n".to_owned()},
      }.as_bytes()
    )?;
  }
  Ok(())
}
fn add_all_files(
  path: &str,
  paths: &mut HashMap<String, String>,
) -> Result<HashMap<String, String>> {
  for file in fs::read_dir(path)? {
    let file = file?;
    if file.file_type()?.is_dir() {
      add_all_files(
        &format!("{}/{}", path, file.file_name().into_string().unwrap()),
        paths,
      )?;
    } else {
      let file_path = format!("{}/{}", path, file.file_name().into_string().unwrap());
      paths.insert(
        if file_path.clone().ends_with("index.html") {
          if path == "." { "" } else { &path[2..] }.to_owned()
        } else {
          let (name, ext) = file_path
            .clone()
            .rsplit_once(".")
            .map(|x| (x.0.to_owned(), x.1.to_owned()))
            .unwrap();
          if ext == "html" {
            name[2..].to_string()
          } else {
            file_path[2..].to_string()
          }
        },
        String::from_utf8_lossy(&fs::read(file_path)?).to_string(),
      );
    }
  }
  Ok(paths.clone())
}

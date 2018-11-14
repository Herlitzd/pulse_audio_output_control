use std::process::Command;
use regex::Regex;

pub struct Device {
    pub name: String,
    pub index: String,
    pub active: bool,
}

pub fn get_sources() -> Vec<String> {
  let resp = exec(r#"pacmd list-sink-inputs | grep -oP '(?<=index:).*' | tr -d \" \t\""#);
  resp.split("\n").map(|x| x.to_string()).collect()
}
pub fn get_volume() -> (String, String) {
  let resp = exec(r#"pacmd list-sinks | grep -v 'base' | grep -e 'index' -e 'volume:' | tr -d ' \n\t' | sed 's/\/.*//' | sed 's/\:front-left//'"#);
  let re = Regex::new(r#"\*index:(\d+)volume:(.+)"#).unwrap();
  let caps = re.captures(&resp).unwrap();
  (caps[1].to_string(), caps[2].to_string())
}

pub fn get_sinks() -> Vec<Device> {
  let resp = exec(
    r#"pacmd list-sinks | grep -e 'device.description' -e 'index' |
                       tr -d " \t" | sed -r "s/device.description=+/ name:/g" |
                       column -x | tr -d "\t""#,
  );

  let re = Regex::new(r#"(\*{0,1})index:(\d+) name:"(.+)""#).unwrap();

  let devices = re.captures_iter(&resp).map(|row| {
    let groups = (row.get(1), row.get(2), row.get(3));
    match groups {
      (Some(star), Some(i), Some(name)) => Some(Device {
        index: i.as_str().to_string(),
        name: name.as_str().to_string(),
        active: star.as_str() == "*",
      }),
      _ => None,
    }
  });
  let names: Vec<Device> = devices
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect();
  names
}

pub fn exec(cmd: &str) -> String {
    let out = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect(&format!("failed to execute process: `{}`", cmd));
    String::from_utf8(out.stdout).unwrap()
}
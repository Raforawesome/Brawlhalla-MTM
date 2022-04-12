// Only works for 1 dimensional JSON arrays with string values
pub fn json_parse(json: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let lines = json.lines();
    for line in lines {
        if line != "{" && line != "}" {
            let l = &line.trim()[..line.len() - 2];
            let split: Vec<String> = l.split(":").map(String::from).collect();
            let l: String = split[0].trim().replace("\"", "");
            let r: String = split[1].trim().replace("\"", "");
            map.insert(l, r);
        }
    }
    map
}
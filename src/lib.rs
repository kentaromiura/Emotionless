fn read_until(s: &str, c: Vec<char>) -> Option<(&str, &str)> {
    let mut result = None;
    if let Some(index) = s.chars().position(|character| c.contains(&character)) {
        result = Some(s.split_at(index + 1));
    }
    return result;
}

fn read_until_first_stop(s: &str) -> Option<(&str, &str)> {
    // stop can be ';' (a rule) or '{' (start of block) or end of string (rule or nothing).
    let mut stop = Vec::new();
    stop.push(';');
    stop.push('{');

    read_until(s, stop)
}

fn read_next(str: String) -> Option<(String, String)> {
    let s = str.as_str();
    if let Some((a, b)) = read_until_first_stop(s) {
        if a.contains('{') {
            let mut close_par = Vec::new();
            close_par.push('}');
            if let Some((a2, b)) = read_until(b, close_par) {
                let mut s = String::new();
                s.push_str(a);
                s.push_str(a2);
                return Some((s, b.to_string()));
            } else {
                // return all the string to the end.
                return Some((a.to_string(), b.to_string()));
            }
        } else {
            return Some((a.to_string(), b.to_string()));
        }
    } else {
        if s.len() > 0 {
            return Some((s.to_string(), "".to_string()));
        } else {
            return None;
        }
    }
}

// https://stackoverflow.com/a/50278316
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn process(a: String, i: u32) -> String {
    let as_b32 = format_radix(i, 32);
    let class_name = format!(".km{as_b32}");
    let this_class = class_name.as_str();

    if a.contains('{') {
        // selector list to change.
        if let Some((selector, definitions)) = read_until(a.as_str(), ['{'].to_vec()) {
            let selectors = selector.split(',');

            let mut processed_selectors = selectors
                .map(|s| {
                    //let replaced = ;
                    let mut res = String::new();
                    res.push_str(&s.trim().replace("&", this_class));
                    if res.starts_with(this_class) {
                        return res;
                    }
                    res.insert_str(0, format!("{this_class} ").as_str());
                    res
                })
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(x.as_str());
                    acc.push_str(",");
                    acc
                });
            processed_selectors.push_str(definitions);
            return processed_selectors;
        } else {
            //can't be
        }
        //handled.
    }

    format!("{this_class} {{ {a} }}")
    //a.replace("&", this_class)
}

pub fn next(id: u32, css: String) -> String {
    let mut result = String::new();
    let mut done = false;
    let mut next = css;
    while !done {
        if let Some((a, b)) = read_next(next.clone()) {
            next = b.clone();
            let aa = process(a, id);
            result.push_str(aa.as_str());
        } else {
            done = true
        }
    }
    result
}

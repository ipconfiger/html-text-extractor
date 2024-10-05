use std::collections::HashMap;
use regex::{Regex, Error as RegexError};

fn clear_a_and_tag(line: &str) -> String {
    let re_a = Regex::new(r"<a[^>]*>.*?</a>").unwrap();
    let cleaned_text = re_a.replace_all(line, "");

    let re_tags = Regex::new(r"<[^>]*>").unwrap();
    re_tags.replace_all(&cleaned_text, "").to_string()
}

fn check_nav_link(line: &str) -> bool {
    let re_a = Regex::new(r"<a[^>]*>.*?</a>").unwrap();
    if re_a.is_match(line) {
        let cleaned_text = clear_a_and_tag(line);
        cleaned_text.trim().len() < 1
    } else {
        false
    }
}

fn check_not_pure_tag(line: &str) -> bool {
    let re_tag = Regex::new(r"<[^>]*>").unwrap();
    if !line.is_empty() {
        let cleaned_text = re_tag.replace_all(line, "");
        cleaned_text.trim().len() > 0
    } else {
        false
    }
}

fn window_group(marked: &Vec<usize>, window_size: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut lst_num = marked[0];
    let mut current:Vec<usize> = vec![lst_num];
    for num in &marked[1..marked.len()] {
        let diff = *num - lst_num;
        if diff <= window_size {
            lst_num = *num;
            current.push(lst_num);
        }else{
            result.push(current.clone());
            current = Vec::new();
            lst_num = *num;
            current.push(lst_num);
        }
    }
    result
}

fn process(html: &str) -> Result<String, RegexError> {
    let body_re =Regex::new(r"(?is)<body[^>]*>(.*?)</body>")?;
    let script_style_re = Regex::new(r"<script[^>]*>.*?</script>|<style[^>]*>.*?</style>")?;
    let body_content = body_re.captures(html).map(|c| c.get(1).map(|m| m.as_str()).unwrap_or("")).unwrap_or("");
    let cleaned_content = script_style_re.replace_all(body_content, "");
    let mut cleaned_map = HashMap::new();
    let mut marked = Vec::new();
    for (line_no, line) in cleaned_content.lines().enumerate() {
        if !check_nav_link(line) {
            if check_not_pure_tag(line) {
                marked.push(line_no);
                cleaned_map.insert(line_no, clear_a_and_tag(line).trim().to_string());
            }
        }
    }
    let groups = window_group(&marked, 2usize);
    let mut max_weight = 0;
    let mut max_weight_idx = -1;
    for (gp_no, gp) in groups.iter().enumerate() {
        let weight = gp.iter().map(|&no| cleaned_map[&no].len()).sum();
        if max_weight <= weight {
            max_weight = weight;
            max_weight_idx = gp_no as i32;
        }
    }
    Ok(groups[max_weight_idx as usize].iter().map(|&line_no| cleaned_map[&line_no].clone()).collect::<Vec<String>>().join("\n"))
}


#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
    use super::*;

    #[test]
    fn check_process_len() {
        let client = reqwest::blocking::Client::new();
        // 设置请求头以模拟Chrome浏览器
        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));
            headers
        };
        // 发送GET请求
        if let Ok(response) = client.get("https://quanben.io/n/wenyishidai/1.html").headers(headers).send() {
            // 检查响应状态码
            if response.status().is_success() {
                if let Ok(html) = response.text() {
                    let result = process(html.as_str());
                    assert!(result.is_ok());
                    if let Ok(result) = result {
                        println!("content:\n{}", result.as_str());
                        return assert!(result.len() > 10);
                    }
                }
            }
        }
        assert_eq!("a", "b");
    }
}

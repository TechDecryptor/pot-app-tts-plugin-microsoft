use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, HOST, ORIGIN, USER_AGENT,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        ORIGIN,
        HeaderValue::from_static("https://azure.microsoft.com"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("same-site"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("cors"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("empty"),
    );
    headers.insert(
        HeaderName::from_static("pragma"),
        HeaderValue::from_static("no-cache"),
    );
    headers.insert(
        HeaderName::from_static("referer"),
        HeaderValue::from_static("https://azure.microsoft.com/"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua"),
        HeaderValue::from_static(
            "\"Google Chrome\";v=\"111\", \"Not(A:Brand\";v=\"8\", \"Chromium\";v=\"111\"",
        ),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-mobile"),
        HeaderValue::from_static("?0"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-platform"),
        HeaderValue::from_static("\"macOS\""),
    );
    headers.insert(
        HeaderName::from_static("accept-language"),
        HeaderValue::from_static("zh-CN,zh;q=0.9"),
    );
    headers.insert(
        HeaderName::from_static("cache-control"),
        HeaderValue::from_static("no-cache"),
    );
    headers.insert(
        HeaderName::from_static("authority"),
        HeaderValue::from_static("southeastasia.api.speech.microsoft.com"),
    );

    headers
}

#[no_mangle]
pub fn tts(
    text: &str,
    lang: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    const URL: &str = "https://southeastasia.api.speech.microsoft.com/accfreetrial/texttospeech/acc/v3.0-beta1/vcg/speak";

    let speaker = match needs.get(&format!("{lang}-speaker")) {
        Some(speaker) => format!("{lang}-{speaker}"),
        None => match lang {
            "zh-CN" => "zh-CN-XiaoxiaoNeural".to_string(),
            "zh-TW" => "zh-TW-HsiaoChenNeural".to_string(),
            "en-US" => "en-US-AriaNeural".to_string(),
            "ja-JP" => "ja-JP-NanamiNeural".to_string(),
            "ko-KR" => "ko-KR-SunHiNeural".to_string(),
            _ => return Err("Language not supported".into()),
        },
    };

    let ssml=format!("<speak xmlns=\"http://www.w3.org/2001/10/synthesis\" xmlns:mstts=\"http://www.w3.org/2001/mstts\" version=\"1.0\" xml:lang=\"{lang}\"><voice name=\"{speaker}\"><mstts:express-as><prosody rate=\"1\" pitch=\"0%\">{text}</prosody></mstts:express-as></voice></speak>");
    let body = json!({
        "ttsAudioFormat": "audio-24khz-160kbitrate-mono-mp3",
        "ssml": ssml
    });
    let res = client
        .post(URL)
        .json(&body)
        .headers(construct_headers())
        .send()?
        .bytes()?;

    let result = res.to_vec();

    Ok(json!(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = tts("你好", "zh-CN", needs).unwrap();
        println!("{result}");
    }
}

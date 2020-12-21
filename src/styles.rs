use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Style {
    Mozilla,
    Apple,
    Google,
    Twitter,
    Samsung,
    WhatsApp,
    LG,
    HTC,
    OpenMoji,
    Microsoft,
    Facebook,
    Messenger,
}

lazy_static! {
    static ref STYLES_MAP: HashMap<&'static Style, Regex> = {
        let mut map = HashMap::new();

        let arr = &[
            Style::Apple,
            Style::Google,
            Style::Twitter,
            Style::Samsung,
            Style::WhatsApp,
            Style::LG,
            Style::HTC,
            Style::OpenMoji,
            Style::Microsoft,
            Style::Mozilla,
            Style::Facebook,
            Style::Messenger,
        ];

        for style in arr.iter() {
            map.insert(
                style,
                Regex::new(&format!(
                    "<img.*?srcset=\"(.+?/{style}/.+?)\"",
                    style = style
                ))
                .unwrap(),
            );
        }
        map
    };
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Style::Apple => write!(f, "apple"),
            Style::Google => write!(f, "google"),
            Style::Twitter => write!(f, "twitter"),
            Style::Samsung => write!(f, "samsung"),
            Style::WhatsApp => write!(f, "whatsapp"),
            Style::LG => write!(f, "lg"),
            Style::HTC => write!(f, "htc"),
            Style::OpenMoji => write!(f, "openmoji"),
            Style::Microsoft => write!(f, "microsoft"),
            Style::Mozilla => write!(f, "mozilla"),
            Style::Facebook => write!(f, "facebook/230"),
            Style::Messenger => write!(f, "facebook/65"),
        }
    }
}

impl Style {
    pub fn regex_from_string(string: &str) -> anyhow::Result<&Regex> {
        match string {
            "apple" => Ok(Style::Apple.to_regex()),
            "google" => Ok(Style::Google.to_regex()),
            "twitter" => Ok(Style::Twitter.to_regex()),
            "samsung" => Ok(Style::Samsung.to_regex()),
            "whatsapp" => Ok(Style::WhatsApp.to_regex()),
            "lg" => Ok(Style::LG.to_regex()),
            "htc" => Ok(Style::HTC.to_regex()),
            "openmoji" => Ok(Style::OpenMoji.to_regex()),
            "microsoft" => Ok(Style::Microsoft.to_regex()),
            "mozilla" => Ok(Style::Mozilla.to_regex()),
            "facebook" => Ok(Style::Facebook.to_regex()),
            "messenger" => Ok(Style::Messenger.to_regex()),
            _ => Err(anyhow::anyhow!("not found :'(")),
        }
    }

    fn to_regex(&self) -> &Regex {
        STYLES_MAP.get(&self).unwrap()
    }
}

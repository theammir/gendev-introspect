use napi_derive::napi;
use regex::Regex;

#[napi(object)]
#[derive(Debug, Default)]
pub struct FnSignature {
    /// The topmost docstring comment
    pub title: Option<String>,
    /// Additional generic comments of a function
    pub description: Option<String>,
    pub parameters: Vec<FnParameter>,
    pub comments: Vec<FnComment>,
}

#[napi(object)]
#[derive(Debug, Default)]
pub struct FnParameter {
    pub name: String,
    /// The part of same line comment before the comma
    pub _type: Option<String>,
    /// The part of same line comment after the comma
    pub comment: Option<String>,
}

#[napi(object)]
#[derive(Debug)]
pub struct FnComment {
    pub name: CommentType,
    pub comment: String,
}

#[napi(string_enum)]
#[derive(Debug)]
pub enum CommentType {
    Example,
    Hint,
    Result,
    Returns,
}

const FUNC_TERMINATORS: [&str; 3] = [") {", ") => {", ") => ("];
const PARAM_REGEX: &str = r"^(.+?),?(?:\s*//\s*(.*?))?$";

const ALL_TYPES: [&str; 14] = [
    "string",
    "number",
    "boolean",
    "undefined",
    "function",
    "array",
    "object",
    "null",
    "symbol",
    "char",
    "hash",
    "record",
    "set",
    "map",
];

pub fn try_parse_fn_comment(comment: &str) -> Option<FnComment> {
    let (comment_type, comment_value) = comment.split_once(": ")?;
    let comment_type = comment_type.to_lowercase();

    let comment_type_variant: CommentType;
    if comment_type.ends_with("example") {
        comment_type_variant = CommentType::Example;
    } else if comment_type.ends_with("returns") {
        comment_type_variant = CommentType::Returns;
    } else if comment_type.ends_with("hint") {
        comment_type_variant = CommentType::Hint;
    } else if comment_type.ends_with("result") {
        comment_type_variant = CommentType::Result;
    } else {
        return None;
    }

    Some(FnComment {
        name: comment_type_variant,
        comment: comment_value.to_string(),
    })
}

pub fn try_parse_fn_parameter(line: &str) -> Option<FnParameter> {
    // Group 1 is the parameter name,
    // Group 2 is the optional trimmed comment string
    let param_regex = Regex::new(PARAM_REGEX).unwrap();

    let capture = param_regex.captures(line)?;

    let mut parameter = FnParameter {
        name: capture[1].to_string(),
        ..Default::default()
    };

    let comment_text = capture.get(2);
    if let Some(r_match) = comment_text {
        let match_str = r_match.as_str();
        if let Some((typehint, comment)) = match_str.split_once(", ") {
            if ALL_TYPES.contains(&typehint) {
                parameter._type = Some(typehint.to_string());
                parameter.comment = Some(comment.to_string());
                return Some(parameter);
            }
        }
        parameter.comment = Some(match_str.to_string());
    }

    Some(parameter)
}

#[napi]
/// Assumes the string is at least 3 lines long with valid syntax (the first and
/// the last lines are discarded)
/// Returns `None` if unable to parse into FnSignature
pub fn introspect_plain(fn_text: String) -> Option<FnSignature> {
    let mut signature = FnSignature::default();

    let mut lines: Vec<&str> = fn_text
        .trim()
        .split("\n")
        .map(|i| i.trim())
        .filter(|i| !i.is_empty())
        .collect();

    if lines.len() < 3 {
        return None;
    }

    lines.remove(0); // no useful info here
    lines.truncate(lines.iter().position(|i| FUNC_TERMINATORS.contains(i))?);

    for l in lines {
        // if starts with "//", check if it's a title, description,
        // parameter comment or FnComment
        // otherwise, if satisfies the parameter regex, parse it
        if l.starts_with("//") {
            let l = l.trim_start_matches("//").trim();
            if let Some(fn_comment) = try_parse_fn_comment(l) {
                signature.comments.push(fn_comment);
            } else if signature.title.is_none() {
                signature.title = Some(l.to_string());
            } else if signature.parameters.is_empty() {
                signature
                    .description
                    .get_or_insert(String::new())
                    .push_str(&format!("\n{}", l));

                // FIX: Only worth to do it once.
                let signature_desc = signature.description.as_mut().unwrap();
                *signature_desc = signature_desc.trim().to_string();
            } else {
                let last_param = signature.parameters.last_mut().unwrap();
                last_param
                    .comment
                    .get_or_insert(String::new())
                    .push_str(&format!("\n{}", l));

                // FIX:
                let comment = last_param.comment.as_mut().unwrap();
                *comment = comment.trim().to_string();
            }
        } else if let Some(param) = try_parse_fn_parameter(l) {
            signature.parameters.push(param);
        }
    }

    Some(signature)
}

use lightningcss::{
  dependencies::{Dependency, DependencyOptions},
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
};

#[derive(Debug)]
pub struct ParseResult {
  pub imports: Vec<String>,
  pub urls: Vec<String>,
}

pub fn parse(code: &str, filename: Option<String>) -> ParseResult {
  let stylesheet = StyleSheet::parse(
    code,
    ParserOptions {
      filename: filename.unwrap_or("".to_string()),
      error_recovery: true,
      ..ParserOptions::default()
    },
  )
  .unwrap();

  let to_css_result = stylesheet
    .to_css(PrinterOptions {
      analyze_dependencies: Some(DependencyOptions {
        remove_imports: false,
      }),
      ..Default::default()
    })
    .unwrap();

  let mut result = ParseResult {
    imports: vec![],
    urls: vec![],
  };

  if let Some(deps) = to_css_result.dependencies {
    deps.iter().for_each(|dep| match dep {
      Dependency::Import(import) => {
        result.imports.push(import.url.to_string());
      }
      Dependency::Url(url) => {
        result.urls.push(url.url.to_string());
      }
    })
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = parse(
      r#"
@import "./base.css";

.foo {
  background: url(foo.png);
  width: 32px;

  & .inner {
    background-image: url("./other.png")
  }
}

.bar {
  background: url(bar.png);
}
"#,
      None,
    );

    assert_eq!(result.imports, ["./base.css"]);
    assert_eq!(result.urls, ["foo.png", "./other.png", "bar.png"]);
  }
}

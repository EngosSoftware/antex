use antex::ColorMode;
use std::io::IsTerminal;

#[test]
fn conversion_should_work() {
  let auto = if std::io::stdout().is_terminal() { "On" } else { "Off" };

  let cm: ColorMode = "always".into();
  assert_eq!("On", format!("{:?}", cm));

  let cm: ColorMode = "never".into();
  assert_eq!("Off", format!("{:?}", cm));

  let cm: ColorMode = "auto".into();
  assert_eq!(auto, format!("{:?}", cm));

  let cm: ColorMode = "always".to_string().into();
  assert_eq!("On", format!("{:?}", cm));

  let cm: ColorMode = "never".to_string().into();
  assert_eq!("Off", format!("{:?}", cm));

  let cm: ColorMode = "auto".to_string().into();
  assert_eq!(auto, format!("{:?}", cm));

  let cm: ColorMode = (&"always".to_string()).into();
  assert_eq!("On", format!("{:?}", cm));

  let cm: ColorMode = (&"never".to_string()).into();
  assert_eq!("Off", format!("{:?}", cm));

  let cm: ColorMode = (&"auto".to_string()).into();
  assert_eq!(auto, format!("{:?}", cm));

  let cm: ColorMode = Some("always".to_string()).into();
  assert_eq!("On", format!("{:?}", cm));

  let cm: ColorMode = Some("never".to_string()).into();
  assert_eq!("Off", format!("{:?}", cm));

  let cm: ColorMode = Some("auto".to_string()).into();
  assert_eq!(auto, format!("{:?}", cm));

  let s: Option<String> = None;
  let cm: ColorMode = s.into();
  assert_eq!(auto, format!("{:?}", cm));

  let cm: ColorMode = Some(&"always".to_string()).into();
  assert_eq!("On", format!("{:?}", cm));

  let cm: ColorMode = Some(&"never".to_string()).into();
  assert_eq!("Off", format!("{:?}", cm));

  let cm: ColorMode = Some(&"auto".to_string()).into();
  assert_eq!(auto, format!("{:?}", cm));

  let s: Option<&String> = None;
  let cm: ColorMode = s.into();
  assert_eq!(auto, format!("{:?}", cm));
}

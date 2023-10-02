use super::color::*;

pub trait ToColorString {
    fn as_color(&self, color_mode: &ColorMode) -> ColorString;
    fn to_color_string(&self, color_mode: &ColorMode) -> String;
}

// 既可以给String添加方法，也可以给&str添加方法，使用泛型
#[allow(unused)]
impl<T> ToColorString for T
where
    T: AsRef<str>,
{
    fn as_color(&self, color_mode: &ColorMode) -> ColorString {
        ColorString {
            content: self.as_ref().to_string(),
            color_mode: color_mode.clone(),
        }
    }
    fn to_color_string(&self, color_mode: &ColorMode) -> String {
        self.as_color(color_mode).to_string()
    }
}

/* 实现From特征可以直接使用Color::Red.into()或者ColorMode::from(Color::Red)的形式 */
impl From<Color> for ColorMode {
    fn from(color: Color) -> Self {
        ColorMode {
            front_color: color,
            ..Default::default()
        }
    }
}

impl From<DisplayMode> for ColorMode {
    fn from(mode: DisplayMode) -> Self {
        ColorMode {
            mode,
            ..Default::default()
        }
    }
}

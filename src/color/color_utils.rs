use super::color::*;

pub trait ToColorString {
    fn as_color(&self, color_mode: &ColorMode) -> ColorString;
    fn to_color_string(&self, color_mode: &ColorMode) -> String;
}

// 可以给任何实现了AsRef<str>的类型实现ToColorString trait
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

// 传入一个或者多个FrontColor, BackColor, DisplayMode的值，返回一个ColorMode
// 可以通过set_colormode_for!宏来快捷创建一个ColorMode
// 注意！如果传入的值有相同类型，那么后面的值会覆盖前面的值
#[allow(unused)]
#[macro_export]
macro_rules! set_colormode {
    ($($x:ident => $y:expr),*) => {
        {
            let mut color_mode = ColorMode::default();
            $(
                color_mode.$x = $y;
            )*
            color_mode
        }
    };
    /* 使用格式：
    set_colormode!(front_color => FrontColor::White)
    或者 set_colormode!(
        front_color => FrontColor::White,
        back_color => BackColor::Black,
    )
    注意这里面FrontColor和BackColor都是Color类型，可以使用Color::Red等
    */
}

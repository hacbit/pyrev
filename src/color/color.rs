// 格式 \033[显示方式;前景色;背景色m
//
// 显示模式
// 0（默认值）、1（高亮）、22（非粗体）、4（下划线）、24（非下划线）、5（闪烁）、25（非闪烁）、7（反显）、27（非反显）
// 前景色
// 30（黑色）、31（红色）、32（绿色）、 33（黄色）、34（蓝色）、35（洋红）、36（青色）、37（白色）
// 背景色
// 40（黑色）、41（红色）、42（绿色）、 43（黄色）、44（蓝色）、45（洋红）、46（青色）、47（白色）

#[allow(unused)]
pub struct ColorString {
    pub content: String,
    pub color_mode: ColorMode,
}

#[allow(unused)]
impl ColorString {
    pub fn to_string(&self) -> String {
        format!(
            "\x1b[{};{};{}m{}\x1b[0m",
            self.color_mode.mode.mode(),
            self.color_mode.front_color.front(),
            self.color_mode.back_color.back(),
            self.content
        )
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct ColorMode {
    pub front_color: FrontColor,
    pub back_color: BackColor,
    pub mode: DisplayMode,
}

#[allow(unused)]
impl Default for ColorMode {
    fn default() -> Self {
        ColorMode {
            front_color: FrontColor::White,
            back_color: BackColor::Black,
            mode: DisplayMode::Highlight, // 注意高亮不是系统本身的默认值
        }
    }
}

#[allow(unused)]
pub type FrontColor = Color;

#[allow(unused)]
pub type BackColor = Color;

#[allow(unused)]
#[derive(Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta, // 洋红
    Cyan,    // 青色
    White,
}

/* 可以通过Color::Red.into()或者ColorMode::from(Color::Red)来快捷创建一个ColorMode */
/* 注意！这里只实现了前景色的封装，没有为背景色提供一个高效的简洁的方法 */

#[allow(unused)]
impl From<Color> for ColorMode {
    fn from(color: Color) -> Self {
        ColorMode {
            front_color: color.into(),
            ..Default::default()
        }
    }
}

trait Front {
    fn front(&self) -> u8;
}

trait Back {
    fn back(&self) -> u8;
}

impl Front for FrontColor {
    fn front(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
        }
    }
}

impl Back for BackColor {
    fn back(&self) -> u8 {
        match self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
        }
    }
}

#[allow(unused)]
#[derive(Clone)]
pub enum DisplayMode {
    Default,
    Highlight,
    NonBold,
    Underline,
    NonUnderline,
    Blink,
    NonBlink,
    Reverse,    // 反显
    NonReverse, // 非反显
}

#[allow(unused)]
impl DisplayMode {
    pub fn mode(&self) -> u8 {
        match self {
            DisplayMode::Default => 0,
            DisplayMode::Highlight => 1,
            DisplayMode::NonBold => 22,
            DisplayMode::Underline => 4,
            DisplayMode::NonUnderline => 24,
            DisplayMode::Blink => 5,
            DisplayMode::NonBlink => 25,
            DisplayMode::Reverse => 7,
            DisplayMode::NonReverse => 27,
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

#![feature(plugin)]
#![plugin(clippy)]
#![warn(clippy_pedantic)]
#![allow(print_stdout,shadow_reuse,unused_imports,dead_code,enum_glob_use)]

mod color;
mod style;

use color::*;

use std::convert::From;
use std::ops::Deref;
use std::string::String;
use std::fmt;

/// Colored mean both color or styled
#[derive(Debug,PartialEq, Eq)]
pub struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: style::Style
}

pub trait Colorize {
    // Font Colors
    fn black(self) -> ColoredString;
    fn red(self) -> ColoredString;
    fn green(self) -> ColoredString;
    fn yellow(self) -> ColoredString;
    fn blue(self) -> ColoredString;
    fn magenta(self) -> ColoredString;
    fn purple(self) -> ColoredString;
    fn cyan(self) -> ColoredString;
    fn white(self) -> ColoredString;
    // Background Colors
    fn on_black(self) -> ColoredString;
    fn on_red(self) -> ColoredString;
    fn on_green(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
    fn on_blue(self) -> ColoredString;
    fn on_magenta(self) -> ColoredString;
    fn on_purple(self) -> ColoredString;
    fn on_cyan(self) -> ColoredString;
    fn on_white(self) -> ColoredString;
    // Styles
    fn clear(self) -> ColoredString;
    fn normal(self) -> ColoredString;
    fn bold(self) -> ColoredString;
    fn dimmed(self) -> ColoredString;
    fn italic(self) -> ColoredString;
    fn underline(self) -> ColoredString;
    fn blink(self) -> ColoredString;
    fn reverse(self) -> ColoredString;
    fn hidden(self) -> ColoredString;
}

impl ColoredString {
    pub fn is_plain(&self) -> bool {
        (self.bgcolor.is_none() && self.fgcolor.is_none()
            && self.style == style::CLEAR)
    }
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
            style: style::CLEAR
        }
    }
}

impl Deref for ColoredString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.input
    }
}

impl<'a> From<&'a str> for ColoredString {
    fn from(s: &'a str) -> Self {
        ColoredString {
            input: String::from(s), .. ColoredString::default()
        }
    }
}

macro_rules! def_color {
    ($side:ident: $name: ident => $color: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                $side: Some($color), .. self
            }
        }
    };
}

macro_rules! def_style {
    ($name: ident, $value: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                style: style::Style::from_both(self.style, $value),
                .. self
            }
        }
    };
}

impl Colorize for ColoredString {
    def_color!(fgcolor: black => Color::Black);
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: Some(Color::Red), .. self
        }
    }
    def_color!(fgcolor: green => Color::Green);
    def_color!(fgcolor: yellow => Color::Yellow);
    def_color!(fgcolor: blue => Color::Blue);
    def_color!(fgcolor: magenta => Color::Magenta);
    def_color!(fgcolor: purple => Color::Magenta);
    def_color!(fgcolor: cyan => Color::Cyan);
    def_color!(fgcolor: white => Color::White);

    def_color!(bgcolor: on_black => Color::Black);
    fn on_red(self) -> ColoredString {
        ColoredString {
            bgcolor: Some(Color::Red), .. self
        }
    }
    def_color!(bgcolor: on_green => Color::Green);
    def_color!(bgcolor: on_yellow => Color::Yellow);
    def_color!(bgcolor: on_blue => Color::Blue);
    def_color!(bgcolor: on_magenta => Color::Magenta);
    def_color!(bgcolor: on_purple => Color::Magenta);
    def_color!(bgcolor: on_cyan => Color::Cyan);
    def_color!(bgcolor: on_white => Color::White);

    fn clear(self) -> ColoredString {
        ColoredString {
            input: self.input,
            .. ColoredString::default()
        }
    }
    fn normal(self) -> ColoredString { self.clear() }
    def_style!(bold, style::Styles::Bold);
    def_style!(dimmed, style::Styles::Dimmed);
    def_style!(italic, style::Styles::Italic);
    def_style!(underline, style::Styles::Underline);
    def_style!(blink, style::Styles::Blink);
    def_style!(reverse, style::Styles::Reversed);
    def_style!(hidden, style::Styles::Hidden);
}

macro_rules! def_str_color {
    ($side:ident: $name: ident => $color: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                input: String::from(self),
                $side: Some($color),
                .. ColoredString::default()
            }
        }
    }
}

macro_rules! def_str_style {
    ($name:ident, $style:path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                input: String::from(self),
                style: style::Style::new($style),
                .. ColoredString::default()
            }
        }
    }
}

impl<'a> Colorize for &'a str {
    def_str_color!(fgcolor: black => Color::Black);
    fn red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(Color::Red),
            .. ColoredString::default()
        }
    }
    def_str_color!(fgcolor: green => Color::Green);
    def_str_color!(fgcolor: yellow => Color::Yellow);
    def_str_color!(fgcolor: blue => Color::Blue);
    def_str_color!(fgcolor: magenta => Color::Magenta);
    def_str_color!(fgcolor: purple => Color::Magenta);
    def_str_color!(fgcolor: cyan => Color::Cyan);
    def_str_color!(fgcolor: white => Color::White);

    def_str_color!(bgcolor: on_black => Color::Black);
    fn on_red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            bgcolor: Some(Color::Red),
            .. ColoredString::default()
        }
    }
    def_str_color!(bgcolor: on_green => Color::Green);
    def_str_color!(bgcolor: on_yellow => Color::Yellow);
    def_str_color!(bgcolor: on_blue => Color::Blue);
    def_str_color!(bgcolor: on_magenta => Color::Magenta);
    def_str_color!(bgcolor: on_purple => Color::Magenta);
    def_str_color!(bgcolor: on_cyan => Color::Cyan);
    def_str_color!(bgcolor: on_white => Color::White);

    fn clear(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            style: style::CLEAR,
            .. ColoredString::default()
        }
    }
    fn normal(self) -> ColoredString { self.clear() }
    def_str_style!(bold, style::Styles::Bold);
    def_str_style!(dimmed, style::Styles::Dimmed);
    def_str_style!(italic, style::Styles::Italic);
    def_str_style!(underline, style::Styles::Underline);
    def_str_style!(blink, style::Styles::Blink);
    def_str_style!(reverse, style::Styles::Reversed);
    def_str_style!(hidden, style::Styles::Hidden);
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.is_plain() {
            try!(f.write_str(&self.input));
            return Ok(())
        }

        try!(f.write_str("\x1B["));
        let mut has_wrote = false;

        if self.style != style::CLEAR {
            try!(f.write_str(&self.style.to_str()));
            has_wrote = true;
        }

        if let Some(ref color) = self.bgcolor {
            if has_wrote { try!(f.write_str(";")) }
            try!(f.write_str(color.to_bg_str()));
            has_wrote = true;
        }

        if let Some(ref color) = self.fgcolor {
            if has_wrote { try!(f.write_str(";")) }
            try!(f.write_str(color.to_fg_str()));
        }

        try!(f.write_str("m")   );
        try!(f.write_str(&self.input));
        try!(f.write_str("\x1B[0m"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let toto = "toto";
        println!("{}", toto.red());
        println!("{}", String::from(toto).red());
        println!("{}", toto.blue());

        println!("blue style ****");
        println!("{}", toto.bold());
        println!("{}", "yeah ! Red bold !".red().bold());
        println!("{}", "yeah ! Yellow bold !".bold().yellow());
        println!("{}", toto.bold().blue());
        println!("{}", toto.blue().bold());
        println!("{}", toto.blue().bold().underline());
        println!("{}", toto.blue().italic());
        println!("******");
        println!("test clearing");
        println!("{}", "red cleared".red().clear());
        println!("{}", "bold cyan cleared".bold().cyan().clear());
        println!("******");


        println!("{}", toto.green());
        println!("{}", toto.yellow());
        println!("{}", toto.purple());
        println!("{}", toto.magenta());
        println!("{}", toto.cyan());
        println!("{}", toto.white());
        println!("{}", toto.white().red().blue().green());
        //*/
        assert!(false)
        //expect!(Style::default().paint("default").red().to_string()).to(be_equal_to("default"));
        // assert!("plop".red().to_string() != "plop")
    }
}

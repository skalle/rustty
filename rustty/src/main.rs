use std::io::stdout;

use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    Result,
};

fn main() -> Result<()>{
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled Text here."))?
        .execute(ResetColor)?;

    Ok(())
}

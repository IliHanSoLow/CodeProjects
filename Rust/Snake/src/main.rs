use std::io;
use termit::prelude::*;

fn main() -> io::Result<()> {
  // The Termit facilitator:
  // * Use the Terminal to set up your terminal experience.
  // * Initialize Termit.
  let mut termit = Terminal::try_system_default()?.into_termit::<NoAppEvent>();

  // The TUI tree:
  // * Canvas sets the default style for the contained widget and fills space.
  // * "Hello World!" - &str/String - is a widget.
  // * We just place the text in the middle with some placement constraints.
  let mut ui = Canvas::new(
      "Snake"
          .place()
          .width(6)
          .height(2)
  ).back(Color::blue(false));

  // render and update:
  termit.update(&mut (), Event::Refresh(0), &mut ui);
  //                  |   |                      |
  //   the mutable    |   |                       \- The UI to show off
  //   application    |   |
  // model goes here -/   |
  //                       \- this is an event, 
  //                           here a refresh

  // print       ,- the reduced scope (rectangular window)
  //            |     None => all of the screen
  termit.print(None)
}
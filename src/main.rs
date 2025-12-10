use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use fishtank::{render::Renderer, sim::Simulation};

fn main() -> anyhow::Result<()> {
    // start terminal
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All))?;

    let mut stdout = stdout();

    let (cols, rows) = terminal::size().unwrap();

    let mut sim = Simulation::new(cols as f64, rows as f64);
    let mut renderer = Renderer::new(rows as usize, cols as usize);

    // used to compute dt
    let mut time = std::time::Instant::now();

    loop {
        let dt = time.elapsed();
        time = std::time::Instant::now();

        if crossterm::event::poll(std::time::Duration::from_millis(20))? {
            let event = crossterm::event::read()?;

            match event {
                crossterm::event::Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        // exit the program
                        break;
                    }
                }
                crossterm::event::Event::Mouse(event) => {
                    if matches!(event.kind, MouseEventKind::Down(..)) {
                        sim.add_particle(event.column as f64, event.row as f64);
                    }
                }
                _ => {}
            }
        }

        // render
        execute!(stdout, MoveTo(0, 0))?;

        sim.update(dt.as_secs_f64());
        let output = renderer.render(&sim);

        stdout.write_all(output.as_bytes())?;
        stdout.flush()?;
    }

    // end terminal
    execute!(stdout, Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

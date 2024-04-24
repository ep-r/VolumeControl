use std::io::{self, stdout, BufReader};
use std::fs::File;
use ratatui::{prelude::*, widgets::*};
use rodio::*;
use rodio::cpal::traits::{HostTrait, DeviceTrait};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};


// fn main() -> io::Result<()> {
//     enable_raw_mode()?;
//     stdout().execute(EnterAlternateScreen)?;
//     let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
//
//     let mut should_quit = false;
//     while !should_quit {
//         terminal.draw(ui)?;
//         should_quit = handle_events()?;
//     }
//
//     disable_raw_mode()?;
//     stdout().execute(LeaveAlternateScreen)?;
//     Ok(())
// }

fn main() {
    let devices = list_host_devices();
    println!("# Devices : {:?}", devices);
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let devices = list_host_devices();
    let dev = devices.join("\n");

    frame.render_widget(
        Paragraph::new(dev)
            .block(Block::default().title("Greetings").borders(Borders::ALL)),
        frame.size(),
    );
}

fn list_host_devices() -> Vec<String> {
    let mut device_names = Vec::new();

    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    for device in devices {
        let dev: Device = device.into();
        let dev_name: String = dev.name().unwrap();
        //println!(" # Device : {}", dev_name);
        device_names.push(dev_name);
    }

    return device_names
}
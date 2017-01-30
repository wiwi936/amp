use std::cmp;
use models::application::modes::OpenMode;
use pad::PadStr;
use presenters::current_buffer_status_line_data;
use scribe::Workspace;
use scribe::buffer::Position;
use view::{Colors, StatusLineData, Style, View};

pub fn display(workspace: &mut Workspace, mode: &OpenMode, view: &mut View) {
    // Wipe the slate clean.
    view.clear();

    let buffer_status = current_buffer_status_line_data(workspace);

    if let Some(buf) = workspace.current_buffer() {
        view.draw_buffer(buf, None, None);

        // Draw the status line.
        view.draw_status_line(&vec![
            StatusLineData {
                content: " OPEN ".to_string(),
                style: Style::Default,
                colors: Colors::Inverted,
            },
            buffer_status
        ]);
    }

    // Display an empty result set message.
    if mode.results.is_empty() {
        view.print(&Position{ line: 0, offset: 0 },
                   Style::Default,
                   Colors::Default,
                   &"No matching files found.".pad_to_width(view.width()));
     }

    // Draw the list of search results.
    for (line, result) in mode.results.iter().enumerate() {
        let colors = if line == mode.results.selected_index() {
            Colors::Focused
        } else {
            Colors::Default
        };
        let padded_content = result.as_path().to_str().unwrap().pad_to_width(view.width());
        view.print(&Position{ line: line, offset: 0 },
                   Style::Default,
                   colors,
                   &padded_content);
    }

    // Clear any remaining lines in the result display area.
    for line in cmp::max(mode.results.len(), 1)..5 {
        view.print(&Position{ line: line, offset: 0 },
                   Style::Default,
                   Colors::Default,
                   &String::new().pad_to_width(view.width()));
    }

    // Draw the divider.
    let line = OpenMode::MAX_RESULTS;
    let colors = if mode.insert {
        Colors::Insert
    } else {
        Colors::Inverted
    };
    let padded_content = mode.input.pad_to_width(view.width());
    view.print(&Position{ line: line, offset: 0 },
               Style::Bold,
               colors,
               &padded_content);

    // Place the cursor on the search input line, right after its contents.
    view.set_cursor(Some(Position {
        line: OpenMode::MAX_RESULTS,
        offset: mode.input.len(),
    }));

    // Render the changes to the screen.
    view.present();
}

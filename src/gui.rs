#[allow(unused_imports, dead_code)]
use bobs8085::{
    changes::Changes,
    Simulator,
    assemble,
};

use std::{
    fs::File,
    io::Write,
};

#[allow(unused_imports, dead_code)]
use iced::{
    Element, Fill, Alignment, Length,
    Border, Color, Theme, Font, window, Settings,
    widget::{
        Scrollable, Row, Column, Container,
        row, column, text, button,
        text_editor, scrollable, container,
    },
};

#[derive(Debug, Clone)]
#[allow(unused_imports, dead_code)]
enum Message {
    Assemble,
    RunAll,
    RunStep,
    Edit(text_editor::Action),
}

#[derive(Debug)]
#[allow(unused_imports, dead_code)]
struct State {
    sim: Simulator,
    editor_content: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        State { sim: Simulator::default(), editor_content: text_editor::Content::default() }
    }
}

#[macro_export]
macro_rules! text_center {
    ($x:expr) => {
        text($x).width(Fill).center()
    };
}

fn container_style() -> container::Style {
    container::Style {
        border: Border {
            color: Color::from_rgb(0.0, 0.0, 0.0),
            width: 2.0,
            radius: 2.0.into(),
        },
        background: None,
        text_color: None,
        shadow: Default::default(),
    }
}

fn editor_box (state: &State) -> Column<'_, Message> {
    column![
        text_editor(&state.editor_content)
            .on_action(Message::Edit)
            .height(Fill),
    ].spacing(8)
     .align_x(Alignment::Center)
}

fn io_box (state: &State) -> Scrollable<'_, Message> {
    let header = row![
        text_center!("1"), text_center!("2"),
        text_center!("3"), text_center!("4"),
        text_center!("5"), text_center!("6"),
        text_center!("7"), text_center!("8"),
        text_center!("9"), text_center!("A"),
        text_center!("B"), text_center!("C"),
        text_center!("D"), text_center!("E"),
        text_center!("F"),
    ].spacing(13);

    let mut io_box = column![header];

    let mut i : u16 = 0;
    while i < 0xFF {
        let mut io_row = row![];
        while ((i+1) % 16) != 0 {
            io_row = io_row.push( text_center!(format!("{:02X}", state.sim.io_get8(i as u8))).size(14) );
            i += 1;
        }
        io_box = io_box.push(io_row.spacing(5));
        i+=1;
    }

    scrollable(
        container(io_box.width(350).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container_style())
    )
}

fn memory_box (state: &State) -> Scrollable<'_, Message> {

    let header = row![
        text_center!("1"), text_center!("2"),
        text_center!("3"), text_center!("4"),
        text_center!("5"), text_center!("6"),
        text_center!("7"), text_center!("8"),
        text_center!("9"), text_center!("A"),
        text_center!("B"), text_center!("C"),
        text_center!("D"), text_center!("E"),
        text_center!("F"),
    ].spacing(13);

    let mut mem_box = column![header];

    let mut i = 0xC000;
    while i < 0xCFFF {
        let mut mem_row = row![];
        while ((i+1) % 16) != 0 {
            mem_row = mem_row.push( text(format!("{:02X}", state.sim.mem_get8(i))).width(Fill).size(14) );
            i+=1;
        }
        mem_box = mem_box.push(mem_row.spacing(5));
        i+=1;
    }

    scrollable(
        container(mem_box.width(350).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container_style())
    )
}

fn reg_row (row: Row<'_, Message>) -> Container<'_, Message> {
    container(row)
        .style(|_theme| container_style())
        .padding(5)
        .center_x(500)
}

fn register_box (state: &State) -> Container<'_, Message> {

    let reg_box = column![
        reg_row(row![text_center!("CPU Registers")].padding([10,0])),
        reg_row(row![text_center!("Accumulator: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(7)))]),
        reg_row(row![text_center!("Register B: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(0)))]),
        reg_row(row![text_center!("Register C: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(1)))]),
        reg_row(row![text_center!("Register D: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(2)))]),
        reg_row(row![text_center!("Register E: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(3)))]),
        reg_row(row![text_center!("Register H: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(4)))]),
        reg_row(row![text_center!("Register L: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(5)))]),
        reg_row(row![text_center!("Memory: "), text_center!(format!("{:02X}", state.sim.cpu_get_reg(6)))]),
    ].padding([0, 25])
     .spacing(5);

    container(reg_box).style(|_theme| container_style()).padding([10, 0])
}

fn update (state: &mut State, message: Message) {
    match message {
        Message::RunAll => {
            state.sim.set_pc(0xC000);
            while state.sim.execute() {}
        }
        Message::Edit(action) => state.editor_content.perform(action),
        Message::Assemble => {
            let mut file = File::create("out").unwrap();
            let text = state.editor_content.text();
            let _ = write![file, "{}", text];
            let _ = assemble("out", "out");
            *state = State { 
                sim: Simulator::bus_from_file("bin/out.bin"),
                editor_content: text_editor::Content::with_text(&text.clone()) 
            };
        },
        _ => println!("Option for message: {:?} is not defined!", message),
    }
}

fn view (state: &State) -> Element<'_, Message> {

    let inst_binary = column![text("binary placeholder")].height(Fill);

    let section_1 = column![editor_box(state), inst_binary];

    let section_2 = column![
        register_box(state),
        row![
            button("Assemble").on_press(Message::Assemble),
            button("RunAll").on_press(Message::RunAll)
        ].spacing(10),
    ].spacing(10);

    let mem = column![
        text_center!("Memory"),
        memory_box(state).spacing(5),
    ].height(Fill).width(Length::Fixed(350.0));

    let io = column![
        text_center!("IO"),
        io_box(state).spacing(5),
    ].height(Fill).width(Length::Fixed(350.0));

    let section_3 = column![
        container(mem)
            .padding(10)
            .style(|_theme| container_style()),
        container(io)
            .padding(10)
            .style(|_theme| container_style()),
    ].spacing(25);

    let interface = row![

        section_1
            .width(Fill)
            .align_x(Alignment::Center),
            
        section_2
            .width(Fill)
            .align_x(Alignment::Center),

        section_3,
    ].padding(10).spacing(15);

    interface.into()
}

fn main () -> iced::Result {

    let mut window_settings = window::Settings::default();
    window_settings.size = iced::Size { width: 1200.0, height: 780.0 };
    window_settings.min_size = Some(iced::Size { width: 950.0, height: 600.0 });

    let mut app_settings = Settings::default();
    app_settings.default_font = Font { family: iced::font::Family::Monospace, ..Font::default() };
    app_settings.default_text_size = iced::Pixels(14.0);

    iced::application("bobs8085-gui", update, view)
        .theme(|_| Theme::Oxocarbon)
//          .theme(|_| Theme::Dark)
        .centered()
        .settings(app_settings)
        .window(window_settings)
        .run()
}

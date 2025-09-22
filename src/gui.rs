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
use iced::widget::{
    Scrollable, Row, Column, Container,
    row, column, text, button,
    text_editor, scrollable, container,
    horizontal_space,
};

#[allow(unused_imports, dead_code)]
use iced::{
    Element, Fill, Alignment, Length, Padding,
    Border, Color, Theme, Font, window, Settings,
};

#[allow(unused_imports, dead_code)]
use iced::highlighter::{self, Highlighter};

#[derive(Debug, Clone)]
#[allow(unused_imports, dead_code)]
enum Message {
    Assemble,
    RunAll,
    RunStep,
    Edit(text_editor::Action),
    MemoryPage(u8),
    ForwardStep,
    BackwardStep,
    StopStep,
}

#[derive(Debug)]
#[allow(unused_imports, dead_code)]
struct State {
    sim: Simulator,
    editor_content: text_editor::Content,
    memory_page_number: u8,
    current_memory_page: u8,
    step: u64,
    changes: Vec<Changes>,
}

impl Default for State {
    fn default() -> Self {
        let mut state = State { 
            sim: Simulator::default(),
            editor_content: text_editor::Content::default(),
            memory_page_number: 16,
            current_memory_page: 0,
            step: u64::MAX,
            changes: vec![Changes::default(); 1],
        };
        state.changes[0].cpu.pc = 0xC000;
        state
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
            .height(Fill)
            .with_highlighter()
    ].spacing(8)
     .align_x(Alignment::Center)
}

fn memory_header () -> Row<'static, Message> {

    row![
        horizontal_space(),
        horizontal_space(),
        text_center!("1"), text_center!("2"),
        text_center!("3"), text_center!("4"),
        text_center!("5"), text_center!("6"),
        text_center!("7"), text_center!("8"),
        text_center!("9"), text_center!("A"),
        text_center!("B"), text_center!("C"),
        text_center!("D"), text_center!("E"),
        text_center!("F"),
    ].spacing(13)
}

fn get_io_box (state: &State) -> Column<'_, Message> {

    let mut io_box = column![memory_header()];

    let mut i : u16 = 0;
    while i < 0xFF {
        let mut io_row = row![text(format!("{:02X}: ", i))];
        while ((i+1) % 16) != 0 {
            io_row = io_row.push( text_center!(format!("{:02X}", state.sim.io_get8(i as u8))).size(14) );
            i += 1;
        }
        io_box = io_box.push(io_row.spacing(5));
        i+=1;
    }
    io_box

}

fn get_memory_pages (state: &State) -> Vec<Column<'_, Message>> {

    let mut mem_pages : Vec<Column<'_, Message>> = vec![];
    let mut mem_box = column![memory_header()];

    let mut i = 0xC000;
    while i < 0xCFFF {
        let mut mem_row = row![text(format!("{:04X}: ", i))];
        while ((i+1) % 16) != 0 {
            let mut text = text(format!("{:02X}", state.sim.mem_get8(i)) )
                .width(Fill)
                .size(14);
            if i == state.sim.get_pc() {
                text = text.color(Color::from_rgb(256.0, 0.0, 0.0));
            }
            else if i == state.sim.get_sp() {
                text = text.color(Color::from_rgb(0.0, 256.0, 0.0));
            }
            mem_row = mem_row.push(text);
            i = i.wrapping_add(1);
        }
        mem_box = mem_box.push(mem_row.spacing(5));

        if ((i+1) % 256) == 0 {
            mem_pages.push(mem_box);
            mem_box = column![memory_header()];
        }
        i = i.wrapping_add(1);
    }
    mem_pages
}

fn get_memory_buttons () -> Row<'static, Message> {
    let mut buttons = row![];
    let mut i = 0;
    while i < 16 {
        let button = button( text(format!("{:X}", i)).size(12).center() )
            .on_press(Message::MemoryPage(i))
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(24.0));
        buttons = buttons.push(button);
        i+=1;
    }
    buttons
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

fn flags_box (state: &State) -> Container<'_, Message> {
    let flag_box = row![
        text_center!(format!("s: {}", state.sim.get_flag(0) as u8)),
        text_center!(format!("z: {}", state.sim.get_flag(1) as u8)),
        text_center!(format!("ac: {}", state.sim.get_flag(2) as u8)),
        text_center!(format!("p: {}", state.sim.get_flag(3) as u8)),
        text_center!(format!("cy: {}", state.sim.get_flag(4) as u8)),
    ];

    container(flag_box).style(|_theme| container_style()).padding([10, 0])
}

fn update (state: &mut State, message: Message) {

    match message {
        Message::RunAll => {
            state.sim.set_pc(0xC000);
            while state.sim.execute() {}
        },
        Message::RunStep => {
            state.sim.set_pc(0xC000);
            state.step = 0;
        },
        Message::StopStep => {
            state.step = u64::MAX;
        }
        Message::ForwardStep => {
            let (cpu_old, mem_old, io_old) = state.sim.clone_cpu_bus();
            let status = state.sim.execute();
            if !status {
                state.step = u64::MAX;
            }
            else {
                let diff = state.sim.get_changes(cpu_old, mem_old, io_old);
                state.changes.push(diff);
                state.step += 1;
            }
        },
        Message::BackwardStep => {
            if state.step > 0 {
                state.step -= 1;
                state.sim.restore(&state.changes[state.step as usize]);
            }
        },
        Message::MemoryPage(page) => state.current_memory_page = page,
        Message::Edit(action) => state.editor_content.perform(action),
        Message::Assemble => {
            state.step = u64::MAX;
            let mut file = File::create("program.asm").unwrap();
            let text = state.editor_content.text();
            let _ = write![file, "{}", text];
            let _ = assemble("program.asm", "out");
            state.sim = Simulator::bus_from_file("bin/out.bin");
        },
    }
}

fn view (state: &State) -> Element<'_, Message> {

    let inst_binary = column![text("binary placeholder")].height(Fill);

    let section_1 = column![
        editor_box(state), 
        button("Assemble").on_press(Message::Assemble),
        inst_binary,
    ].spacing(10);

    let control_buttons;
    if state.step == u64::MAX {
        control_buttons = row![
            button("Run All").on_press(Message::RunAll),
            button("Run Step").on_press(Message::RunStep)
        ];
    }
    else {
        control_buttons = row![
            button("Backward").on_press(Message::BackwardStep),
            button("Stop").on_press(Message::StopStep),
            button("Forward").on_press(Message::ForwardStep),
        ];
    }

    let pc_sp = container(
        row![
            text_center!(format!("pc: 0x{:04X}", state.sim.get_pc())),
            text_center!(format!("sp: 0x{:04X}", state.sim.get_sp())),
        ]
    ).style(|_theme| container_style()).padding([10, 0]);

    let section_2 = column![
        register_box(state),
        pc_sp,
        flags_box(state),
        control_buttons.spacing(10),
    ].spacing(10);

    let box_size = Length::Fixed(475.0);
    let mem_box = get_memory_pages(state).remove(state.current_memory_page as usize);
    let scroll = scrollable(
        container(mem_box.width(box_size).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container_style())
    );

    let io_box = scrollable(
        container(get_io_box(state).width(box_size).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container_style())
    );

    let mem = column![
        text_center!("Memory"),
        get_memory_buttons().spacing(5).padding([0,2]),
        scroll.spacing(5),
    ].height(Fill).width(box_size).spacing(5);

    let io = column![
        text_center!("IO"),
        io_box.spacing(5),
    ].height(Fill).width(box_size);

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
        .centered()
        .settings(app_settings)
        .window(window_settings)
        .run()
}

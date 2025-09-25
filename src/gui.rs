use bobs8085::{
    changes::Changes,
    Simulator,
    assemble,
};

use std::{
    fs::File,
    io::Write,
};

use iced::{
    Element, Fill, Alignment, Length,
    Border, Color, Theme, Font, window, Settings,
};

#[allow(unused_imports, dead_code)]
use iced::widget::{
    Scrollable, Row, Column, Container,
    row, column, text, button,
    text_editor, scrollable, container,
    horizontal_space,
};


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
    step: bool,
    changes: Vec<Changes>,
}

impl Default for State {
    fn default() -> Self {
        let mut state = State { 
            sim: Simulator::default(),
            editor_content: text_editor::Content::default(),
            memory_page_number: 16,
            current_memory_page: 0,
            step: false,
            changes: vec![Changes::default(); 1],
        };
        state.changes[0].cpu.pc = 0xC000;
        state
    }
}

impl State {
    
    fn reset_changes(&mut self) {
        self.changes = vec![Changes::default(); 1];
        self.changes[0].cpu.pc = 0xC000;
    }

}


#[macro_export]
macro_rules! text_center {
    ($x:expr) => {
        text($x).width(Fill).center()
    };
}

#[macro_export]
macro_rules! title {
    ($x:expr) => {
        text($x)
            .width(Fill)
            .center()
            .size(16)
            .color(Color::parse("33c3ff").unwrap())
    };
    ($x:expr, $s:expr) => {
        text($x)
            .width(Fill)
            .center()
            .size($s)
            .color(Color::parse("33c3ff").unwrap())
    };
}

#[macro_export]
macro_rules! add_border {
    ($x:expr) => {
        container($x)
            .style(|_theme| container_style())
    };

    ($x:expr, $p:expr) => {
        container($x)
            .style(|_theme| container_style())
            .padding($p)
    }
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
            if i == (state.sim.get_pc().wrapping_sub(1)) {
                text = text.color(Color::from_rgb(255.0, 0.0, 0.0));
            }
            else if i == state.sim.get_sp() {
                text = text.color(Color::from_rgb(0.0, 255.0, 0.0));
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
    container(row.padding(5)).align_x(Alignment::Center).center(Fill)
}

fn register_box (state: &State) -> Container<'_, Message> {

    let reg_box = column![
        reg_row(row![title!("CPU Registers")].padding([10,0])),
        reg_row(row![text("Accumulator: "), text(format!("{:02X}", state.sim.cpu_get_reg(7)))]),
        reg_row(row![text("Register B: "), text(format!("{:02X}", state.sim.cpu_get_reg(0)))]),
        reg_row(row![text("Register C: "), text(format!("{:02X}", state.sim.cpu_get_reg(1)))]),
        reg_row(row![text("Register D: "), text(format!("{:02X}", state.sim.cpu_get_reg(2)))]),
        reg_row(row![text("Register E: "), text(format!("{:02X}", state.sim.cpu_get_reg(3)))]),
        reg_row(row![text("Register H: "), text(format!("{:02X}", state.sim.cpu_get_reg(4)))]),
        reg_row(row![text("Register L: "), text(format!("{:02X}", state.sim.cpu_get_reg(5)))]),
        reg_row(row![text("Memory: "), text(format!("{:02X}", state.sim.cpu_get_reg(6)))]),
        row![text_center!(format!("pc: 0x{:04X}", state.sim.get_pc())), text_center!(format!("sp: 0x{:04X}", state.sim.get_sp()))],
    ].spacing(5);

    add_border!(reg_box).padding([10, 0])
}

fn flags_box (state: &State) -> Container<'_, Message> {
    let flag_box = column![
        title!("Flags"),
        row![
            text_center!(format!("s: {}", state.sim.get_flag(0) as u8)),
            text_center!(format!("z: {}", state.sim.get_flag(1) as u8)),
            text_center!(format!("ac: {}", state.sim.get_flag(2) as u8)),
            text_center!(format!("p: {}", state.sim.get_flag(3) as u8)),
            text_center!(format!("cy: {}", state.sim.get_flag(4) as u8)),
        ],
    ].spacing(10);

    add_border!(flag_box, [10, 0])
}

fn int_color (line : &str, int: bool, mask: bool) -> Container<'_, Message> {
    let target = text(line);
    let mut val = text(format!("{}", int as u8));
    if mask {
        val = val.color(Color::from_rgb(255.0, 0.0, 0.0));
    }
    container(row![target, val]).align_x(Alignment::Center).center_x(Fill)
}

fn interrupts_box (state: &State) -> Container<'_, Message> {

    let int_status = column![
        title!("Interrupts"),
        row![
            text_center!(format!("sod: {}", state.sim.get_sod() as u8)),
            text_center!(format!("sid: {}", state.sim.get_sid() as u8)),
        ]
    ].spacing(10);

    let pending = state.sim.get_pending_int();
    let masked = state.sim.get_masked_int();

    let ints = column![
        int_color("trap:", pending.trap, masked.trap),
        int_color("r7_5:", pending.rst7_5, masked.rst7_5),
        int_color("r6_5:", pending.rst6_5, masked.rst6_5),
        int_color("r5_5:", pending.rst5_5, masked.rst5_5),
        int_color("intr:", pending.intr, masked.intr),
    ].padding([0, 25])
     .spacing(5);

    add_border!(
        column![
            int_status,
            ints,
        ].spacing(10)
    ).padding(10)
}

fn update (state: &mut State, message: Message) {

    match message {
        Message::RunAll => {
            state.reset_changes();
            state.sim.clear_cpu();
            state.sim.set_pc(0xC000);
            while state.sim.execute() {}
        },
        Message::RunStep => {
            state.reset_changes();
            state.sim.clear_cpu();
            state.sim.set_pc(0xC000);
            state.step = true;
        },
        Message::StopStep => {
            state.step = false;
        }
        Message::ForwardStep => {
            let (cpu_old, mem_old, io_old) = state.sim.clone_cpu_bus();
            let status = state.sim.execute();
            if !status {
                state.step = false;
            }
            else {
                let diff = state.sim.get_changes(cpu_old, mem_old, io_old);
                state.changes.push(diff);
            }
        },
        Message::BackwardStep => {
            if state.changes.len() > 1 {
                match &state.changes.pop() {
                    Some(changes) => state.sim.restore(changes),
                    _ => (),
                }
            }
        },
        Message::MemoryPage(page) => state.current_memory_page = page,
        Message::Edit(action) => state.editor_content.perform(action),
        Message::Assemble => {
            state.step = false;
            let mut file = File::create("program.asm").unwrap();
            let text = state.editor_content.text();
            let _ = write![file, "{}", text];
            let _ = assemble("program.asm", "out");
            state.sim = Simulator::bus_from_file("bin/out.bin");
            state.reset_changes();
        },
    }
}

fn view (state: &State) -> Element<'_, Message> {

//      let inst_binary = column![text("binary placeholder")].height(Fill);

    // Section 1
    let section_1 = column![
        editor_box(state), 
        button("Assemble").on_press(Message::Assemble),
//          inst_binary,
    ].spacing(10);



    // Section 2
    let control_buttons;
    if state.step == false {
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

    let section_2 = column![
        register_box(state),
        flags_box(state),
        interrupts_box(state),
        control_buttons.spacing(10),
    ].spacing(10);



    // Section 3
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
        title!("Memory", 20),
        get_memory_buttons().spacing(5).padding([0,2]),
        scroll.spacing(5),
    ].height(Fill).width(box_size).spacing(5);

    let io = column![
        title!("IO", 20),
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



    // Interface
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
    window_settings.min_size = Some(iced::Size { width: 1000.0, height: 660.0 });

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

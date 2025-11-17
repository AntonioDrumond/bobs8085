use crate::gui_lib::{State, Message};

use std::{
    fs, path::Path
};

use iced::{
    Alignment, Border, Color, Fill, FillPortion, Length, Element,
};

use iced::widget::{
    Row, Column, Container, scrollable, Space,
    row, column, text, button,
    text_editor, container,
};

use iced_font_awesome::fa_icon_solid;

#[macro_export]
macro_rules! text_center {
    ($x:expr) => {
        text($x).width(FillPortion(1)).center()
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
        container($x).style(|_theme| container_style())
            .style(|_theme| container_style())
    };

    ($x:expr, $p:expr) => {
        container($x)
            .style(|_theme| container_style())
            .padding($p)
    }
}

#[macro_export]
macro_rules! nav_button {
    ($x:expr, $m:expr) => {
        button($x)
            .style(|_theme, _active| nav_button_style(Color::from_rgb(0.0, 0.0, 0.0)))
            .on_press($m)
    };
    ($x:expr, $m:expr, $c:expr) => {
        button($x)
            .style(|_theme, _active| nav_button_style($c))
            .on_press($m)
    };
}

pub fn container_style() -> container::Style {
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

pub fn nav_button_style(border_color: Color) -> button::Style {
    button::Style {
        border: Border {
            color: border_color,
            width: 2.0,
            radius: 2.0.into(),
        },
        background: None,
        text_color: Color::from_rgb(255.0, 255.0, 255.0), 
        shadow: Default::default(),
    }
}

fn editor_box(state: &State) -> Column<'_, Message> {
    column![
        text_editor(&state.editor_content)
            .on_action(Message::EditText)
            .height(Fill)
    ]
    .spacing(8)
    .align_x(Alignment::Center)
}

fn logging_box(state: &State) -> Column<'_, Message> {
    if state.assemble_error {
        column![text(state.logging_message.clone()).color(Color::from_rgb(1.0, 0.0, 0.0))]
            .spacing(8)
            .align_x(Alignment::Center)
    } else {
        column![]
    }
}

fn memory_header() -> Row<'static, Message> {
    row![
        Space::with_width(Length::FillPortion(2)),
        text(" 0 ").width(Length::FillPortion(1)),
        text(" 1 ").width(Length::FillPortion(1)),
        text(" 2 ").width(Length::FillPortion(1)),
        text(" 3 ").width(Length::FillPortion(1)),
        text(" 4 ").width(Length::FillPortion(1)),
        text(" 5 ").width(Length::FillPortion(1)),
        text(" 6 ").width(Length::FillPortion(1)),
        text(" 7 ").width(Length::FillPortion(1)),
        text(" 8 ").width(Length::FillPortion(1)),
        text(" 9 ").width(Length::FillPortion(1)),
        text(" A ").width(Length::FillPortion(1)),
        text(" B ").width(Length::FillPortion(1)),
        text(" C ").width(Length::FillPortion(1)),
        text(" D ").width(Length::FillPortion(1)),
        text(" E ").width(Length::FillPortion(1)),
        text(" F ").width(Length::FillPortion(1)),
    ]
}

fn get_io_box(state: &State) -> Column<'_, Message> {
    let mut io_box = column![memory_header()];

    let mut i: u16 = 0;
    while i < 0xFF {
        let mut io_row = row![text(format!("{:04X}: ", i))];
        let mut j = 0;
        while j < 16 {
            io_row = io_row.push(text_center!(format!("{:02X}", state.sim.io_get8((i+j) as u8))).size(14));
            j += 1;
        }
        io_box = io_box.push(io_row.spacing(5));
        i += 16;
    }
    io_box
}

fn get_memory_pages(state: &State) -> Vec<Column<'_, Message>> {
    let mut mem_pages: Vec<Column<'_, Message>> = vec![];
    let mut mem_box = column![memory_header()];

    let mut i = 0xC000;
    while i < 0xD000 {

        if (i > 0xC000) && (i % 256 == 0) {
            mem_pages.push(mem_box);
            mem_box = column![memory_header()];
        }

        let mut mem_row = row![text(format!("{:04X}: ", i))];

        let mut j = 0;
        while j < 16 {
            let mut text = text(format!("{:02X}", state.sim.mem_get8(i+j)))
                .width(Fill)
                .size(14);
            if i+j == state.sim.get_pc() {
                text = text.color(Color::from_rgb(255.0, 0.0, 0.0));
            } else if i+j == state.sim.get_sp() {
                text = text.color(Color::from_rgb(0.0, 255.0, 0.0));
            }
            mem_row = mem_row.push(text);
            j += 1;
        }
        mem_box = mem_box.push(mem_row.spacing(5));
        i = i.wrapping_add(16);
    }
    mem_pages
}

fn get_memory_buttons() -> Row<'static, Message> {
    let mut buttons = row![];
    let mut i = 0;
    while i < 16 {
        let button = button(text(format!("{:X}", i)).size(12).center())
            .on_press(Message::MemoryPage(i))
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(24.0));
        buttons = buttons.push(button);
        i += 1;
    }
    buttons
}

fn reg_row(row: Row<'_, Message>) -> Container<'_, Message> {
    container(row/*.padding(5)*/)
        .align_x(Alignment::Center)
        .center(Fill)
}

fn register_box(state: &State) -> Container<'_, Message> {

    let reg_box = column![
        reg_row(row![title!("CPU Registers")].padding([10, 0])),
        reg_row(row![
            text("Accumulator: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(7)))
        ]),
        reg_row(row![
            text("Register B: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(0)))
        ]),
        reg_row(row![
            text("Register C: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(1)))
        ]),
        reg_row(row![
            text("Register D: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(2)))
        ]),
        reg_row(row![
            text("Register E: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(3)))
        ]),
        reg_row(row![
            text("Register H: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(4)))
        ]),
        reg_row(row![
            text("Register L: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(5)))
        ]),
        reg_row(row![
            text("Memory: "),
            text(format!("{:02X}", state.sim.cpu_get_reg(6)))
        ]),
        row![
            text_center!(format!("pc: 0x{:04X}", state.sim.get_pc())),
            text_center!(format!("sp: 0x{:04X}", state.sim.get_sp()))
        ],
    ]
    .spacing(5);

    add_border!(reg_box).padding([10, 0])
}

fn flags_box(state: &State) -> Container<'_, Message> {
    let flag_box = column![
        title!("Flags"),
        row![
            text_center!(format!("s: {}", state.sim.get_flag(0) as u8)),
            text_center!(format!("z: {}", state.sim.get_flag(1) as u8)),
            text_center!(format!("ac: {}", state.sim.get_flag(2) as u8)),
            text_center!(format!("p: {}", state.sim.get_flag(3) as u8)),
            text_center!(format!("cy: {}", state.sim.get_flag(4) as u8)),
        ],
    ]
    .spacing(10);

    add_border!(flag_box, [10, 0])
}

fn int_color(line: &str, int: bool, mask: bool) -> Container<'_, Message> {
    let target = text(line);
    let mut val = text(format!("{}", int as u8));
    if mask {
        val = val.color(Color::from_rgb(255.0, 0.0, 0.0));
    }
    container(row![target, val])
        .align_x(Alignment::Center)
        .center_x(Fill)
}

fn interrupts_box(state: &State) -> Container<'_, Message> {

    let int_status = column![
        title!("Interrupts"),
        row![
            text_center!(format!("sod: {}", state.sim.get_sod() as u8)),
            text_center!(format!("sid: {}", state.sim.get_sid() as u8)),
        ]
    ]
    .spacing(10);

    let pending = state.sim.get_pending_int();
    let masked = state.sim.get_masked_int();

    let ints = column![
        int_color("trap:", pending.trap, masked.trap),
        int_color("r7_5:", pending.rst7_5, masked.rst7_5),
        int_color("r6_5:", pending.rst6_5, masked.rst6_5),
        int_color("r5_5:", pending.rst5_5, masked.rst5_5),
        int_color("intr:", pending.intr, masked.intr),
    ]
    .padding([0, 25])
    .spacing(5);

    add_border!(
        column![int_status, ints,] .spacing(10)
    )
    .padding(10)
}

fn default_interface (state: &State) -> Container<'_, Message> {

    // Section 1
    let mut filename : &str = "No file was selected";
    match state.current_file.file_name() {
        Some(name) => filename = name.to_str().unwrap(),
        None => (),
    }

    let section_1 = column![
        text(format!("{}", filename)).align_x(Alignment::Center),
        editor_box(state), 
        logging_box(state),
        button("Assemble").on_press(Message::Assemble),
    ].spacing(10);

    // Section 2
    let control_buttons;
    if state.step == false {
        control_buttons = row![
            button("Run All").on_press(Message::RunAll),
            button("Run Step").on_press(Message::RunStep)
        ];
    } else {
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
    ]
    .spacing(10);

    // Section 3
    let box_size = Length::Fixed(475.0);
    let mem_box = get_memory_pages(state).remove(state.current_memory_page as usize);
    let scroll = scrollable(
        container(mem_box.width(box_size).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container_style()),
    );

    let io_box = scrollable(
        container(get_io_box(state).width(box_size).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container_style()),
    );

    let mem = column![
        title!("Memory", 20),
        get_memory_buttons().spacing(5).padding([0, 2]),
        scroll.spacing(5),
    ]
    .height(Fill)
    .width(box_size)
    .spacing(5);

    let io = column![title!("IO", 20), io_box.spacing(5),]
        .height(Fill)
        .width(box_size);

    let section_3 = column![
        container(mem).padding(10).style(|_theme| container_style()),
        container(io).padding(10).style(|_theme| container_style()),
    ]
    .spacing(25);

    // Main
    let main = row![
        section_1
            .width(Fill)
            .align_x(Alignment::Center),
            
        section_2
            .width(Fill)
            .align_x(Alignment::Center),
        section_3,
    ]
    .padding(10)
    .spacing(15);

    container(main).into()
}

fn openfile_interface(state: &State) -> Container<'_, Message> {

    let cwd = Path::new(&state.cwd);

    let mut parent = cwd.to_path_buf();
    parent.pop();

    let header = column![
        text(format!("Current Directory: {}", cwd.to_str().unwrap().to_string())).size(20), 
        row![
            button(
                fa_icon_solid("arrow-up")
                .size(18.0)
                .color(Color::from_rgb(0.0, 0.0, 0.0))
            )
            .on_press(Message::NavigateTo(parent)),

            button(
                fa_icon_solid("house")
                .size(18.0)
                .color(Color::from_rgb(0.0, 0.0, 0.0))
            )
            .on_press(Message::NavigateTo(state.simulator_path.clone())),

        ].spacing(10)
    ].spacing(10).width(Fill);

    let mut cwd_box = column![]
        .spacing(8)
        .width(Fill);

    if cwd.is_dir() {

        let mut dir_row : Row<'_, Message> = row![];
        let mut n = 0;
        let max_rows = 4;

        for entry in cwd.read_dir().expect("The directory could not be read!") {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let mut dir = entry.file_name().to_str().unwrap().to_string();
                    if dir.len() > 30 {
                        dir = dir[0..30].to_string();
                        dir.push_str("...");
                    }
                    dir_row = dir_row.push(
                        nav_button!(
                            row![
                                fa_icon_solid("folder-open").size(18.0),
                                text(format!("{}", dir)).size(18),
                            ].spacing(5),
                            Message::NavigateTo(entry.path()),
                            Color::from_rgb(0.0, 0.0, 0.0)
                        )
                    );
                    n += 1;
                    if n >= max_rows {
                        cwd_box = cwd_box.push(dir_row.spacing(15));
                        dir_row = row![];
                        n = 0;
                    }
                }
            }
        }
        for entry in cwd.read_dir().expect("The directory could not be read!") {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let mut dir = entry.file_name().to_str().unwrap().to_string();
                    if dir.len() > 30 {
                        dir = dir[0..30].to_string();
                        dir.push_str("...");
                    }
                    if entry.path() != state.selected_file {
                        dir_row = dir_row.push(
                            nav_button!(
                                text(format!("{}", dir)).size(18),
                                Message::SelectFile(entry.path())
                            )
                        );
                    } else {
                        dir_row = dir_row.push(
                            button(text(format!("{}", dir)).size(18))
                                .on_press(Message::SelectFile(entry.path()))
                        );
                    }
                    n += 1;
                    if n >= max_rows {
                        cwd_box = cwd_box.push(dir_row.spacing(15));
                        dir_row = row![];
                        n = 0;
                    }
                }
            }
        }
    }

    let main = column![

        header.height(Length::FillPortion(1)),
        add_border!(scrollable(cwd_box), 10).height(Length::FillPortion(10)),

        container(
            button(text("Open"))
                .on_press(Message::OpenFile(state.selected_file.clone()))
        ).height(Length::FillPortion(1))
    ].align_x(Alignment::Center)
     .spacing(40);

    add_border!(main, 10)
        .width(Fill)
        .into()
}

fn help_interface(state: &State)  -> Container<'_, Message> {

    let mut main = column![];
    if state.simulator_path.exists() && state.simulator_path.is_dir() {
        let mut path = state.simulator_path.clone();
        path.push("man");
        match state.current_help_page {
            1 => {
                path.push("branching.txt");
                match fs::read_to_string(path) {
                    Ok(content) => main = main.push(text(content)),
                    Err(err) => eprintln!("{}", err),
                }
            },
            2 => {
                path.push("control.txt");
                match fs::read_to_string(path) {
                    Ok(content) => main = main.push(text(content)),
                    Err(err) => eprintln!("{}", err),
                }
            },
            3 => {
                path.push("data_transfer.txt");
                match fs::read_to_string(path) {
                    Ok(content) => main = main.push(text(content)),
                    Err(err) => eprintln!("{}", err),
                }
            },
            4 => {
                path.push("logical.txt");
                match fs::read_to_string(path) {
                    Ok(content) => main = main.push(text(content)),
                    Err(err) => eprintln!("{}", err),
                }
            },
            0 | _ => {
                path.push("arithmetic.txt");
                match fs::read_to_string(path) {
                    Ok(content) => main = main.push(text(content)),
                    Err(err) => eprintln!("{}", err),
                }
            },
        };
    };
    container(scrollable(add_border!(main, 20))).into()
}

pub fn view (state: &State) -> Element<'_, Message> {

    let header: Container<'_, Message>;
    let main: Container<'_, Message>;
    match state.interface {
        0x1 => {    // Open file
            header = add_border![row![
                button(
                    fa_icon_solid("arrow-left-long")
                    .size(18.0)
                    .color(Color::from_rgb(0.0, 0.0, 0.0))
                ).on_press(Message::SetInterface(0x0)),
            ].spacing(5), 10].width(Fill);
            main = openfile_interface(state);
        },
        0x2 => {    // Help
            header = add_border![row![
                button(
                    fa_icon_solid("arrow-left-long")
                    .size(18.0)
                    .color(Color::from_rgb(0.0, 0.0, 0.0))
                ).on_press(Message::SetInterface(0x0)),

                Space::with_width(Length::Fill),

                button("Arithmetic").on_press(Message::HelpPage(0)),
                button("Branching").on_press(Message::HelpPage(1)),
                button("Control").on_press(Message::HelpPage(2)),
                button("Data Transfer").on_press(Message::HelpPage(3)),
                button("Logical").on_press(Message::HelpPage(4)),

            ].width(Fill).spacing(5), 10];
            main = help_interface(state);
        },
        0x0 | _ => {    // Simualtor
            header = add_border![row![
                button(text("Open File")).on_press(Message::SetInterface(0x1)),
                button(text("Save File")).on_press(Message::SaveFile),
                button(text("Help")).on_press(Message::SetInterface(0x2)),
            ].spacing(5), 10].width(Fill);
            main = default_interface(state);
        },
    }
    column![
        header,
        main,
    ].padding(10).spacing(5).into()
}

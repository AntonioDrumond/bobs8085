#[allow(unused_imports, dead_code)]
use bobs8085::{Simulator, assemble, changes::Changes};

use std::{fs::File, io::Write};

#[allow(unused_imports, dead_code)]
use iced::{
    Alignment, Application, Border, Color, Element, Fill, Length, Theme, theme,
    widget::{
        Column, Row, Scrollable, button, center, column, container, mouse_area, opaque, row,
        scrollable, stack, text, text_editor,
    },
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ContextModal {
    File,
    Edit,
}

#[derive(Debug, Clone)]
#[allow(unused_imports, dead_code)]
enum Message {
    Assemble,
    RunAll,
    RunStep,
    Edit(text_editor::Action),
    ContextOpen(ContextModal),
}

#[derive(Debug)]
#[allow(unused_imports, dead_code)]
struct State {
    sim: Simulator,
    editor_content: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        let sim = Simulator::bus_from_file("bin/fibonacci.bin");
        State {
            sim,
            editor_content: text_editor::Content::default(),
        }
    }
}

fn theme() -> Theme {
    Theme::TokyoNight
}

fn editor_box(state: &State) -> Column<'_, Message> {
    column![
        text_editor(&state.editor_content)
            .on_action(Message::Edit)
            .height(Fill),
        row![
            button("Assemble").on_press(Message::Assemble),
            button("RunAll").on_press(Message::RunAll)
        ]
        .spacing(10),
    ]
    .spacing(8)
    .align_x(Alignment::Center)
}

fn memory_box(state: &State) -> Scrollable<'_, Message> {
    let header = row![
        text("1").width(Fill),
        text("2").width(Fill),
        text("3").width(Fill),
        text("4").width(Fill),
        text("5").width(Fill),
        text("6").width(Fill),
        text("7").width(Fill),
        text("8").width(Fill),
        text("9").width(Fill),
        text("A").width(Fill),
        text("B").width(Fill),
        text("C").width(Fill),
        text("D").width(Fill),
        text("E").width(Fill),
        text("F").width(Fill),
    ]
    .spacing(13);

    let mut mem_box = column![header];

    let mut i = 0xC000;
    while i < 0xCFFF {
        let mut mem_row = row![];
        while ((i + 1) % 16) != 0 {
            mem_row = mem_row.push(
                text(format!("{:02X}", state.sim.mem_get8(i)))
                    .width(Fill)
                    .size(14),
            );
            i += 1;
        }
        mem_box = mem_box.push(mem_row.spacing(5));
        i += 1;
    }

    let mem_box = scrollable(
        container(mem_box.width(350).align_x(Alignment::Center))
            .padding(5)
            .style(|_theme| container::Style {
                border: Border {
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                background: None,
                text_color: None,
                shadow: Default::default(),
            }),
    );

    mem_box.height(Length::Fixed(500.0)).spacing(5)
}

fn register_box(state: &State) -> Column<'_, Message> {
    column![
        row![text("Register").width(Fill), text("Value").width(Fill)],
        row![
            text("Accumulator").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(7))).width(Fill)
        ],
        row![
            text("Register B").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(0))).width(Fill)
        ],
        row![
            text("Register C").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(1))).width(Fill)
        ],
        row![
            text("Register D").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(2))).width(Fill)
        ],
        row![
            text("Register E").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(3))).width(Fill)
        ],
        row![
            text("Register H").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(4))).width(Fill)
        ],
        row![
            text("Register L").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(5))).width(Fill)
        ],
        row![
            text("Memory").width(Fill),
            text(format!("{:02X}", state.sim.cpu_get_reg(6))).width(Fill)
        ],
    ]
    .height(Fill)
    .align_x(Alignment::Center)
}

fn context_menu() -> Row<'static, Message> {
    row![
        button("File").on_press(Message::ContextOpen(ContextModal::File)),
        button("Edit").on_press(Message::ContextOpen(ContextModal::Edit))
    ]
    .width(Fill)
    .spacing(5)
    .padding(5)
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.0,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::RunAll => {
            state.sim.set_pc(0xC000);
            state.sim.execute();
        }
        Message::Edit(action) => state.editor_content.perform(action),
        Message::Assemble => {
            let mut file = File::create("out").unwrap();
            let text = state.editor_content.text();
            let _ = write![file, "{}", text];
            let _ = assemble("out", "out");
            *state = State {
                sim: Simulator::bus_from_file("bin/out.bin"),
                editor_content: text_editor::Content::with_text(&text.clone()),
            };
        }
        _ => println!("Option for message: {:?} is not defined!", message),
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let inst_binary = column![text("binary placeholder")].height(Fill);
    let inst_box = column![editor_box(state), inst_binary];
    let regs_box = column![register_box(state)];

    let content = column![
        row![context_menu()],
        row![
            inst_box.width(Fill).align_x(Alignment::Center),
            regs_box.width(Fill).align_x(Alignment::Center),
            memory_box(state)
        ]
        .padding(10)
        .spacing(5)
    ];
    content.into()
}

fn main() -> iced::Result {
    iced::application("bobs8085-gui", update, view)
        .theme(|_| theme())
        .centered()
        .run()
}

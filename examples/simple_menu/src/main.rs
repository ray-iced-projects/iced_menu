
use iced_menu::{Menu, MenuMessage, MenuConfig, MenuItem, MenuTooltip};
use iced::widget::container;
use iced::{Element, Task, Theme, Widget, window};
use std::rc::Rc;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::TokyoNight)
        .run()
}

// User-defined enum for messages that custom menu widgets can emit
#[derive(Debug, Clone)]
enum MenuAction {
    SetDarkMode(bool),
}

// Holds all state that custom menu widgets need to reflect
#[derive(Debug, Clone, Default)]
struct State {
    is_dark_mode: bool,
}

struct App {
    menu: Menu<'static, MenuAction>,
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMessage<MenuAction>),
}

impl App {
    fn default() -> Self {
        let state = State::default();
        let configs = define_menu_configs(&state);
        let len = configs.len();
        let mut menu = Menu::default();
        menu.configs = configs;
        menu.is_open = vec![false; len];
        Self {
            menu,
            state,
        }
    }
}

impl App {
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Menu(mm) => match mm {
                MenuMessage::ItemPressed(label, menubar_index) => {
                    let menubar_name = menubar_index
                        .and_then(|idx| self.menu.configs.get(idx).map(|c| c.label))
                        .unwrap_or("Unknown");
                    println!("{} > {}", menubar_name, &label);
                    self.menu.clear_timers();
                    if label == "Exit" {
                        window::latest().and_then(window::close)
                    } else {
                        Task::none()
                    }
                }
                MenuMessage::Custom(action) => match action {
                    MenuAction::SetDarkMode(checked) => {
                        self.state.is_dark_mode = checked;
                        // Rebuild configs so widgets reflect the new state
                        self.menu.configs = define_menu_configs(&self.state);
                        println!("Dark mode: {}", self.state.is_dark_mode);
                        Task::none()
                    }
                }
                _ => self.menu.update(mm).map(Message::Menu)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let menu = self.menu.construct_menu().map(Message::Menu);
        container(menu)
            .padding(10.0)
            .boxed()
    }
}

fn define_menu_configs(state: &State) -> Vec<MenuConfig<'static, MenuAction>> {
    let size = 14.0; 
    vec![        
        MenuConfig {
            label: "File",
            width: 250.0,
            items: vec![
                MenuItem::Item {
                    label: "New File", size},
                MenuItem::Item {
                    label:"New Window", size},
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "New Window with Profile >",
                        width: 200.0,
                        items: vec![
                            MenuItem::Item { label: "Profile 1", size },
                            MenuItem::Item { label: "Profile 2", size },
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: 100.0,
                                    items: vec![
                                        MenuItem::Item {
                                            label: "Profile 3",
                                            size },
                                        MenuItem::Item {
                                            label: "Profile 4",
                                            size },
                                        MenuItem::Item {
                                            label: "Profile 5",
                                            size },
                                    ],
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Open File", size },
                MenuItem::Item { label: "Open Folder", size },
                MenuItem::Item { label: "Open Wokspace from File", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent >",
                        width: 240.0,
                        items: vec![
                            MenuItem::Item {
                                label: "~/document/iced_menu", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Save", size },
                MenuItem::Item { label: "Save As...", size },
                MenuItem::Item { label: "Save All", size },
                MenuItem::Item { label: "Revert File", size },
                MenuItem::Item { label: "Exit", size },
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            width: 200.0,
            gap: None,
            items: vec![
                MenuItem::Item { label: "Undo", size },
                MenuItem::Item { label: "Redo", size },
                MenuItem::Item { label: "Cut", size },
                MenuItem::Item { label: "Copy", size },
                MenuItem::Item { label: "Paste", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Find >",
                        width: 150.0,
                        items: vec![
                            MenuItem::Item { label: "Find", size },
                            MenuItem::Item { label: "Find Next", size },
                            MenuItem::Item { label: "Find Previous", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Item { label: "Replace", size },
                MenuItem::Item { label: "Find in Files", size },
                MenuItem::Item { label: "Replace in Files", size },
                MenuItem::Element(Rc::new(dark_mode_toggle(state.is_dark_mode))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Functions",
            width: 150.0,
            scrollable_height: Some(300.0),
            items: functions()
                .into_iter()
                .map(|(name, signature, description)| {
                    let tooltip_label: &'static str =
                        Box::leak(format!("{signature}\n{description}").into_boxed_str());

                    MenuItem::ItemWithTooltip {
                        label: name,
                        size,
                        tooltip: MenuTooltip {
                            label: tooltip_label,
                            position: iced::widget::tooltip::Position::Right,
                        },
                    }
                })
                .collect(),
            ..Default::default()
        }
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || {
        iced::widget::rule::horizontal(2.0).boxed()
    }
}


fn dark_mode_toggle(checked: bool) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        iced::widget::checkbox(checked)
            .label("Dark Mode")
            .on_toggle(|b| MenuMessage::Custom(MenuAction::SetDarkMode(b)))
            .boxed()
    }
}

fn functions() -> Vec<(&'static str, &'static str, &'static str)> {
    let fs = vec![
        ("ABS", "ABS(number)", "Absolute value of a number."),
        ("ACOS", "ACOS(number)", "Arccosine in radians."),
        ("ACOSH", "ACOSH(number)", "Inverse hyperbolic cosine."),
        ("ACOT", "ACOT(number)", "Arccotangent in radians (0 to pi)."),
        ("ACOTH", "ACOTH(number)", "Inverse hyperbolic cotangent."),
        ("ARABIC", "ARABIC(text)", "Converts Roman numerals to a number."),
        ("ASIN", "ASIN(number)", "Arcsine in radians."),
        ("ASINH", "ASINH(number)", "Inverse hyperbolic sine."),
        ("ATAN", "ATAN(number)", "Arctangent in radians."),
        ("ATAN2", "ATAN2(x_num, y_num)", "Angle of the point (x, y) from the x-axis in radians."),
        ("ATANH", "ATANH(number)", "Inverse hyperbolic tangent."),
        ("BASE", "BASE(number, radix, [min_length])", "Writes a number in another base, padded with zeros."),
        ("CEILING.MATH", "CEILING.MATH(number, [significance], [mode])", "Rounds a number up to the nearest integer or multiple."),
        ("CEILING.PRECISE", "CEILING.PRECISE(number, [significance])", "Rounds a number toward positive infinity to a multiple."),
        ("COMBIN", "COMBIN(number, number_chosen)", "Number of combinations without repetition."),
        ("COMBINA", "COMBINA(number, number_chosen)", "Number of combinations with repetition."),
        ("COS", "COS(number)", "Cosine of an angle in radians."),
        ("COSH", "COSH(number)", "Hyperbolic cosine."),
        ("COT", "COT(number)", "Cotangent of an angle in radians."),
        ("COTH", "COTH(number)", "Hyperbolic cotangent."),
        ("CSC", "CSC(number)", "Cosecant of an angle in radians."),
        ("CSCH", "CSCH(number)", "Hyperbolic cosecant."),
        ("DECIMAL", "DECIMAL(text, radix)", "Reads text in a given base as a number."),
        ("DEGREES", "DEGREES(angle)", "Converts radians to degrees."),
        ("EVEN", "EVEN(number)", "Rounds away from zero to the nearest even integer."),
        ("EXP", "EXP(number)", "e raised to the given power."),
        ("FACT", "FACT(number)", "Factorial of a number."),
        ("FACTDOUBLE", "FACTDOUBLE(number)", "Double factorial of a number."),
        ("FLOOR.MATH", "FLOOR.MATH(number, [significance], [mode])", "Rounds a number down to the nearest integer or multiple."),
        ("FLOOR.PRECISE", "FLOOR.PRECISE(number, [significance])", "Rounds a number toward negative infinity to a multiple."),
        ("GCD", "GCD(number1, [number2], ...)", "Greatest common divisor of integers."),
        ("INT", "INT(number)", "Rounds a number down to the nearest integer."),
        ("ISO.CEILING", "ISO.CEILING(number, [significance])", "Rounds a number toward positive infinity to a multiple."),
        ("LCM", "LCM(number1, [number2], ...)", "Least common multiple of integers."),
        ("LN", "LN(number)", "Natural logarithm."),
        ("LOG", "LOG(number, [base])", "Logarithm to the given base (10 by default)."),
        ("LOG10", "LOG10(number)", "Base-10 logarithm."),
        ("MDETERM", "MDETERM(array)", "Determinant of a square matrix."),
        ("MINVERSE", "MINVERSE(array)", "Inverse of a square matrix."),
        ("MMULT", "MMULT(array1, array2)", "Matrix product of two arrays."),
        ("MOD", "MOD(number, divisor)", "Remainder of a division, with the sign of the divisor."),
        ("MROUND", "MROUND(number, multiple)", "Rounds a number to the nearest multiple."),
        ("MULTINOMIAL", "MULTINOMIAL(number1, [number2], ...)", "Ratio of the factorial of a sum to the product of factorials."),
        ("MUNIT", "MUNIT(dimension)", "Identity matrix of the given size."),
        ("ODD", "ODD(number)", "Rounds away from zero to the nearest odd integer."),
        ("PERCENTOF", "PERCENTOF(data_subset, data_all)", "The sum of a subset as a fraction of the sum of all the values."),
        ("PI", "PI()", "The constant pi to 15 digits."),
        ("POWER", "POWER(number, power)", "Raises a number to a power."),
        ("PRODUCT", "PRODUCT(number1, [number2], ...)", "Multiplies all the numbers in the arguments."),
        ("QUOTIENT", "QUOTIENT(numerator, denominator)", "Integer part of a division."),
        ("RADIANS", "RADIANS(angle)", "Converts degrees to radians."),
        ("RAND", "RAND()", "A random number between 0 and 1, recalculated every time."),
        ("RANDBETWEEN", "RANDBETWEEN(bottom, top)", "A random integer between two bounds, inclusive."),
        ("ROMAN", "ROMAN(number, [form])", "Converts a number to Roman numerals in one of five styles."),
        ("ROUND", "ROUND(number, num_digits)", "Rounds a number to a given number of digits, halves away from zero."),
        ("ROUNDDOWN", "ROUNDDOWN(number, num_digits)", "Rounds a number toward zero to a given number of digits."),
        ("ROUNDUP", "ROUNDUP(number, num_digits)", "Rounds a number away from zero to a given number of digits."),
        ("SEC", "SEC(number)", "Secant of an angle in radians."),
        ("SECH", "SECH(number)", "Hyperbolic secant."),
        ("SERIESSUM", "SERIESSUM(x, n, m, coefficients)", "Sum of a power series."),
        ("SIGN", "SIGN(number)", "1 for positive, -1 for negative and 0 for zero."),
        ("SIN", "SIN(number)", "Sine of an angle in radians."),
        ("SINH", "SINH(number)", "Hyperbolic sine."),
        ("SQRT", "SQRT(number)", "Positive square root."),
        ("SQRTPI", "SQRTPI(number)", "Square root of the number times pi."),
        ("SUM", "SUM(number1, [number2], ...)", "Adds all the numbers in the arguments."),
        ("SUMIF", "SUMIF(range, criteria, [sum_range])", "Adds the cells that meet a condition."),
        ("SUMIFS", "SUMIFS(sum_range, criteria_range1, criteria1, ...)", "Adds the cells that meet every condition."),
        ("SUMPRODUCT", "SUMPRODUCT(array1, [array2], ...)", "Multiplies matching elements of equally sized arrays and adds the products."),
        ("SUMSQ", "SUMSQ(number1, [number2], ...)", "Adds the squares of the numbers."),
        ("SUMX2MY2", "SUMX2MY2(array_x, array_y)", "Sums the differences of squares of matching elements."),
        ("SUMX2PY2", "SUMX2PY2(array_x, array_y)", "Sums the sums of squares of matching elements."),
        ("SUMXMY2", "SUMXMY2(array_x, array_y)", "Sums the squared differences of matching elements."),
        ("TAN", "TAN(number)", "Tangent of an angle in radians."),
        ("TANH", "TANH(number)", "Hyperbolic tangent."),
        ("TRUNC", "TRUNC(number, [num_digits])", "Cuts off the fractional part of a number at the given digits."),
        ];
    fs
}
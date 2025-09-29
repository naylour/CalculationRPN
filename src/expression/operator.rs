use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum HighPriority {
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
pub enum LowPriority {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    High(HighPriority),
    Low(LowPriority),
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::High(_) => 2,
            Operator::Low(_) => 1,
        }
    }
}



pub fn char_to_operator(c: char) -> Option<Operator> {
    match c {
        '+' => Some(Operator::Low(LowPriority::Add)),
        '-' => Some(Operator::Low(LowPriority::Sub)),
        '*' => Some(Operator::High(HighPriority::Mul)),
        '/' => Some(Operator::High(HighPriority::Div)),
        _ => None,
    }
}

pub fn operator_to_char(operator: Operator) -> char {
    match operator {
        Operator::High(HighPriority::Mul) => '*',
        Operator::High(HighPriority::Div) => '/',
        Operator::Low(LowPriority::Add) => '+',
        Operator::Low(LowPriority::Sub) => '-',
    }
}

// -------------------- Реализация Display --------------------

impl fmt::Display for HighPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HighPriority::Mul => write!(f, "*"),
            HighPriority::Div => write!(f, "/"),
        }
    }
}

impl fmt::Display for LowPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LowPriority::Add => write!(f, "+"),
            LowPriority::Sub => write!(f, "-"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::High(h) => write!(f, "{}", h),
            Operator::Low(l) => write!(f, "{}", l),
        }
    }
}

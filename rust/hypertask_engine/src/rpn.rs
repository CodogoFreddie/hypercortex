use crate::engine::HyperTaskEngineContext;
use crate::error::*;
use crate::task::Task;
use chrono::prelude::*;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub enum RPNSymbol {
    Add,
    Divide,
    Duplicate,
    Equal,
    GetContext,
    GetProp,
    GetTag,
    GreaterThan,
    LessThan,
    Log,
    Multiply,
    Number(f64),
    Pow,
    Rem,
    Sqrt,
    Subtract,
    Symbol(String),
}

impl RPNSymbol {
    fn parse(s: &str) -> Self {
        use RPNSymbol::*;
        match s {
            "+" => Add,
            "/" => Divide,
            "&" => Duplicate,
            "=" => Equal,
            "@" => GetContext,
            ":" => GetProp,
            "#" => GetTag,
            ">" => GreaterThan,
            "<" => LessThan,
            "~" => Log,
            "*" => Multiply,
            "^" => Pow,
            "%" => Rem,
            "$" => Sqrt,
            "-" => Subtract,
            x => match x.parse::<f64>() {
                Ok(n) => Number(n),
                Err(s) => Symbol(x.to_string()),
            },
        }
    }
}

fn sanitize_date_time(dt: &Option<DateTime<Utc>>) -> f64 {
    dt.map(|date_time| date_time.timestamp()).unwrap_or(0) as f64
}

pub struct StackMachine<
    'a,
    InputIterator: Iterator<Item = HyperTaskResult<Task>>,
    Context: HyperTaskEngineContext<InputIterator>,
> {
    stack: Vec<RPNSymbol>,
    instructions: Rc<Vec<RPNSymbol>>,
    context: &'a Context,
    phantom: PhantomData<InputIterator>,
}

macro_rules! stack_machine_binary_method {
    ($name:ident, $a: ident, $b: ident, $op:expr, $msg:expr) => {
        fn $name(&mut self) -> HyperTaskResult<()> {
            let $a = self.pop_number().map_err( |e| { HyperTaskError::new( HyperTaskErrorDomain::ScoreCalculator, HyperTaskErrorAction::Run).msg($msg) }) ?;
            let $b = self.pop_number().map_err( |e| { HyperTaskError::new( HyperTaskErrorDomain::ScoreCalculator, HyperTaskErrorAction::Run).msg($msg) }) ?;
            self.push_number($op)
        }
    };
}

macro_rules! stack_machine_unary_method {
    ($name:ident, $a: ident, $op:expr, $msg:expr) => {
        fn $name(&mut self) -> HyperTaskResult<()> {
            let $a = self.pop_number().map_err( |e| { HyperTaskError::new( HyperTaskErrorDomain::ScoreCalculator, HyperTaskErrorAction::Run).msg($msg) }) ?;
            self.push_number($op)
        }
    };
}

impl<
        'a,
        InputIterator: Iterator<Item = HyperTaskResult<Task>>,
        Context: HyperTaskEngineContext<InputIterator>,
    > StackMachine<'a, InputIterator, Context>
{
    fn new(instructions: Vec<RPNSymbol>, context: &'a Context) -> Self {
        Self {
            stack: Vec::with_capacity((instructions.len() as f64).sqrt() as usize),
            instructions: Rc::new(instructions),
            context,
            phantom: PhantomData,
        }
    }

    fn pop(&mut self) -> HyperTaskResult<RPNSymbol> {
        self.stack.pop().ok_or(
            HyperTaskError::new(
                HyperTaskErrorDomain::ScoreCalculator,
                HyperTaskErrorAction::Run,
            )
            .msg("tried to pop an empty stack"),
        )
    }

    fn pop_number(&mut self) -> HyperTaskResult<f64> {
        let value = self.pop()?;

        if let RPNSymbol::Number(n) = value {
            Ok(n)
        } else {
            Err(HyperTaskError::new(
                HyperTaskErrorDomain::ScoreCalculator,
                HyperTaskErrorAction::Run,
            )
            .msg("popped value is not a number"))
        }
    }

    fn pop_symbol(&mut self) -> HyperTaskResult<String> {
        let value = self.pop()?;

        if let RPNSymbol::Symbol(s) = value {
            Ok(s)
        } else {
            Err(HyperTaskError::new(
                HyperTaskErrorDomain::ScoreCalculator,
                HyperTaskErrorAction::Run,
            )
            .msg("popped value is not a symbol"))
        }
    }

    fn push_number(&mut self, number: f64) -> HyperTaskResult<()> {
        self.stack.push(RPNSymbol::Number(number));
        Ok(())
    }

    stack_machine_unary_method!(run_sqrt, lhs, lhs.sqrt(), "could not sqrt");
    stack_machine_binary_method!(run_add, lhs, rhs, lhs + rhs, "could not add");
    stack_machine_binary_method!(run_divide, lhs, rhs, lhs / rhs, "could not divide");
    stack_machine_binary_method!(run_multiply, lhs, rhs, lhs * rhs, "could not multiply");
    stack_machine_binary_method!(run_subtract, lhs, rhs, lhs - rhs, "could not subtract");
    stack_machine_binary_method!(run_pow, lhs, rhs, lhs.powf(rhs), "could not pow");
    stack_machine_binary_method!(run_log, lhs, rhs, lhs.log(rhs), "could not get log");
    stack_machine_binary_method!(
        run_rem,
        lhs,
        rhs,
        lhs.rem_euclid(rhs),
        "could not get remainder"
    );
    stack_machine_binary_method!(
        run_equal,
        lhs,
        rhs,
        if lhs == rhs { 1.0 } else { 0.0 },
        "could not compare for equality"
    );
    stack_machine_binary_method!(
        run_greater_than,
        lhs,
        rhs,
        if lhs > rhs { 1.0 } else { 0.0 },
        "could not compare for greater than"
    );
    stack_machine_binary_method!(
        run_less_than,
        lhs,
        rhs,
        if lhs < rhs { 1.0 } else { 0.0 },
        "could not compare for less than"
    );

    fn run_duplicate(&mut self) -> HyperTaskResult<()> {
        let x = self.pop()?;
        self.stack.push(x.clone());
        self.stack.push(x.clone());
        Ok(())
    }

    fn run_get_prop(&mut self, task: &Task) -> HyperTaskResult<()> {
        let prop_name = self.pop_symbol()?;

        let replace = match prop_name.as_str() {
            "created_at" => task.get_created_at().timestamp() as f64,
            "done" => sanitize_date_time(task.get_done()),
            "due" => sanitize_date_time(task.get_due()),
            "snooze" => sanitize_date_time(task.get_snooze()),
            "updated_at" => task.get_updated_at().timestamp() as f64,
            "wait" => sanitize_date_time(task.get_wait()),

            _ => {
                return Err(HyperTaskError::new(
                    HyperTaskErrorDomain::ScoreCalculator,
                    HyperTaskErrorAction::Run,
                )
                .with_msg(|| format!("`{}` is not a valid prop name", &prop_name)));
            }
        };

        self.push_number(replace)
    }

    fn run_get_tag(&mut self, task: &Task) -> HyperTaskResult<()> {
        let tag_name = self.pop_symbol()?;

        let replace = if task.get_tags().contains(&tag_name) {
            1.0
        } else {
            0.0
        };

        self.push_number(replace)
    }

    fn run_get_context(&mut self) -> HyperTaskResult<()> {
        let context_name = self.pop_symbol()?;

        let replace = match context_name.as_str() {
            "now" => self.context.get_now().timestamp() as f64,

            _ => {
                return Err(HyperTaskError::new(
                    HyperTaskErrorDomain::ScoreCalculator,
                    HyperTaskErrorAction::Run,
                )
                .with_msg(|| format!("`{}` is not a valid context name", &context_name)));
            }
        };

        self.push_number(replace)
    }

    fn run_on(&mut self, task: &Task) -> HyperTaskResult<f64> {
        self.stack.clear();

        for instruction in &*(self.instructions.clone()) {
            match instruction {
                RPNSymbol::Add => self.run_add(),
                RPNSymbol::Divide => self.run_divide(),
                RPNSymbol::Duplicate => self.run_duplicate(),
                RPNSymbol::Equal => self.run_equal(),
                RPNSymbol::GetContext => self.run_get_context(),
                RPNSymbol::GetProp => self.run_get_prop(task),
                RPNSymbol::GetTag => self.run_get_tag(task),
                RPNSymbol::GreaterThan => self.run_greater_than(),
                RPNSymbol::LessThan => self.run_less_than(),
                RPNSymbol::Log => self.run_log(),
                RPNSymbol::Multiply => self.run_multiply(),
                RPNSymbol::Pow => self.run_pow(),
                RPNSymbol::Rem => self.run_rem(),
                RPNSymbol::Sqrt => self.run_sqrt(),
                RPNSymbol::Subtract => self.run_subtract(),

                RPNSymbol::Number(n) => Err(HyperTaskError::new(
                    HyperTaskErrorDomain::ScoreCalculator,
                    HyperTaskErrorAction::Run,
                )
                .msg("can't run a number as an operation")),
                RPNSymbol::Symbol(s) => Err(HyperTaskError::new(
                    HyperTaskErrorDomain::ScoreCalculator,
                    HyperTaskErrorAction::Run,
                )
                .msg("can't run a symbol as an operation")),
            }?
        }

        Ok(self.pop_number()?)
    }
}

use crate::error::*;
use crate::id::Id;
use crate::task::Task;
use chrono::prelude::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum RPNSymbol {
    Add,
    Branch,
    Count,
    Divide,
    Duplicate,
    Equal,
    GetEnvironment,
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
    Swap,
    Symbol(String),
}

impl RPNSymbol {
    pub fn parse_programs(ss: &[String]) -> Vec<Self> {
        let mut acc = vec![];

        for s in ss {
            for symbol in s.split_whitespace() {
                acc.push(Self::parse(symbol));
            }
        }

        acc
    }

    pub fn parse_program(s: &str) -> Vec<Self> {
        s.split_whitespace().map(Self::parse).collect::<Vec<Self>>()
    }

    pub fn stringify(self) -> String {
        use RPNSymbol::*;
        match self {
            Add => String::from("+"),
            Branch => String::from("?"),
            Count => String::from("|"),
            Divide => String::from("/"),
            Duplicate => String::from("&"),
            Equal => String::from("="),
            GetEnvironment => String::from("$"),
            GetProp => String::from(":"),
            GetTag => String::from("#"),
            GreaterThan => String::from(">"),
            LessThan => String::from("<"),
            Log => String::from("~"),
            Multiply => String::from("*"),
            Number(f) => format!("{}", f),
            Pow => String::from("^"),
            Rem => String::from("%"),
            Sqrt => String::from("_"),
            Subtract => String::from("-"),
            Swap => String::from("@"),
            Symbol(s) => s,
        }
    }

    pub fn parse(s: &str) -> Self {
        use RPNSymbol::*;
        match s {
            "#" => GetTag,
            "$" => GetEnvironment,
            "%" => Rem,
            "&" => Duplicate,
            "*" => Multiply,
            "+" => Add,
            "-" => Subtract,
            "/" => Divide,
            ":" => GetProp,
            "<" => LessThan,
            "=" => Equal,
            ">" => GreaterThan,
            "?" => Branch,
            "@" => Swap,
            "^" => Pow,
            "_" => Sqrt,
            "|" => Count,
            "~" => Log,
            x => match x.parse::<f64>() {
                Ok(n) => Number(n),
                Err(_) => Symbol(x.to_string()),
            },
        }
    }
}

fn sanitize_date_time(dt: &Option<DateTime<Utc>>) -> f64 {
    dt.map(|date_time| date_time.timestamp()).unwrap_or(0) as f64
}

macro_rules! stack_machine_binary_method {
    ($name:ident, $a: ident, $b: ident, $op:expr, $msg:expr) => {
        fn $name(&mut self) -> HyperTaskResult<()> {
            let $a = self.pop_number().map_err( |e| { HyperTaskError::new( HyperTaskErrorDomain::ScoreCalculator, HyperTaskErrorAction::Run).msg($msg).from(e) }) ?;
            let $b = self.pop_number().map_err( |e| { HyperTaskError::new( HyperTaskErrorDomain::ScoreCalculator, HyperTaskErrorAction::Run).msg($msg).from(e) }) ?;
            self.push_number($op)
        }
    };
}

macro_rules! stack_machine_unary_method {
    ($name:ident, $a: ident, $op:expr, $msg:expr) => {
        fn $name(&mut self) -> HyperTaskResult<()> {
            let $a = self.pop_number().map_err( |e| { HyperTaskError::new( HyperTaskErrorDomain::ScoreCalculator, HyperTaskErrorAction::Run).msg($msg).from(e) }) ?;
            self.push_number($op)
        }
    };
}

pub struct StackMachine {
    stack: Vec<RPNSymbol>,
    instructions: Rc<Vec<RPNSymbol>>,
    environment: HashMap<&'static str, f64>,
}

impl StackMachine {
    pub fn new(instructions: Vec<RPNSymbol>, environment: HashMap<&'static str, f64>) -> Self {
        Self {
            stack: Vec::with_capacity((instructions.len() as f64).sqrt() as usize),
            instructions: Rc::new(instructions),
            environment,
        }
    }

    fn pop(&mut self) -> HyperTaskResult<RPNSymbol> {
        self.stack.pop().ok_or_else(|| {
            HyperTaskError::new(
                HyperTaskErrorDomain::ScoreCalculator,
                HyperTaskErrorAction::Run,
            )
            .msg("tried to pop an empty stack")
        })
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
        if (lhs - rhs).abs() < std::f64::EPSILON {
            1.0
        } else {
            0.0
        },
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

    fn run_get_prop(
        &mut self,
        task: &Task,

        dependants_map: &HashMap<Rc<Id>, Vec<Rc<Task>>>,
    ) -> HyperTaskResult<()> {
        let prop_name = self.pop_symbol()?;

        let replace = match prop_name.as_str() {
            "created_at" => task.get_created_at().timestamp() as f64,
            "done" => sanitize_date_time(task.get_done()),
            "due" => sanitize_date_time(task.get_due()),
            "snooze" => sanitize_date_time(task.get_snooze()),
            "updated_at" => task.get_updated_at().timestamp() as f64,
            "wait" => sanitize_date_time(task.get_wait()),

            "blocked" => match task.get_blocked_by() {
                Some(_) => 1.0,
                None => 0.0,
            },

            "blocking" => match dependants_map.get(&task.get_id()) {
                Some(ds) => ds.len() as f64,
                None => 0.0,
            },

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

    fn run_get_environment(&mut self) -> HyperTaskResult<()> {
        let environment_name = self.pop_symbol()?;

        let replace = *self
            .environment
            .get(environment_name.as_str())
            .ok_or_else(|| {
                HyperTaskError::new(
                    HyperTaskErrorDomain::ScoreCalculator,
                    HyperTaskErrorAction::Run,
                )
                .with_msg(|| format!("`{}` is not a valid environment name", &environment_name))
            })?;

        self.push_number(replace)
    }

    fn run_swap(&mut self) -> HyperTaskResult<()> {
        let one = self.pop()?;
        let two = self.pop()?;
        self.stack.push(one);
        self.stack.push(two);
        Ok(())
    }

    fn run_count(&mut self) -> HyperTaskResult<()> {
        self.stack.push(RPNSymbol::Number(self.stack.len() as f64));
        Ok(())
    }

    fn run_branch(&mut self) -> HyperTaskResult<()> {
        let query = self.pop_number()?;
        let if_true = self.pop()?;
        let if_false = self.pop()?;

        let to_push = if query == 0.0 || query.is_infinite() || query.is_nan() {
            if_false
        } else {
            if_true
        };

        self.stack.push(to_push);

        Ok(())
    }

    fn run_one_step(
        &mut self,
        task: &Task,
        dependants_map: &HashMap<Rc<Id>, Vec<Rc<Task>>>,
        instruction: &RPNSymbol,
    ) -> HyperTaskResult<()> {
        match instruction {
            RPNSymbol::Add => self.run_add(),
            RPNSymbol::Branch => self.run_branch(),
            RPNSymbol::Count => self.run_count(),
            RPNSymbol::Divide => self.run_divide(),
            RPNSymbol::Duplicate => self.run_duplicate(),
            RPNSymbol::Equal => self.run_equal(),
            RPNSymbol::GetEnvironment => self.run_get_environment(),
            RPNSymbol::GetProp => self.run_get_prop(task, &dependants_map),
            RPNSymbol::GetTag => self.run_get_tag(task),
            RPNSymbol::GreaterThan => self.run_greater_than(),
            RPNSymbol::LessThan => self.run_less_than(),
            RPNSymbol::Log => self.run_log(),
            RPNSymbol::Multiply => self.run_multiply(),
            RPNSymbol::Pow => self.run_pow(),
            RPNSymbol::Rem => self.run_rem(),
            RPNSymbol::Sqrt => self.run_sqrt(),
            RPNSymbol::Subtract => self.run_subtract(),
            RPNSymbol::Swap => self.run_swap(),

            RPNSymbol::Number(n) => self.push_number(*n),
            RPNSymbol::Symbol(s) => {
                self.stack.push(RPNSymbol::Symbol(s.to_string()));
                Ok(())
            }
        }
    }

    pub fn run_on(
        &mut self,
        task: &Task,
        dependants_map: &HashMap<Rc<Id>, Vec<Rc<Task>>>,
    ) -> HyperTaskResult<f64> {
        self.stack.clear();

        for instruction in &*(self.instructions.clone()) {
            self.run_one_step(task, dependants_map, instruction)?;
        }

        Ok(self.pop_number()?)
    }

    pub fn run_with_snapshots(
        &mut self,
        task: &Task,
        dependants_map: &HashMap<Rc<Id>, Vec<Rc<Task>>>,
    ) -> HyperTaskResult<Vec<Vec<RPNSymbol>>> {
        self.stack.clear();
        let mut snapshots: Vec<Vec<RPNSymbol>> = vec![];

        for instruction in &*(self.instructions.clone()) {
            self.run_one_step(task, dependants_map, instruction)?;
            snapshots.push(self.stack.clone());
        }

        Ok(snapshots)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_date() -> DateTime<Utc> {
        Utc.ymd(2019, 11, 15).and_hms(9, 10, 11)
    }

    #[test]
    fn can_run_with_snapshots() {
        let instructions = RPNSymbol::parse_program("1 2 + test # due : now $");
        let mut env = HashMap::new();

        env.insert("now", 1234.0);

        let mut machine = StackMachine::new(instructions, env);

        let trace = machine
            .run_with_snapshots(&Task::generate(&get_test_date()), &HashMap::new())
            .unwrap();

        assert_eq!(
            trace,
            vec![
                RPNSymbol::parse_program("1"),
                RPNSymbol::parse_program("1 2"),
                RPNSymbol::parse_program("3"),
                RPNSymbol::parse_program("3 test"),
                RPNSymbol::parse_program("3 0"),
                RPNSymbol::parse_program("3 0 due"),
                RPNSymbol::parse_program("3 0 0"),
                RPNSymbol::parse_program("3 0 0 now"),
                RPNSymbol::parse_program("3 0 0 1234"),
            ]
        );
    }
}

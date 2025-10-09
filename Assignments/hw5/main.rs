#![allow(dead_code, unused_variables)] // REMOVE THIS BY THE END
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::io;
use std::fmt;

#[derive(Clone)]
enum Command {
    Push(i32),
    Drop,
    Dup,
    Dup2,
    Rot,
    Swap,
    Print,
    PrintStack,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lte,
    If,
    EndIf,
    Recurse(Option<Program>),
}

#[derive(Clone)]
struct Program(Vec<Command>);

impl Default for Program {
    fn default() -> Program {
        Program(Vec::default())
    }
}

impl Deref for Program {
    type Target = Vec<Command>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Program {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Program {
    fn new() -> Program {
        Program(Vec::new())
    }
}

impl From<Vec<Command>> for Program {
    fn from(v : Vec<Command>) -> Program {
        Program(v)
    }
}

type Dict = HashMap <String, Program>;

struct Stack(Vec<i32>);

impl Deref for Stack {
    type Target = Vec<i32>;

    fn deref(&self) -> &Vec<i32> {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Vec<i32> {
        &mut self.0
    }
}

impl Stack {
    fn new() -> Stack {
        Stack(Vec::new())
    }
}

impl Stack {
    fn eval_prog(&mut self, prog: &Program) -> Result<(), EvalError> {
        todo!()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.len())?;
        for i in self.iter() {
            write!(f, " {}", i)?;
        }
        Ok(())
    }
}

enum State {
    Compile,
    Interpret,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    State::Compile => write!(f, "compiling"),
	    State::Interpret => write!(f, "ok"),
	}
    }
}

struct Config {
    dict: Dict,
    stack: Stack,
    state: State,
    compiled: Program,
    compiled_word: String,
}

enum EvalError {
    StackUnderflow,
    DivByZero,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
	    EvalError::StackUnderflow => write!(f, "stack underflow"),
	    EvalError::DivByZero => write!(f, "division by zero"),
        }
    }
}

enum Error<'a> {
    Eval(EvalError),
    UnknownWord(&'a str),
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
	    Error::Eval(e) => write!(f, "{}", e),
            Error::UnknownWord(word) => write!(f, "unknown word: {}", &word),
        }
    }
}

impl<'a> From<EvalError> for Error<'a> {
    fn from (e : EvalError) -> Error<'a> {
        Error::Eval(e)
    }
}

impl Config {
    fn new() -> Config {
	Config {
	    state: State::Interpret,
	    compiled: Program::new(),
	    compiled_word: String::new(),
	    stack: Stack::new(),
	    dict: HashMap::from([
		(String::from("drop"), Program::from(vec![Command::Drop])),
		(String::from("2dup"), Program::from(vec![Command::Dup2])),
		(String::from("dup"), Program::from(vec![Command::Dup])),
		(String::from("rot"), Program::from(vec![Command::Rot])),
		(String::from("."), Program::from(vec![Command::Print])),
		(String::from(".s"), Program::from(vec![Command::PrintStack])),
		(String::from("swap"), Program::from(vec![Command::Swap])),
		(String::from("+"), Program::from(vec![Command::Add])),
		(String::from("-"), Program::from(vec![Command::Sub])),
		(String::from("*"), Program::from(vec![Command::Mul])),
		(String::from("/"), Program::from(vec![Command::Div])),
		(String::from("mod"), Program::from(vec![Command::Mod])),
		(String::from("<="), Program::from(vec![Command::Lte])),
	    ])
	}
    }

    fn eval_word<'a>(&mut self, word: &'a str) -> Result<(), Error<'a>> {
        todo!()
    }
}

fn main() {
    println!("Type `bye' to exit");
    let mut config = Config::new();
    let mut line = String::new();
    'interp: loop {
	let mut okay = true;
	line.clear();
	if let Ok(n) = io::stdin().read_line(&mut line) {
	    if n == 0 {
		print!("\x1b[1F");
		print!("{}", &config.stack);
		println!("\x1b[0K");
		break 'interp
	    }
	    line.pop();
	    print!("\x1b[1F{line}");
	    for word in line.split_whitespace().into_iter() {
		if word == "bye" {
		    println!("");
		    break 'interp;
		} else if let Err(err) = config.eval_word(word) {
		    print!(" \x1b[31m{err}\x1b[0m");
		    okay = false;
		    break
		}
	    }
	    if okay { print!(" \x1b[32m{}\x1b[0m", config.state) }
	    println!("\x1b[0K");
	} else {
	    panic!("Error reading line")
	}
    }
}

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Distribution {
    Constant(f64),
    Exponential(f64),
}
#[derive(Clone, Debug)]
pub struct Station {
    pub name: String,
    pub source: bool,
    pub capacity: usize,
    pub initial: usize,
    pub service: Distribution,
    pub destination: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct Model {
    pub stations: Vec<Station>,
    pub horizon: f64,
}
#[derive(Clone)]
struct Token {
    value: String,
    line: usize,
    column: usize,
}
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    fn error(&self, message: &str) -> String {
        let t = &self.tokens[self.pos.min(self.tokens.len() - 1)];
        format!(
            "Line {}, column {}: {} (found '{}')",
            t.line, t.column, message, t.value
        )
    }
    fn peek(&self) -> &str {
        &self.tokens[self.pos].value
    }
    fn take(&mut self) -> String {
        let s = self.peek().to_owned();
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        s
    }
    fn expect(&mut self, s: &str) -> Result<(), String> {
        if self.peek() != s {
            return Err(self.error(&format!(
                "Expected {s}; unsupported constructs are not ignored"
            )));
        }
        self.take();
        Ok(())
    }
    fn ident(&mut self) -> Result<String, String> {
        if !self
            .peek()
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphabetic())
        {
            return Err(self.error("Expected a station identifier"));
        }
        let s = self.take();
        if s == "OUT" {
            return Err(self.error("OUT is reserved for the network exit"));
        }
        Ok(s)
    }
    fn number(&mut self) -> Result<f64, String> {
        let x = self.peek().parse::<f64>().map_err(|_| {
            self.error(
                "Expected a numeric literal (variables and expressions are not supported yet)",
            )
        })?;
        if !x.is_finite() || x < 0.0 {
            return Err(self.error("Expected a finite nonnegative number"));
        }
        self.take();
        Ok(x)
    }
    fn integer(&mut self, max: usize) -> Result<usize, String> {
        let x = self.number()?;
        if x.fract() != 0.0 || x > max as f64 {
            return Err(self.error(&format!("Expected an integer between 0 and {max}")));
        }
        Ok(x as usize)
    }
    fn section(&mut self, s: &str) -> Result<(), String> {
        self.expect("/")?;
        self.expect(s)?;
        self.expect("/")
    }
}
fn lex(source: &str) -> Result<Vec<Token>, String> {
    if source.len() > 65536 {
        return Err("Source exceeds 64 KiB".into());
    }
    let mut tokens = Vec::new();
    let mut last = 1;
    for (l, line) in source.lines().enumerate() {
        last = l + 1;
        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'&' {
                break;
            }
            if b.is_ascii_whitespace() {
                i += 1;
                continue;
            }
            let start = i;
            if b.is_ascii_alphabetic() {
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
            } else if b.is_ascii_digit() || b == b'.' {
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
            } else if b"/=;,()".contains(&b) {
                i += 1;
            } else {
                return Err(format!(
                    "Line {}, column {}: unsupported character '{}'",
                    l + 1,
                    i + 1,
                    line[i..].chars().next().unwrap()
                ));
            }
            tokens.push(Token {
                value: line[start..i].to_ascii_uppercase(),
                line: l + 1,
                column: start + 1,
            });
        }
    }
    tokens.push(Token {
        value: "<EOF>".into(),
        line: last,
        column: 1,
    });
    Ok(tokens)
}
pub fn parse(source: &str) -> Result<Model, String> {
    let mut p = Parser {
        tokens: lex(source)?,
        pos: 0,
    };
    p.section("DECLARE")?;
    p.expect("QUEUE")?;
    let mut names = Vec::new();
    loop {
        let name = p.ident()?;
        if names.contains(&name) {
            return Err(p.error("Duplicate QUEUE declaration"));
        }
        names.push(name);
        if names.len() > 128 {
            return Err(p.error("At most 128 stations are supported"));
        }
        if p.peek() != "," {
            break;
        }
        p.take();
    }
    p.expect(";")?;
    let mut stations = vec![None; names.len()];
    loop {
        p.expect("/")?;
        let section = p.take();
        p.expect("/")?;
        if section == "CONTROL" {
            break;
        }
        if section != "STATION" {
            return Err(p.error("Expected /STATION/ or /CONTROL/"));
        }
        p.expect("NAME")?;
        p.expect("=")?;
        let name = p.ident()?;
        p.expect(";")?;
        let index = names
            .iter()
            .position(|n| n == &name)
            .ok_or_else(|| p.error("Station was not declared with QUEUE"))?;
        if stations[index].is_some() {
            return Err(p.error("Duplicate station definition"));
        }
        let (mut source, mut capacity, mut initial, mut service, mut destination) =
            (false, 1, 0, None, None);
        let mut seen = HashSet::new();
        while p.peek() != "/" && p.peek() != "<EOF>" {
            let key = p.take();
            if !seen.insert(key.clone()) {
                return Err(p.error("Duplicate station property"));
            }
            p.expect("=")?;
            match key.as_str() {
                "TYPE" => {
                    match p.take().as_str() {
                        "SOURCE" => source = true,
                        "SERVER" => {}
                        _ => return Err(p.error("Only SOURCE and SERVER are supported")),
                    }
                    if p.peek() == "," {
                        p.take();
                        if source {
                            return Err(p.error("SOURCE cannot have a server capacity"));
                        }
                        match p.take().as_str() {
                            "SINGLE" => {}
                            "MULTIPLE" => {
                                p.expect("(")?;
                                capacity = p.integer(1024)?;
                                p.expect(")")?;
                                if capacity == 0 {
                                    return Err(p.error("Capacity must be positive"));
                                }
                            }
                            _ => return Err(p.error("Only SINGLE and MULTIPLE(n) are supported")),
                        }
                    }
                }
                "SERVICE" => {
                    let dist = p.take();
                    p.expect("(")?;
                    let value = p.number()?;
                    p.expect(")")?;
                    if value <= 0.0 || value > 1e9 {
                        return Err(p.error("Service duration must be in (0, 1e9]"));
                    }
                    service = Some(match dist.as_str() {
                        "CST" => Distribution::Constant(value),
                        "EXP" => Distribution::Exponential(value),
                        _ => return Err(p.error("Only CST(duration) and EXP(mean) are supported")),
                    });
                }
                "TRANSIT" => {
                    let dest = p.take();
                    destination = Some(if dest == "OUT" {
                        None
                    } else {
                        Some(
                            names
                                .iter()
                                .position(|n| n == &dest)
                                .ok_or_else(|| p.error("Unknown TRANSIT destination"))?,
                        )
                    });
                }
                "INIT" => initial = p.integer(100000)?,
                _ => return Err(p.error(&format!("Unsupported station property {key}"))),
            }
            p.expect(";")?;
        }
        if source && initial != 0 {
            return Err(p.error("INIT on a SOURCE is not supported"));
        }
        stations[index] = Some(Station {
            name,
            source,
            capacity,
            initial,
            service: service.ok_or_else(|| p.error("SERVICE is required"))?,
            destination: destination.ok_or_else(|| p.error("TRANSIT is required"))?,
        });
    }
    p.expect("TMAX")?;
    p.expect("=")?;
    let horizon = p.number()?;
    p.expect(";")?;
    if horizon <= 0.0 || horizon > 1e9 {
        return Err(p.error("TMAX must be in (0, 1e9]"));
    }
    p.section("EXEC")?;
    p.expect("BEGIN")?;
    p.expect("SIMUL")?;
    p.expect(";")?;
    p.expect("END")?;
    p.expect(";")?;
    p.section("END")?;
    p.expect("<EOF>")?;
    let stations: Vec<Station> = stations
        .into_iter()
        .enumerate()
        .map(|(i, s)| s.ok_or_else(|| format!("Missing /STATION/ for {}", names[i])))
        .collect::<Result<_, _>>()?;
    if stations.iter().map(|s| s.initial).sum::<usize>() > 100000 {
        return Err("Initial population exceeds 100000".into());
    }
    for s in &stations {
        if s.destination.is_some_and(|i| stations[i].source) {
            return Err(format!("{}: routing to a SOURCE is unsupported", s.name));
        }
    }
    if !stations.iter().any(|s| s.source || s.initial > 0) {
        return Err("No customers: add a SOURCE or a positive INIT".into());
    }
    Ok(Model { stations, horizon })
}

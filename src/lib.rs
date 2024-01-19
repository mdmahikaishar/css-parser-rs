
/// CSS Lexer Event
#[derive(Debug)]
pub enum Event {
    /// When a selector starts.
    StartSelector(Vec<String>),
    /// When a selector ends.
    EndSelector(Vec<String>),
    /// When encouter a rule.
    Rule(String, String),
    /// When a comment found.
    Comment(String),
}

/// CSS Lexer
pub struct Lexer {
    content: Vec<char>,
    data: Vec<Event>,
    selector_stack: Vec<Vec<String>>,
}

impl Lexer {
    /// New CSS Lexer instance.
    /// 
    /// `contents` is read css file.
    pub fn new(contents: &str) -> Self {
        Self {
            content: contents.chars().collect::<Vec<char>>(),
            data: Vec::new(),
            selector_stack: Vec::new(),
        }
    }

    /// Parse the contents.
    pub fn parse(&mut self) -> &[Event] {
        while !self.content.is_empty() {
            self.trim_whitespaces();

            // START SELECTOR
            // eg, `*` || `html` || `.class` || `#id`
            if !self.content.is_empty()
                && (self.content[0] == '*'
                    || self.content[0] == '.'
                    || self.content[0] == '#'
                    || self.content[0].is_alphabetic())
            {
                self.parse_start_selector();
            }

            self.trim_whitespaces();

            // RULES
            // eg, `justify-content: center;`
            if !self.content.is_empty() && self.content[0] == '{' {
                self.parse_rules()
            }

            self.trim_whitespaces();

            // END SELECTOR
            // eg, `}`
            if !self.content.is_empty() && self.content[0] == '}' {
                self.parse_end_selector();
            }

            self.trim_whitespaces();

            // COMMENT
            // eg, `/* comment */`
            if self.content.len() > 1 && self.content[0] == '/' && self.content[1] == '*' {
                self.parse_comment();
            }
        }

        &self.data
    }

    /// START SELECTOR
    ///
    /// its also implement the multiple selector login.
    ///
    /// eg, `*` || `html` || `.class` || `#id`
    fn parse_start_selector(&mut self) {
        let mut selectors = Vec::new();

        // multiple selector logic
        while !self.content.is_empty() && self.content[0] != '{' {
            let selector = self.take_while(|x| x != ' ' && x != ',' && x != '{');

            self.trim_whitespaces();

            if !self.content.is_empty() && self.content[0] == ',' {
                self.take_slice(0, 1); // remove `,`
                self.trim_whitespaces();
            }

            selectors.push(selector);
        }

        // remove `{` will occured in `fn parse_rules()`
        // coz rules are declered inside of `{`

        self.data.push(Event::StartSelector(selectors.clone()));
        self.selector_stack.push(selectors);
    }

    /// RULES
    ///
    /// TODO: Implement multiple properties.
    ///
    /// eg, `justify-content: center;`
    fn parse_rules(&mut self) {
        self.take_slice(0, 1); // remove `{`

        self.trim_whitespaces();

        while self.content[0] != '}' {
            let rule_name = self.take_while(|x| x.is_alphabetic() || x == '-');
            self.trim_whitespaces();

            if self.content[0] != ':' {
                eprintln!("ERROR: expecting `:`");
                break;
            }
            self.take_slice(0, 1); // remove `:`
            self.trim_whitespaces();

            let rule_value = self.take_while(|x| x != ';');
            self.take_slice(0, 1); // remove `;`

            self.data.push(Event::Rule(rule_name, rule_value));

            self.trim_whitespaces();
        }
    }

    /// END SELECTOR
    /// eg, `}`
    fn parse_end_selector(&mut self) {
        self.take_slice(0, 1); // remove `}`

        if let Some(selector) = self.selector_stack.pop() {
            self.data.push(Event::EndSelector(selector));
        } else {
            eprintln!("ERROR: invalid clsoing selector. Need a staring selector.");
        }
    }

    /// COMMENT
    ///
    /// TODO: Implement multiline comment.
    ///
    /// eg, `/* comment */`
    fn parse_comment(&mut self) {
        self.take_slice(0, 2); // remove `/*`

        let mut astric_count = 0;

        let comment = self.take_while(|x| {
            if x == '*' {
                astric_count += 1;
            }
            // end comment
            else if x == '/' && astric_count == 1 {
                return false;
            }

            true
        });

        self.take_slice(0, 2); // remove `*/`

        // `*` get included in comment
        // coz, we take `*` as state, THEN we are loking for `/` to stop
        let comment = comment[..comment.len() - 1].to_string();

        self.data.push(Event::Comment(comment));
    }

    fn trim_whitespaces(&mut self) -> () {
        self.take_while(|x| x.is_whitespace());
    }

    fn take_while<F>(&mut self, predict: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        self.take_while_from(0, predict)
    }

    fn take_while_from<F>(&mut self, start: usize, mut predict: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut i = start;

        while self.content.len() > i && predict(self.content[i]) {
            i += 1;
        }

        self.take_slice(start, i)
    }

    fn take_slice(&mut self, from: usize, to: usize) -> String {
        let slice = self.content[from..to].iter().collect::<String>();
        self.content = self.content[to..].to_vec();

        slice
    }
}

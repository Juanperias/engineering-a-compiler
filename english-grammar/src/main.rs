// Sentence -> Subject verb Object endmark
// Subject -> noun
// Subject -> Modifier noun
// Object -> noun
// Object -> Modifier noun
// Modifier -> adjective

#[derive(Debug, Clone)]
pub struct Token(String);

#[derive(Clone, Debug)]
pub enum Syntax {
    And(Vec<Syntax>),
    Or(Vec<Syntax>),
    Expand(Box<Syntax>),
    Just(Token),
}

#[derive(Clone, Debug)]
pub struct SyntacticVar(String, Syntax);

fn main() {
    let token1 = Token("20".to_string());
    let noun = SyntacticVar(
        "Noun".to_string(),
        Syntax::Or(vec![
            Syntax::Just(Token("soccer".to_string())),
            Syntax::Just(Token("I".to_string())),
            Syntax::Just(Token("Russia".to_string())),
        ]),
    );

    let adjective = Syntax::Or(vec![
        Syntax::Just(Token("red".to_string())),
        Syntax::Just(Token("big".to_string())),
    ]);

    let verb = SyntacticVar(
        "Verb".to_string(),
        Syntax::Or(vec![
            Syntax::Just(Token("is".to_string())),
            Syntax::Just(Token("play".to_string())),
        ]),
    );

    let endmark = SyntacticVar("Endmark".to_string(), Syntax::Just(Token(".".to_string())));

    let modifier = SyntacticVar("Modifier".to_string(), Syntax::Expand(Box::new(adjective)));

    let subject = SyntacticVar(
        "Subject".to_string(),
        Syntax::Or(vec![
            Syntax::Expand(Box::new(noun.1.clone())),
            Syntax::And(vec![
                Syntax::Expand(Box::new(modifier.1.clone())),
                Syntax::Expand(Box::new(noun.clone().1)),
            ]),
        ]),
    );

    let object = SyntacticVar(
        "Object".to_string(),
        Syntax::Or(vec![
            Syntax::Expand(Box::new(modifier.clone().1)),
            Syntax::Expand(Box::new(subject.clone().1)),
        ]),
    );

    // Sentence -> Subject verb Object
    let sentence = SyntacticVar(
        "Sentence".to_string(),
        Syntax::And(vec![
            Syntax::Expand(Box::new(subject.1.clone())),
            Syntax::Expand(Box::new(verb.1.clone())),
            Syntax::Expand(Box::new(object.1)),
        ]),
    );

    println!("{:?}", parse("Russia is Russia", &sentence));
    println!("{:?}", parse("Russia is big", &sentence));
    println!("{:?}", parse("I play soccer", &sentence));

    println!("{:?}", token1);
}

fn parse(raw: &str, var: &SyntacticVar) -> (String, Vec<Token>) {
    let mut v = Vec::new();

    let splited: Vec<&str> = raw.split(" ").collect();
    v.extend(rule_parser(&splited, &var.1).unwrap());
    // }

    (var.0.clone(), v)
}

// TODO: convert to Option
fn rule_parser<'a>(tokens: &'a [&'a str], rule: &Syntax) -> Option<Vec<Token>> {
    match rule {
        Syntax::Just(t) => {
            if !tokens.is_empty() && tokens[0] == t.0 {
                Some(vec![t.clone()])
            } else {
                None
            }
        }
        Syntax::Or(choices) => choices.iter().find_map(|s| rule_parser(tokens, s)),
        Syntax::And(seq) => {
            let mut matched = Vec::new();
            let mut remaining = tokens;
            for s in seq {
                if let Some(res) = rule_parser(remaining, s) {
                    matched.extend_from_slice(&res);
                    remaining = &remaining[res.len()..];
                } else {
                    return None;
                }
            }
            Some(matched)
        }
        Syntax::Expand(v) => rule_parser(tokens, &v),
        _ => None,
    }
}

use crate::lexer::Lexer;
use crate::parser::parser::*;
use crate::environment::environment::Environment;
use crate::environment::heap::Heap;
use std::rc::Rc;
use std::cell::RefCell;
    use colored::Colorize;
use reedline::{
    default_emacs_keybindings, Emacs, KeyCode, KeyModifiers, Reedline, Signal, DefaultPrompt, ReedlineEvent, Highlighter, StyledText,
};
use nu_ansi_term::{Color, Style};

struct LoopHighlighter;

impl Highlighter for LoopHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut styled_parts: Vec<(Style, String)> = Vec::new();
        
        let mut current_pos = 0;
        while current_pos < line.len() {
            let mut temp_lexer = Lexer::new(line[current_pos..].to_string());
            if let Ok((token, consumed)) = temp_lexer.tokenize_next_with_index() {
                if consumed == 0 {
                    styled_parts.push((Style::default(), line[current_pos..].to_string()));
                    break;
                }

                let start = current_pos;
                let end = current_pos + consumed;
                let text = line[start..end].to_string();

                let style = match token {
                    crate::lexer::token::Token::Literal(_) => Style::new().fg(Color::Yellow),
                    crate::lexer::token::Token::Identifier(_) => Style::new().fg(Color::White),
                    crate::lexer::token::Token::Type(_) => Style::new().fg(Color::Green),
                    crate::lexer::token::Token::Conditional(_)
                    | crate::lexer::token::Token::Loop(_)
                    | crate::lexer::token::Token::Function(_)
                    | crate::lexer::token::Token::VariableDeclaration(_)
                    | crate::lexer::token::Token::Module(_)
                    | crate::lexer::token::Token::TypeDeclaration(_) => Style::new().fg(Color::Magenta),
                    crate::lexer::token::Token::Operator(_) => Style::new().fg(Color::Cyan),
                    crate::lexer::token::Token::Comment(_) => Style::new().fg(Color::DarkGray),
                    _ => Style::default(),
                };
                styled_parts.push((style, text));
                current_pos = end;
            } else {
                styled_parts.push((Style::default(), line[current_pos..].to_string()));
                break;
            }
        }

        let mut styled_text = StyledText::new();
        styled_parts.iter().for_each(|elem| {
            styled_text.push(elem.clone());
        });
        styled_text
    }
}

pub fn repl(print_ast: bool, print_tokens: bool) {
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::SHIFT,
        KeyCode::Enter,
        ReedlineEvent::Submit,
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut line_editor = Reedline::create()
        .with_highlighter(Box::new(LoopHighlighter))
        .with_edit_mode(edit_mode);

    let prompt = DefaultPrompt::default();
    let heap = Heap::new();
    let heap_rc = Rc::new(RefCell::new(heap));
    let mut env = Environment::new(None, Some(heap_rc.clone()));

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                if buffer.trim().is_empty() {
                    continue;
                }
                
                // Check if user typed 'exit'
                if buffer.trim() == "exit" {
                    println!("Exiting...");
                    break;
                } else if buffer.trim().starts_with("#type") {
                    let identifier = buffer.trim().split_whitespace().nth(1).unwrap();
                    let type_ = env.get_type(identifier);
                    match type_ {
                        Ok(type_) => println!("{:?}", type_),
                        Err(e) => eprintln!("{} {:?}", "Error:".red(), e),
                    }
                    continue;
                } else if buffer.trim().starts_with("#heap") {
                    heap_rc.borrow().print();
                    continue;
                }
                
                let mut lexer = Lexer::new(buffer.clone());
                match lexer.tokenize() {
                    Ok(_) => {
                        lexer.clean_tokens();
                        if print_tokens {
                            println!("Tokens: {:?}", lexer.tokens);
                        }
                        let mut parser = Parser::new(lexer.tokens.clone());
                        match parser.parse() {
                            Ok(ast) => {
                                if print_ast {
                                    println!("{}", ast.to_string());
                                }
                                match ast.eval(&mut env) {
                                    Ok(value) => println!("{}", value.to_string().green()),
                                    Err(e) => eprintln!("{} {:?}", "Error:".red(), e),
                                }
                            }
                            Err(e) => {
                                eprintln!("{} {:?}", "Error:".red(), e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {:?}", "Error:".red(), e);
                    }
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                eprintln!("{} {:?}", "Error:".red(), err);
                break;
            }
        }
    }
}

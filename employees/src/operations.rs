pub enum Operation {
    Add(String, String),
    Remove(String, String),
    Transfer(String, String, String),
    ListAll(String),
    End,
}

pub fn parse_op(op: &str) -> Option<Operation> {
    let words: Vec<&str> = op
        .clone()
        .trim()
        .split(' ')
        .filter(|x| x.len() > 0)
        .collect();

    if words.len() > 0 {
        let op_type = words
            .get(0)
            .expect("one of the operations of add, remove or transfer");
        let args = &words[1..];

        match op_type.to_ascii_lowercase().as_str() {
            "add" => parse_add_op(args),
            "remove" => parse_remove_op(args),
            "transfer" => parse_transfer_op(args),
            "list" => parse_list_op(args),
            "end" => Some(Operation::End),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_add_op(args: &[&str]) -> Option<Operation> {
    if args.len() == 3 {
        if args[1].to_ascii_lowercase().as_str() == "to" {
            Some(Operation::Add(args[0].to_string(), args[2].to_string()))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_remove_op(args: &[&str]) -> Option<Operation> {
    if args.len() == 3 {
        if args[1].to_ascii_lowercase().as_str() == "to" {
            Some(Operation::Remove(args[0].to_string(), args[2].to_string()))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_transfer_op(args: &[&str]) -> Option<Operation> {
    if args.len() == 5 {
        if args[1].to_ascii_lowercase().as_str() == "from"
            && args[3].to_ascii_lowercase().as_str() == "to"
        {
            Some(Operation::Transfer(
                args[0].to_string(),
                args[2].to_string(),
                args[4].to_string(),
            ))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_list_op(args: &[&str]) -> Option<Operation> {
    if args.len() == 1 {
        Some(Operation::ListAll(args[0].to_string()))
    } else {
        None
    }
}

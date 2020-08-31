use serenity::model::channel::Message;

pub fn get_arg_string(msg: &Message, cmd_name: &str) -> Option<String> {
    let cmd_len = cmd_name.len() + 2;

    if &msg.content[0..2] == "$ " && msg.content.len() > cmd_len {
        Some(msg.content[(cmd_len + 1)..].to_string())
    } else if msg.content.len() > cmd_len {
        Some(msg.content[cmd_len..].to_string())
    } else {
        None
    }
}

use std::process::Command;
use std::str;
use std::fs;
use std::time::Duration;
use enigo::{Enigo, Key};
use enigo::KeyboardControllable;
use regex::Regex;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use tray_item::TrayItem;
use home::home_dir;

fn main() {
    let mut tray = TrayItem::new("messauto", "icon").unwrap();
    tray.add_menu_item("启动", || {
        std::thread::spawn(move || {
            let auto_input = true;
            let flag = "验证码";
            let chat_db_path = home_dir().expect("获取用户目录失败").join("Library/Messages/chat.db-wal");
            let mut last_metadata_modified = fs::metadata(&chat_db_path).expect("获取元数据失败").modified().unwrap();
            loop{
                let now_metadata = fs::metadata(&chat_db_path).expect("获取元数据失败").modified().unwrap();
                if now_metadata != last_metadata_modified{
                    last_metadata_modified = now_metadata;
                    let stdout = get_message_in_one_minute();
                    if check_captcha_or_other(&stdout, flag){
                        let captcha = get_captcha(&stdout);
                        println!("{}", captcha);
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(captcha.to_owned()).unwrap();
                        println!("{:?}", ctx.get_contents());
                        if auto_input{input_and_enter();}
                    }
                }
                std::thread::sleep(Duration::new(1, 0));
            }
        });
    }).unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("退出");
    inner.display();
}

fn check_captcha_or_other(stdout:&String, flag:&str) -> bool{
    if stdout.contains(flag){
        return true;
    }else {
        return false;
    }
}

fn get_captcha(stdout:&String) -> String{
    let re = Regex::new(r"\d{4,6}").unwrap();
    let stdout_str = stdout.as_str();
    let captcha = re.find(stdout_str).map(|m| m.as_str()).unwrap_or("").to_string();
    return captcha;
}

fn get_message_in_one_minute() -> String{
    let output = Command::new("sqlite3")
                                .arg("/Users/ls/Library/Messages/chat.db")
                                .arg("SELECT text FROM message WHERE datetime(date/1000000000 + 978307200,\"unixepoch\",\"localtime\") > datetime(\"now\",\"localtime\",\"-60 second\") ORDER BY date DESC LIMIT 1;")
                                .output()
                                .expect("sqlite命令运行失败");
    let stdout = String::from_utf8(output.stdout).unwrap();
    // let stderr = String::from_utf8(output.stderr).unwrap();
    // println!("stdout: {:?}", stdout);
    // println!("stderr: {:?}", stderr);
    return stdout;
}

fn input_and_enter() {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Raw(0x09));
    enigo.key_up(Key::Meta);
    enigo.key_click(Key::Return);
}

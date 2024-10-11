use std::process::Command;

pub fn espeak(text: String) {
    // 定义要说出的文本
    // let text = "Hello, World!";

    // 调用 eSpeak
    let output = Command::new("espeak-ng")
        .arg(&text)
        .output()
        .expect("Failed to execute espeak");

    // 如果需要查看命令的输出（通常不需要，因为是语音输出）
    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

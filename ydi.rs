use std::process::{Command, Stdio};
use std::io::{self, Write, BufRead, BufReader};

fn main() {
    // 실행 파일 이름 정의
    let yt_binary = "./yt-dlp_linux";

    println!("--- yt-dlp_linux Video & Subtitle TUI (2026) ---");
    
    print!("URL을 입력하세요: ");
    io::stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();
    if url.is_empty() { return; }

    // 1. 포맷 리스트 출력 (480p 이상 동영상 및 128kbps 이상 오디오만 표시, mhtml 제외)
    println!("\n[1/3] 이용 가능한 포맷 리스트 (480p+ & 128k+, mhtml 제외):");

    // -f 옵션 뒤에 조건을 넣고 -F를 붙이면 해당 조건에 맞는 리스트만 출력됩니다.
    let _ = Command::new(yt_binary)
        .args(&[
            "--no-playlist", 
            // 1. 비디오: 세로 해상도(height) 480 이상
            // 2. 오디오: 비트레이트(abr) 128 이상
            // 3. 공통: mhtml 확장자 제외 [ext!=mhtml]
            "-f", "vcodec!=none[height>=480][ext!=mhtml] / vcodec=none[abr>=128][ext!=mhtml]", 
            "-F", 
            url
        ])
        .status();

/*
    // 2. 자막 리스트 출력
    println!("\n[2/3] 이용 가능한 자막 리스트:");
    let _ = Command::new(yt_binary)
        .args(&["--no-playlist", "--list-subs", url])
        .status();
*/
    println!("\n[2/3] 이용 가능한 자막 리스트 조회 생략 --no-playlist --list-subs");
    
    // 3. 사용자 선택
    println!("\n[3/3] 선택 단계");
    print!("포맷 코드 입력 (예: 137+140 avc+m4a or 248+251 vp9+opus): ");
    io::stdout().flush().unwrap();
    let mut f_code = String::new();
    io::stdin().read_line(&mut f_code).unwrap();

    print!("자막 언어 코드 (예: ko, en / ja, zh-Hans, zh-Hant / 필요 없으면 엔터): ");
    io::stdout().flush().unwrap();
    let mut s_code = String::new();
    io::stdin().read_line(&mut s_code).unwrap();

    // 4. 다운로드 실행
    println!("\n다운로드를 시작합니다...");
    let mut cmd = Command::new(yt_binary);
    
    // 기본 설정: 플레이리스트 방지 및 출력 파일명 형식 지정
    cmd.args(&[
        "--no-playlist",
        "-o", "%(title)s.%(ext)s", // 파일명 형식 고정
        "-f", f_code.trim()
    ]);

    // 자막 옵션 처리
    let sub_lang = s_code.trim();
    if !sub_lang.is_empty() {
        cmd.args(&[
            "--write-sub", 
            "--sub-lang", sub_lang, 
    //        "--embed-subs",
            "--convert-subs", "srt" // 자막 호환성을 위해 srt로 변환 (선택사항)
        ]);
    }

    let mut child = cmd.arg(url)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("yt-dlp_linux 실행 실패. 파일이 해당 경로에 있는지 확인하세요.");

    // 실시간 출력 피드백
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                if l.contains("%") {
                    print!("\r{}", l);
                    io::stdout().flush().unwrap();
                } else {
                    println!("{}", l);
                }
            }
        }
    }

    let _ = child.wait();
    println!("\n\n다운로드가 완료되었습니다: %(title)s.%(ext)s 형식으로 저장됨");
}

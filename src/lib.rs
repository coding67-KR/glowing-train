use rand::prelude::*;
use slint::Weak;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

slint::slint! {
    import { Button, LineEdit, VerticalBox } from "std-widgets.slint";

    component MainWindow inherits Window {
        width: 420px;
        height: 820px;
        title: "끝말잇기";

        property <string> required_char: "시작";
        property <string> cpu_word: "";
        property <string> status_text: "시작 버튼을 눌러 게임을 시작하세요.";
        property <string> score_text: "점수 0";
        property <string> streak_text: "콤보 0";
        property <string> used_count_text: "사용 0개";
        property <bool> playing: false;
        property <bool> can_submit: false;

        callback start_game();
        callback submit_word(string);
        callback new_round();

        Rectangle {
            background: #10172a;

            VerticalBox {
                padding: 22px;
                spacing: 14px;

                Text {
                    text: "끝말잇기";
                    font-size: 32px;
                    font-weight: 700;
                    color: white;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Rust Edition";
                    font-size: 14px;
                    color: #9fb0d0;
                    horizontal-alignment: center;
                }

                Rectangle {
                    height: 130px;
                    border-radius: 18px;
                    background: #19233d;
                    border-color: #2b3a60;
                    border-width: 1px;

                    VerticalBox {
                        padding: 18px;
                        spacing: 7px;

                        Text {
                            text: playing ? "다음 글자" : "준비";
                            color: #9fb0d0;
                            font-size: 15px;
                            horizontal-alignment: center;
                        }
                        Text {
                            text: required_char;
                            color: #ffffff;
                            font-size: 48px;
                            font-weight: 800;
                            horizontal-alignment: center;
                        }
                    }
                }

                HorizontalLayout {
                    spacing: 10px;
                    Text { text: score_text; color: #d9e3ff; font-size: 15px; }
                    Text { text: streak_text; color: #d9e3ff; font-size: 15px; }
                    Text { text: used_count_text; color: #d9e3ff; font-size: 15px; }
                }

                Rectangle {
                    height: 112px;
                    border-radius: 18px;
                    background: #171f35;
                    border-color: #273554;
                    border-width: 1px;

                    VerticalBox {
                        padding: 16px;
                        spacing: 8px;
                        Text {
                            text: "컴퓨터";
                            color: #8ea4cf;
                            font-size: 13px;
                        }
                        Text {
                            text: cpu_word == "" ? "—" : cpu_word;
                            color: white;
                            font-size: 24px;
                            font-weight: 700;
                        }
                    }
                }

                LineEdit {
                    input := LineEdit {
                        enabled: playing;
                        placeholder-text: playing ? (required_char + "로 시작하는 단어") : "게임을 먼저 시작하세요";
                        font-size: 20px;
                        padding: 14px;
                        accepted(text) => {
                            if (playing) { submit_word(text); input.text = ""; }
                        }
                    }
                }

                Button {
                    text: playing ? "단어 제출" : "게임 시작";
                    enabled: playing ? can_submit : true;
                    clicked => {
                        if (playing) { submit_word(input.text); input.text = ""; }
                        else { start_game(); }
                    }
                }

                Button {
                    text: "새 게임";
                    enabled: playing;
                    clicked => { new_round(); }
                }

                Rectangle {
                    height: 150px;
                    border-radius: 18px;
                    background: #171f35;
                    border-color: #273554;
                    border-width: 1px;

                    VerticalBox {
                        padding: 16px;
                        spacing: 7px;
                        Text {
                            text: "상태";
                            color: #8ea4cf;
                            font-size: 13px;
                        }
                        Text {
                            text: status_text;
                            color: #eef4ff;
                            font-size: 16px;
                            wrap: word-wrap;
                        }
                        Text {
                            text: "프로젝트 루트의 txt.txt를 단어 목록으로 사용합니다.";
                            color: #6f83aa;
                            font-size: 12px;
                        }
                    }
                }

                Rectangle { background: transparent; height: 1px; }
                Text {
                    text: "Enter로 제출 · 중복 단어는 사용 불가";
                    color: #6f83aa;
                    font-size: 12px;
                    horizontal-alignment: center;
                }
            }
        }
    }
}

fn normalize_word(raw: &str) -> Option<String> {
    let w = raw.trim().to_lowercase();
    if w.is_empty() || w.starts_with('#') {
        return None;
    }
    if w.contains(char::is_whitespace) {
        return None;
    }
    Some(w)
}

fn first_scalar(s: &str) -> Option<char> {
    s.chars().next()
}

fn last_scalar(s: &str) -> Option<char> {
    s.chars().next_back()
}

fn load_words() -> Vec<String> {
    include_str!("../txt.txt")
        .lines()
        .filter_map(normalize_word)
        .collect()
}

struct GameState {
    words: Vec<String>,
    used: HashSet<String>,
    required: Option<char>,
    score: i32,
    streak: i32,
    cpu_word: String,
}

impl GameState {
    fn new() -> Self {
        Self {
            words: load_words(),
            used: HashSet::new(),
            required: None,
            score: 0,
            streak: 0,
            cpu_word: String::new(),
        }
    }

    fn reset(&mut self) {
        self.used.clear();
        self.required = None;
        self.score = 0;
        self.streak = 0;
        self.cpu_word.clear();
    }

    fn cpu_pick(&self, start: char) -> Option<String> {
        let candidates: Vec<&String> = self
            .words
            .iter()
            .filter(|w| !self.used.contains(*w))
            .filter(|w| first_scalar(w) == Some(start))
            .collect();
        let mut rng = rand::rng();
        candidates.choose(&mut rng).map(|word| (*word).clone())
    }

    fn play_cpu(&mut self, start: char) -> Result<String, &'static str> {
        let Some(word) = self.cpu_pick(start) else {
            return Err("CPU가 이어갈 단어를 찾지 못했습니다.");
        };
        self.used.insert(word.clone());
        self.required = last_scalar(&word);
        self.cpu_word = word.clone();
        Ok(word)
    }
}

fn set_ui(ui: &MainWindow, game: &GameState, status: &str, playing: bool) {
    ui.set_playing(playing);
    ui.set_can_submit(playing && game.required.is_some());
    ui.set_score_text(format!("점수 {}", game.score).into());
    ui.set_streak_text(format!("콤보 {}", game.streak).into());
    ui.set_used_count_text(format!("사용 {}개", game.used.len()).into());
    ui.set_status_text(status.into());
    ui.set_cpu_word(game.cpu_word.clone().into());
    ui.set_required_char(
        game.required
            .map(|c| c.to_string())
            .unwrap_or_else(|| "시작".into())
            .into(),
    );
}

fn start_new_game(ui: &MainWindow, game: &mut GameState, message: &str) {
    game.reset();
    if game.words.is_empty() {
        set_ui(ui, game, "txt.txt에 단어를 넣어주세요.", false);
        return;
    }

    let idx = rand::rng().random_range(0..game.words.len());
    let first = game.words[idx].clone();
    game.used.insert(first.clone());
    game.required = last_scalar(&first);
    game.cpu_word = first;
    set_ui(ui, game, message, game.required.is_some());
}

fn handle_submission(ui: &MainWindow, game: &mut GameState, raw: &str) {
    let Some(required) = game.required else {
        set_ui(ui, game, "먼저 게임을 시작하세요.", false);
        return;
    };

    let Some(word) = normalize_word(raw) else {
        set_ui(ui, game, "단어는 한 단어로 입력해주세요.", true);
        return;
    };

    if first_scalar(&word) != Some(required) {
        game.streak = 0;
        set_ui(ui, game, &format!("❌ '{}'로 시작하는 단어가 필요합니다.", required), true);
        return;
    }
    if !game.words.iter().any(|w| w == &word) {
        game.streak = 0;
        set_ui(ui, game, "❌ txt.txt에 없는 단어입니다.", true);
        return;
    }
    if game.used.contains(&word) {
        game.streak = 0;
        set_ui(ui, game, "❌ 이미 사용한 단어입니다.", true);
        return;
    }

    game.used.insert(word.clone());
    game.score += 10 + game.streak * 2;
    game.streak += 1;

    let Some(last) = last_scalar(&word) else {
        game.streak = 0;
        set_ui(ui, game, "❌ 올바르지 않은 단어입니다.", true);
        return;
    };

    match game.play_cpu(last) {
        Ok(cpu) => {
            game.score += 5;
            set_ui(ui, game, &format!("✅ 좋습니다! 컴퓨터: {}", cpu), true);
        }
        Err(_) => {
            game.score += 30;
            game.required = None;
            game.cpu_word.clear();
            set_ui(ui, game, "🏆 컴퓨터가 더 이상 이어갈 수 없습니다! 당신의 승리!", false);
        }
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).expect("failed to init Android backend");
    let ui = MainWindow::new().expect("failed to create UI");
    run(ui);
}

#[cfg(not(target_os = "android"))]
pub fn main() {
    let ui = MainWindow::new().expect("failed to create UI");
    run(ui);
}

fn run(ui: MainWindow) {
    let game = Rc::new(RefCell::new(GameState::new()));

    {
        let state = game.borrow();
        if state.words.is_empty() {
            set_ui(&ui, &state, "txt.txt에 단어를 넣어주세요.", false);
        } else {
            set_ui(&ui, &state, "시작 버튼을 눌러 게임을 시작하세요.", false);
        }
    }

    let weak: Weak<MainWindow> = ui.as_weak();
    let game_for_start = Rc::clone(&game);
    ui.on_start_game(move || {
        if let Some(ui) = weak.upgrade() {
            let mut state = game_for_start.borrow_mut();
            start_new_game(&ui, &mut state, "컴퓨터가 첫 단어를 냈습니다. 이어서 입력하세요!");
        }
    });

    let weak: Weak<MainWindow> = ui.as_weak();
    let game_for_new = Rc::clone(&game);
    ui.on_new_round(move || {
        if let Some(ui) = weak.upgrade() {
            let mut state = game_for_new.borrow_mut();
            start_new_game(&ui, &mut state, "새 게임 시작! 이어서 입력하세요.");
        }
    });

    let weak: Weak<MainWindow> = ui.as_weak();
    let game_for_submit = Rc::clone(&game);
    ui.on_submit_word(move |raw| {
        if let Some(ui) = weak.upgrade() {
            let mut state = game_for_submit.borrow_mut();
            handle_submission(&ui, &mut state, raw.as_str());
        }
    });

    ui.run().expect("failed to run UI");
}

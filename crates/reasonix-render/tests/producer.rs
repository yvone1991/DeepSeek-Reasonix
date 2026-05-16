use reasonix_render::producer::{build_setup_frame, build_trace_frame};
use reasonix_render::scene::{SceneNode, TextRun};
use reasonix_render::state::{EditMode, SceneCard, SceneState, SessionItem, SetupState, SlashMatch, ToolStatus};

fn root(state: SceneState) -> SceneNode {
    let frame = build_trace_frame(&state, 142, 38);
    frame.root
}

fn flatten(node: &SceneNode) -> String {
    let mut out = String::new();
    walk(node, &mut out);
    out
}

fn walk(node: &SceneNode, out: &mut String) {
    match node {
        SceneNode::Text { runs, .. } => {
            for run in runs {
                out.push_str(&run.text);
            }
            out.push('\n');
        }
        SceneNode::Box { children, .. } => {
            for child in children {
                walk(child, out);
            }
        }
    }
}

fn scroll_children(root: &SceneNode) -> &[SceneNode] {
    let outer = unbox(root).expect("outer box");
    let scroll = unbox(&outer[0]).expect("scroll box");
    scroll
}

fn dock_children(root: &SceneNode) -> &[SceneNode] {
    let outer = unbox(root).expect("outer box");
    let dock = unbox(&outer[1]).expect("dock box");
    dock
}

fn unbox(node: &SceneNode) -> Option<&[SceneNode]> {
    match node {
        SceneNode::Box { children, .. } => Some(children),
        _ => None,
    }
}

fn run_text(runs: &[TextRun]) -> String {
    runs.iter().map(|r| r.text.as_str()).collect()
}

#[test]
fn empty_state_renders_boot_block_with_reasonix_logo_and_tools_row() {
    let root = root(SceneState {
        model: Some("deepseek-chat".to_string()),
        cwd: Some("/work/reasonix".to_string()),
        ..Default::default()
    });
    let flat = flatten(&root);
    assert!(flat.contains("██████╗"), "missing logo");
    assert!(flat.contains("DeepSeek code agent"), "missing tagline");
    assert!(flat.contains("model"), "missing model field");
    assert!(flat.contains("deepseek-chat"), "missing model value");
    assert!(flat.contains("workdir"), "missing workdir field");
    assert!(flat.contains("/work/reasonix"), "missing cwd");
    assert!(flat.contains("tools"), "missing tools row");
}

#[test]
fn cards_state_replaces_boot_block_with_one_card_block_per_card() {
    let root = root(SceneState {
        cards: vec![
            SceneCard {
                kind: "user".to_string(),
                summary: "hello".to_string(),
                body: Some("hello".to_string()),
                ts: Some(0),
                ..Default::default()
            },
            SceneCard {
                kind: "streaming".to_string(),
                summary: "hi back".to_string(),
                body: Some("hi back".to_string()),
                ..Default::default()
            },
        ],
        card_count: 2,
        ..Default::default()
    });
    let scroll = scroll_children(&root);
    assert_eq!(scroll.len(), 2, "two cards expected");
    let flat = flatten(&root);
    assert!(flat.contains("YOU"), "missing YOU label");
    assert!(flat.contains("hello"), "missing user body");
    assert!(flat.contains("reasonix"), "missing reasonix label");
    assert!(flat.contains("hi back"), "missing streaming body");
    assert!(!flat.contains("██████╗"), "boot block should be hidden with cards");
}

#[test]
fn dock_has_composer_meta_status_when_no_overlay() {
    let root = root(SceneState::default());
    let dock = dock_children(&root);
    assert_eq!(dock.len(), 3, "composer + meta + status");
    let flat = flatten(&root);
    assert!(flat.contains("type to chat"));
    assert!(flat.contains("send"));
    assert!(flat.contains("newline"));
    assert!(flat.contains("reasonix"));
}

#[test]
fn composer_renders_text_with_cursor_block_at_offset() {
    let root = root(SceneState {
        composer_text: Some("hello".to_string()),
        composer_cursor: Some(2),
        ..Default::default()
    });
    let dock = dock_children(&root);
    let composer = unbox(&dock[0]).expect("composer box");
    let inner = match &composer[0] {
        SceneNode::Text { runs, .. } => run_text(runs),
        _ => panic!("expected text"),
    };
    assert!(inner.contains("he"), "before-cursor missing");
    assert!(inner.contains("▮"), "cursor block missing");
    assert!(inner.contains("llo"), "after-cursor missing");
}

#[test]
fn approval_replaces_composer_with_y_n_stub() {
    let root = root(SceneState {
        approval_kind: Some("shell".to_string()),
        approval_prompt: Some("rm -rf /tmp/x".to_string()),
        composer_text: Some("typing…".to_string()),
        ..Default::default()
    });
    let flat = flatten(&root);
    assert!(flat.contains("❓"));
    assert!(flat.contains("[shell]"));
    assert!(flat.contains("rm -rf /tmp/x"));
    assert!(flat.contains("[y/n]"));
    assert!(!flat.contains("typing…"));
}

#[test]
fn slash_overlay_renders_above_composer() {
    let root = root(SceneState {
        slash_matches: Some(vec![
            SlashMatch {
                cmd: "/help".to_string(),
                summary: "show help".to_string(),
                args_hint: None,
            },
            SlashMatch {
                cmd: "/model".to_string(),
                summary: "switch model".to_string(),
                args_hint: Some("<name>".to_string()),
            },
        ]),
        slash_selected_index: Some(1),
        ..Default::default()
    });
    let flat = flatten(&root);
    assert!(flat.contains("/help"));
    assert!(flat.contains("/model"));
    assert!(flat.contains("<name>"));
    assert!(flat.contains("switch model"));
    assert!(flat.contains("▸"));
}

#[test]
fn sessions_picker_block_appears_with_header_and_hint() {
    let root = root(SceneState {
        sessions: Some(vec![
            SessionItem {
                title: "feat-foo".to_string(),
                meta: Some("main · 12 turns".to_string()),
            },
            SessionItem {
                title: "spike-bar".to_string(),
                meta: Some("release/4.5".to_string()),
            },
        ]),
        sessions_focused_index: Some(0),
        ..Default::default()
    });
    let flat = flatten(&root);
    assert!(flat.contains("sessions"));
    assert!(flat.contains("2 saved"));
    assert!(flat.contains("feat-foo"));
    assert!(flat.contains("spike-bar"));
    assert!(flat.contains("navigate"));
    assert!(flat.contains("open"));
}

#[test]
fn status_bar_carries_model_and_wallet_and_cwd_segments() {
    let root = root(SceneState {
        model: Some("deepseek-chat".to_string()),
        wallet_balance: Some(184.2),
        wallet_currency: Some("CNY".to_string()),
        cwd: Some("/workspace/reasonix-core".to_string()),
        cards: vec![SceneCard {
            kind: "user".to_string(),
            summary: "x".to_string(),
            body: Some("x".to_string()),
            ..Default::default()
        }],
        card_count: 1,
        ..Default::default()
    });
    let dock = dock_children(&root);
    let status = &dock[dock.len() - 1];
    let flat: String = match status {
        SceneNode::Box { children, .. } => children
            .iter()
            .map(|c| match c {
                SceneNode::Text { runs, .. } => run_text(runs),
                _ => String::new(),
            })
            .collect::<Vec<_>>()
            .join(" | "),
        _ => panic!("expected status box"),
    };
    assert!(flat.contains("reasonix"));
    assert!(flat.contains("deepseek-chat"));
    assert!(flat.contains("¥184.20"));
    assert!(flat.contains("reasonix-core"));
}

#[test]
fn edit_mode_segment_uses_per_mode_color_glyph() {
    let root_yolo = root(SceneState {
        edit_mode: Some(EditMode::Yolo),
        ..Default::default()
    });
    let flat = flatten(&root_yolo);
    assert!(flat.contains("mode"));
    assert!(flat.contains("yolo"));
}

#[test]
fn tool_card_renders_in_rich_arrow_args_status_format() {
    let root = root(SceneState {
        cards: vec![SceneCard {
            kind: "tool".to_string(),
            summary: "Read".to_string(),
            args: Some("src/parser.ts".to_string()),
            status: Some(ToolStatus::Ok),
            elapsed: Some("120ms".to_string()),
            id: Some("#a4f1".to_string()),
            ..Default::default()
        }],
        card_count: 1,
        ..Default::default()
    });
    let flat = flatten(&root);
    assert!(flat.contains("▸"));
    assert!(flat.contains("Read"));
    assert!(flat.contains("(src/parser.ts)"));
    assert!(flat.contains("✓"));
    assert!(flat.contains("120ms"));
    assert!(flat.contains("#a4f1"));
}

#[test]
fn tool_card_with_err_status_uses_x_glyph() {
    let root = root(SceneState {
        cards: vec![SceneCard {
            kind: "tool".to_string(),
            summary: "Bash".to_string(),
            args: Some("false".to_string()),
            status: Some(ToolStatus::Err),
            ..Default::default()
        }],
        card_count: 1,
        ..Default::default()
    });
    assert!(flatten(&root).contains("✗"));
}

#[test]
fn setup_frame_renders_welcome_banner_and_masked_input() {
    let frame = build_setup_frame(
        &SetupState {
            buffer_length: 4,
            error: None,
        },
        80,
        24,
    );
    let flat = flatten(&frame.root);
    assert!(flat.contains("REASONIX"));
    assert!(flat.contains("welcome"));
    assert!(flat.contains("API key"));
    assert!(flat.contains("••••"));
    assert!(flat.contains("▮"));
    assert!(flat.contains("Ctrl+C"));
}

#[test]
fn setup_frame_with_error_renders_err_row() {
    let frame = build_setup_frame(
        &SetupState {
            buffer_length: 0,
            error: Some("key malformed".to_string()),
        },
        80,
        24,
    );
    let flat = flatten(&frame.root);
    assert!(flat.contains("✗"));
    assert!(flat.contains("key malformed"));
}

#[test]
fn long_card_body_capped_at_max_lines_with_blanks_skipped() {
    let body = vec!["a", "", "b", "c", "", "d", "e", "f", "g"].join("\n");
    let root = root(SceneState {
        cards: vec![SceneCard {
            kind: "user".to_string(),
            summary: "a".to_string(),
            body: Some(body),
            ..Default::default()
        }],
        card_count: 1,
        ..Default::default()
    });
    let flat = flatten(&root);
    assert!(!flat.contains(" g"), "body lines should cap before g");
}

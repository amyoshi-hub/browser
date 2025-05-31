fn extract_tokens(html: &str) -> Vec<String> {
    let mut in_tag = false;
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut space_count = 0;

    for c in html.chars() {
        match c {
            '<' => {
                in_tag = true;
                // 繧ｿ繧ｰ蜀・↓蜈･縺｣縺溘ｉ繝医・繧ｯ繝ｳ遒ｺ螳夲ｼ医ｂ縺励≠繧後・・・
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                space_count = 0;
            }
            '>' => {
                in_tag = false;
                space_count = 0;
            }
            _ if in_tag => {
                // 繧ｿ繧ｰ蜀・・辟｡隕・
                space_count = 0;
                // current_token縺ｯ縺昴・縺ｾ縺ｾ
            }
            ' ' => {
                space_count += 1;
                if space_count < 10 {
                    // 10蛟区悴貅縺ｮ騾｣邯壹せ繝壹・繧ｹ縺ｯ蜊倥↑繧九せ繝壹・繧ｹ謇ｱ縺・
                    // 繧ｹ繝壹・繧ｹ縺後ヨ繝ｼ繧ｯ繝ｳ縺ｮ荳驛ｨ縺ｪ繧芽ｿｽ蜉縲√◎縺・〒縺ｪ縺代ｌ縺ｰ辟｡隕・
                    if !current_token.is_empty() {
                        current_token.push(' ');
                    }
                } else if space_count == 10 {
                    // 10蛟狗岼縺ｮ繧ｹ繝壹・繧ｹ縺ｫ驕斐＠縺溘ｉ繝医・繧ｯ繝ｳ蛹ｺ蛻・ｊ
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    // 縺薙ｌ莉･髯阪・繧ｹ繝壹・繧ｹ辟｡隕厄ｼ磯｣邯壹せ繝壹・繧ｹ10蛟倶ｻ･荳翫・鬟帙・縺呻ｼ・
                }
                // 10蛟倶ｻ･荳翫↑繧我ｽ輔ｂ縺励↑縺・ｼ医せ繝壹・繧ｹ繧ｹ繧ｭ繝・・・・
            }
            _ => {
                // 繧ｹ繝壹・繧ｹ騾｣邯壹Μ繧ｻ繝・ヨ
                space_count = 0;
                current_token.push(c);
            }
        }
    }

    // 譛蠕後・繝医・繧ｯ繝ｳ繧定ｿｽ蜉
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn main() {
    let html = r#"<div>hello          world     this          is    a    test</div>"#;
    let tokens = extract_tokens(html);
    for (i, token) in tokens.iter().enumerate() {
        println!("token {}: '{}'", i, token);
    }
}

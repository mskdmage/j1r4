pub fn get_padded_string(text: &str, width: usize, pad: &str) -> String {
    let text_width = text.len();

    if text_width >= width {
        return text.to_string();
    }

    let to_pad = width - text_width;
    let is_even = to_pad % 2 == 0;

    if is_even {
        let side_padding = to_pad / 2;
        let left_pad = pad.repeat(side_padding);
        let right_pad = pad.repeat(side_padding);
        format!("{}{}{}", left_pad, text, right_pad)
    } else {
        let side_padding = to_pad / 2;
        let left_pad = pad.repeat(side_padding);
        let right_pad = pad.repeat(side_padding + 1);
        format!("{}{}{}", left_pad, text, right_pad)
    }
}

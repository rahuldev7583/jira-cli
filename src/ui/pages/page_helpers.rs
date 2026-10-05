use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    // println!("text:{}", text);

    // println!("width: {}", width);

    let text_len = text.len();

    //println!("text_len: {}", text_len);

    if text_len > 3 && text_len > width && width > 3 {
        //  println!("call truncate_ellipse");

        //  println!("width: {}", width);
        let str_leng = width - 3 as usize;

        let final_text = text.get(0..str_leng).unwrap();

        let result = final_text.clone().to_owned() + "...";

        // println!("result: {}", result);

        result
    } else if text_len > 0 && width <= 3 && width > 0 {
        //  println!("text:{}", text);

        if let 1 = width {
            ".".to_owned()
        } else if let 2 = width {
            "..".to_owned()
        } else {
            "...".to_owned()
        }
    } else {
        //   println!("cases");

        if text_len <= width && width > 0 {
            let mut rst = text.to_owned();

            for i in 1..=(width - text_len) {
                rst.push_str(" ");
            }
            rst
        } else {
            "".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}

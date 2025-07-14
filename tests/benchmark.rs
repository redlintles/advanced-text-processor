#[cfg(test)]
pub mod benchmark {
    use std::time::Instant;
    use atp_project::builder::atp_builder::AtpBuilder;

    #[test]
    fn process_sbs_all_tokens() {
        let runs = 100;

        let mut total = 0.0;

        let (mut processor, identifier) = AtpBuilder::new()
            .add_to_beginning("Banana")
            .add_to_end("pizza")
            .repeat(3)
            .delete_after(20 as usize)
            .delete_before(3 as usize)
            .delete_chunk(0 as usize, 3 as usize)
            .delete_first()
            .delete_last()
            .replace_all_with(r"a", "e")
            .replace_first_with("L", "coxinha")
            .replace_count_with("e", "carro", 3)
            .insert(0, "Coxinha Banana")
            .rotate_left(1 as usize)
            .rotate_right(2 as usize)
            .trim_both()
            .trim_left()
            .trim_right()
            .select(3, 7)
            .replace_count_with("a", "b", 3)
            .to_uppercase_all()
            .to_lowercase_all()
            .to_uppercase_single(3)
            .to_lowercase_single(2)
            .capitalize_first_word()
            .capitalize_last_word()
            .capitalize_range(1, 5)
            .split_select("b", 1)
            .capitalize_chunk(1, 3)
            .replace_last_with("b", "c")
            .replace_nth_with("b", "d", 3)
            .to_url_encoded()
            .to_url_decoded()
            .to_reverse()
            .split_characters()
            .to_html_escaped()
            .to_html_unescaped()
            .to_json_escaped()
            .to_json_unescaped()
            .insert(1, "banana")
            .to_uppercase_chunk(1, 3)
            .to_lowercase_chunk(0, 2)
            .build()
            .text_debug_processor();

        for _ in 0..runs {
            let start = Instant::now();

            let string_to_process = "Banana Laranja cheia de canja";

            let processed_string = processor
                .process_all_with_debug(&identifier, string_to_process)
                .unwrap();

            println!("{}", processed_string);
            let elapsed = start.elapsed().as_secs_f64();

            total += elapsed;
        }

        let avg = total / (runs as f64);

        println!("Média: {:.6} Segundos", avg);

        assert!(avg < 0.003, "Executou muito devagar");
    }
    #[test]
    fn process_all_tokens() {
        let runs = 100;

        let mut total = 0.0;

        let (mut processor, identifier) = AtpBuilder::new()
            .add_to_beginning("Banana")
            .add_to_end("pizza")
            .repeat(3)
            .delete_after(20 as usize)
            .delete_before(2 as usize)
            .insert(0, "Pizza")
            .delete_chunk(0 as usize, 3 as usize)
            .delete_first()
            .delete_last()
            .replace_all_with(r"a", "e")
            .replace_first_with("L", "coxinha")
            .replace_count_with("e", "carro", 3)
            .rotate_left(1 as usize)
            .rotate_right(2 as usize)
            .trim_both()
            .trim_left()
            .trim_right()
            .select(3, 7)
            .replace_count_with("a", "b", 3)
            .to_uppercase_all()
            .to_lowercase_all()
            .to_uppercase_single(3)
            .to_lowercase_single(2)
            .capitalize_first_word()
            .capitalize_last_word()
            .capitalize_range(1, 5)
            .split_select("b", 1)
            .capitalize_chunk(1, 3)
            .replace_last_with("b", "c")
            .replace_nth_with("b", "d", 3)
            .to_url_encoded()
            .to_url_decoded()
            .to_reverse()
            .split_characters()
            .to_html_escaped()
            .to_html_unescaped()
            .to_json_escaped()
            .to_json_unescaped()
            .insert(1, "banana")
            .build()
            .text_processor();

        for _ in 0..runs {
            let start = Instant::now();

            let string_to_process = "Banana Laranja cheia de canja";

            let processed_string = processor.process_all(&identifier, string_to_process).unwrap();

            println!("{}", processed_string);

            let elapsed = start.elapsed().as_secs_f64();

            total += elapsed;
        }

        let avg = total / (runs as f64);

        println!("Média: {:.6} Segundos", avg);

        assert!(avg < 0.003, "Executou muito devagar");
    }
}

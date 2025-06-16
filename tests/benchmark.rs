#[cfg(test)]
pub mod benchmark {
    use std::time::Instant;
    use atp_project::builder::atp_builder::AtpBuilder;

    #[test]
    fn process_sbs_all_tokens() {
        let runs = 100;

        let mut total = 0.0;

        let (processor, identifier) = AtpBuilder::new()
            .add_to_beginning("Banana")
            .add_to_end("pizza")
            .repeat(3)
            .delete_after(20 as usize)
            .delete_before(2 as usize)
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

        let (processor, identifier) = AtpBuilder::new()
            .add_to_beginning("Banana")
            .add_to_end("pizza")
            .repeat(3)
            .delete_after(20 as usize)
            .delete_before(2 as usize)
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

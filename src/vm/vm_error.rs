use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

use crate::vm::BUNDCore;

impl BUNDCore {
    pub fn display_message(&mut self, msg: String, inst: String) {
        println!("{}", self.generate_message(msg, inst))
    }
    pub fn generate_message(&mut self, msg: String, inst: String) -> String {
        let mut table = Table::new();
        let mut stacks_table = Table::new();
        let mut current_stack_table = Table::new();
        let mut call_stack_table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(60);
        stacks_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);
        current_stack_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);
        call_stack_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);
        table.add_row(vec![
                Cell::new("Message").fg(Color::Red),
                Cell::new(msg.as_str()).fg(Color::Yellow),
        ]);
        table.add_row(vec![
                Cell::new("Instruction").fg(Color::Blue),
                Cell::new(inst.as_str()).fg(Color::White),
        ]);
        table.add_row(vec![
                Cell::new("Number of stacks").fg(Color::Blue),
                Cell::new(format!("{}", self.stack.len())).fg(Color::White),
        ]);
        table.add_row(vec![
                Cell::new("Size of call stack").fg(Color::Blue),
                Cell::new(format!("{}", self.call_stack.len())).fg(Color::White),
        ]);
        table.add_row(vec![
                Cell::new("Number of elements in current stack").fg(Color::Blue),
                Cell::new(format!("{}", self.stack.stack_len())).fg(Color::White),
        ]);
        stacks_table.add_row(
            vec![Cell::new("List of stacks").fg(Color::Blue),]
        );
        for i in self.stack.stack.stack.iter() {
            stacks_table.add_row(
                vec![Cell::new(format!("{}", i.stack_id())).fg(Color::White),]
            );
        }
        current_stack_table.add_row(
            vec![Cell::new("Elements in current stack").fg(Color::Blue),]
        );
        let cs = self.stack.current().unwrap();
        for e in cs.stack.iter() {
            current_stack_table.add_row(
                vec![Cell::new(format!("{:?}", e)).fg(Color::White),]
            );
        }
        call_stack_table.add_row(
            vec![Cell::new("Elements in call stack").fg(Color::Blue),]
        );
        for e in self.call_stack.iter() {
            call_stack_table.add_row(
                vec![Cell::new(format!("{:?}", e)).fg(Color::White),]
            );
        }
        format!("{}\n\n{}\n\n{}\n\n{}", table, stacks_table, current_stack_table, call_stack_table)
    }
}

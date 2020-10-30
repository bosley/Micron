use std::collections::VecDeque;
use micron_ast::Statement;

/// Instruction queue for function /method calling
#[derive(Debug, Clone)]
pub(crate) struct InstructionQueue {

    // Think of this as a growing list of function statements. If a function is called,
    // its instructions are put into a VecDeque and that VecDeque is pushed to the Vec.
    // When a return from a function happens, its instruction VecDeque will be removed
    instructions: Vec<VecDeque<Box<Statement>>>
}

impl InstructionQueue {

    fn new() -> Self {
        Self {
            instructions: Vec::new()
        }
    }

    /// If a return is hit, and there were instructions in the queue we need to do something
    pub(crate) fn mark_return_statement(&mut self) {

        // If this was called in error, we just bail
        if self.instructions.len() == 0 {
            return;
        }

        // Remove the current operating function's instructions from the list
        self.instructions.pop();
    }

    /// Get the next instruction if one exists
    pub(crate) fn get_next_ins(&mut self) -> Option<Box<Statement>> {

        // If there are no instructions at all, bail
        if self.instructions.len() == 0 {
            return None;
        }

        // Get the current list to pull from
        let n = self.instructions.len();

        // If The list we are operating on is empty, remove it
        // and bail
        if self.instructions[n-1].len() == 0 {

            self.instructions.pop();
            return None;
        }

        // Attempt to get the instruction from the list we are working on
        let result = match self.instructions[n-1].front() {

            Some(value) => {

                // Clone it out so we can keep it forever <3
                value.clone()
            }

            None => {
                return None
            }
        };

        // Pop the instruction. We won't need it anymore
        self.instructions[n-1].pop_front();

        // If the list is empty now, remove it from the outer list
        if self.instructions[n-1].len() == 0 {
            
            self.instructions.pop();
        }

        // Return the item
        return Some(result);
    }
}
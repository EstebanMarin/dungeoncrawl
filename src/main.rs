use bracket_lib::prelude::*;

struct State {
    score: i32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu, 
	    score: 0,
        }
    }
    
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Dungeon");
        ctx.print_centered(8, "(p) Play game");
        ctx.print_centered(9, "(q) quit game");
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, "(p) Play game");
        ctx.print_centered(9, "(q) quit game");
    }

}

enum GameMode {
    Menu,
    Playing,
    End,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}

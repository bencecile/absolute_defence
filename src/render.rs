use web_sys::{
    CanvasRenderingContext2d,
};

pub struct DrawingKit<'a> {
    context: &'a CanvasRenderingContext2d,
    width: f64,
    height: f64,
}
impl <'a> DrawingKit<'a> {
    pub fn new(context: &CanvasRenderingContex2d, width: f64, height: f64) -> DrawingKit<'a> {
        DrawingKit { context, width, height }
    }
}
impl <'a> DrawingKit<'a> {
    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn draw_world(&self, world: &World) {
        // TODO Figure out the world's viewport with the same width ratio as the context
        // TODO Draw the player directly in the center of the screen

        // TODO Print out the player's health
        // TODO Print out the player's progress (y position)
        context.save();
        context.set_fill_style(&("black".into()));

        // Wall
        context.stroke_rect(75.0, 140.0, 150.0, 110.0);

        // Door
        context.fill_rect(130.0, 190.0, 40.0, 60.0);

        // Roof
        context.begin_path();
        context.move_to(50.0, 140.0);
        context.line_to(150.0, 60.0);
        context.line_to(250.0, 140.0);
        context.close_path();
        context.stroke();

        context.restore();
    }
}

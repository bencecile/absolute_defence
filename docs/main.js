import init, {
    GameState,
    first_time_setup,
    init_context,
} from "./assets/absolute_defence.js";

async function run() {
    await init();

    first_time_setup();

    const canvas_holder = {
        canvas: document.querySelector("#glCanvas"),
        has_changed: false,
        resize_to_window() {
            this.canvas.width = window.innerWidth;
            this.canvas.height = window.innerHeight;
            this.has_changed = true;
        },
        create_context() {
            this.has_changed = false;
            const context = this.canvas.getContext("2d");
            if (!context) {
                alert("Failed to get a 2D context");
                return null;
            }
            return { context, width: this.canvas.width, height: this.canvas.height };
        }
    };
    window.onresize = () => canvas_holder.resize_to_window;
    canvas_holder.resize_to_window();

    let context = null;
    let width = null;
    let height = null;
    let last_update_time = window.performance.now();
    let game_state = GameState.starting_state();
    const game_loop = (current_time) => {
        if (canvas_holder.has_changed) {
            let context_object = canvas_holder.create_context();
            if (!context_object.context) {
                console.error("Failed to get a context from the canvas");
                return;
            }
            context = context_object.context;
            init_context(context);
            width = context_object.width;
            height = context_object.height;
        }

        let time_delta = current_time - last_update_time;
        last_update_time = current_time;
        game_state.update(time_delta);
        game_state.render(context, width, height);
        window.requestAnimationFrame(game_loop);
    }
    window.requestAnimationFrame(game_loop);
}

run();

use piston_window::*;
use rand::*;

// WINDOW params
const WIDTH: f64 = 1280.0;
const HEIGHT: f64 = 720.0;

// COLORS
const BLUE: [f32; 4] = [19.0/255.0, 124.0/255.0, 229.0/255.0, 1.0];
const ORANGE: [f32; 4] = [237.0/255.0, 173.0/255.0, 108.0/255.0, 1.0];

// Bubble struct 
struct Bubble {
    speed: f64,
    x: f64,
    y: f64,
    r: f64,
}
// implement new function for bubble struct 
impl Bubble {
    pub fn new(num: Option<f64>) -> Bubble {
        let b = Bubble {
            // Start at a random x position
            x: (random::<f64>() * WIDTH),
            // Start with a random radius
            r: (random::<f64>() * 50.0) + 10.0,

            speed: random::<f64>(),

            // Start some on the bottom, some at random heights
            y: match num {
                Some(x) => x,
                None => HEIGHT,
            },
        };
        // return bubble 
        b
    }
}

fn main() {
    // Create window
    let mut window: PistonWindow = WindowSettings::new("Lava Lamp", (WIDTH,HEIGHT))
        .exit_on_esc(true).build().unwrap();

    // Create event handler 
    let mut events = Events::new(EventSettings::new());

    // Create a vector of bubbles
    let mut bubbles: Vec<Bubble> = Vec::new();
    for _ in 0..20 {
       bubbles.push(Bubble::new(None));
       bubbles.push(Bubble::new(Some(random::<f64>() * HEIGHT)));
    }

    // Get events from the window
    while let Some(event) = events.next(&mut window) {
        // drawing on screen
        if let Some(_) = event.render_args()
        {
            window.draw_2d(&event, |context, graphics, _device| {
                // Draw the background color
                clear(BLUE, graphics);
                //ellipse(ORANGE, // RED
                //    [bubble.x, bubble.y, bubble.r * 2.0, bubble.r * 2.0], // [x,y, r*2, r*2]
                //    context.transform, graphics);

                // Get immutable reference to bubbles Vec
                for b in &bubbles {
                    ellipse(ORANGE,
                        [b.x, b.y, b.r*2.0, b.r*2.0],
                        context.transform, graphics);
                }
            });
        }
        // updating motion variables
        if let Some(u) = event.update_args()
        {
            // get mutable reference to bubbles Vec
            for b in &mut bubbles {
                // Update y position of bubble
                b.y -= b.speed + u.dt;
                if b.y <= 0.0 { b.y = HEIGHT; }
            }
        }
    }
}

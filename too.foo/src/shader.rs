use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;

pub struct BackgroundEffect {
    time: f64,
    width: f64,
    height: f64,
}

impl BackgroundEffect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            time: 0.0,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt * 0.5; // Slow movement
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // Create a subtle gradient background
        // 5-10% opacity as requested
        
        let t = self.time;
        
        // Base dark background
        ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
        ctx.fill_rect(0.0, 0.0, self.width, self.height);
        
        // Wave 1: Cyan/Blue
        let x1 = (t.sin() * 0.5 + 0.5) * self.width;
        let y1 = (t.cos() * 0.3 + 0.5) * self.height;
        let r1 = self.width.min(self.height) * 0.8;
        
        let grad1 = ctx.create_radial_gradient(x1, y1, 0.0, x1, y1, r1).unwrap();
        grad1.add_color_stop(0.0, "rgba(0, 255, 200, 0.05)").unwrap(); // 5% opacity
        grad1.add_color_stop(1.0, "rgba(0, 255, 200, 0.0)").unwrap();
        
        ctx.set_fill_style(&grad1);
        ctx.fill_rect(0.0, 0.0, self.width, self.height);
        
        // Wave 2: Purple/Magenta
        let t2 = t * 0.7 + 2.0;
        let x2 = (t2.cos() * 0.5 + 0.5) * self.width;
        let y2 = (t2.sin() * 0.4 + 0.5) * self.height;
        let r2 = self.width.min(self.height) * 0.9;
        
        let grad2 = ctx.create_radial_gradient(x2, y2, 0.0, x2, y2, r2).unwrap();
        grad2.add_color_stop(0.0, "rgba(150, 0, 255, 0.04)").unwrap(); // 4% opacity
        grad2.add_color_stop(1.0, "rgba(150, 0, 255, 0.0)").unwrap();
        
        ctx.set_fill_style(&grad2);
        ctx.fill_rect(0.0, 0.0, self.width, self.height);
        
        // Subtle scanline effect
        ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.1)"));
        for i in (0..self.height as i32).step_by(4) {
            ctx.fill_rect(0.0, i as f64, self.width, 1.0);
        }
    }
}
mod window;

fn main() {
    _ = window::create_window(800, 600, 50, 50, 0x00FF0000);
    _ = window::create_window(100, 100, 190, 108, 0x00FFFFFF);
}

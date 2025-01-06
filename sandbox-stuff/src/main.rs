use shapes::j2rect::Rect;

mod shapes;

fn main() {
    let rect = Rect::default();

    println!("{}", rect.to_string());
}

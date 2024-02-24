
fn keyboard(mut windows: Query<&mut Window>) {
    let e_mouse = enigo::Enigo::new();
    let (x, y) = e_mouse.mouse_location();
    windows.get_single_mut().unwrap().position = WindowPosition::At(IVec2::new(x + 10, y));
}
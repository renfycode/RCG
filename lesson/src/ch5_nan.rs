

pub fn check() {
    let x: f32 = 1.0 / 0.0;
    assert!(x.is_infinite())
}

pub fn is_nan_is_not_equal() {
    // Пример демонстрирует
    // Что NaN не равно самому себе
    check();
    let x = (-42.0_f32).sqrt();
    assert_ne!(x, x);
}

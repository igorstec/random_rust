fn main() {
    println!("Hello, world!");
    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    println!("{:?}", v1);
    how_box_works(v1[0]);
    how_rc_arc_works(v1[2]);
    how_borrowrefference_works(&v1[3]);
    let el = v1[3];
    v1.push(5);
    println!("{}", v1[3]);
    println!("{}", el);
    how_demo_rust_features_work();
}

// unique pointer in c++
fn how_box_works(x: i32) {
    let b = Box::new(x);
    println!("Box value: {}", b);
}

// shared pointer in c++
fn how_rc_arc_works(x: i32) {
    let rc = std::rc::Rc::new(x);
    println!("Rc value: {}", rc);
}

// reference in c++
fn how_borrowrefference_works(x: &i32) {
    println!("Reference value: {}", x);
}

/// enumy, pattern matching, Result/Option, closure,
/// iterators, ownership i borrowing.
pub fn how_demo_rust_features_work() {
    #[derive(Debug)]
    enum Shape {
        Circle { r: f64 },
        Rectangle { w: f64, h: f64 },
        Triangle { a: f64, b: f64, c: f64 },
    }

    // Closure zwracająca Result: pole figury albo błąd dla niepoprawnego trójkąta
    let area = |s: &Shape| -> Result<f64, String> {
        match *s {
            Shape::Circle { r } => Ok(std::f64::consts::PI * r * r),
            Shape::Rectangle { w, h } => Ok(w * h),
            Shape::Triangle { a, b, c } => {
                if a + b <= c || a + c <= b || b + c <= a {
                    return Err(format!("niepoprawny trójkąt: {a}, {b}, {c}"));
                }
                let p = (a + b + c) / 2.0;
                Ok((p * (p - a) * (p - b) * (p - c)).sqrt()) // wzór Herona
            }
        }
    };

    let shapes = vec![
        Shape::Circle { r: 1.5 },
        Shape::Rectangle { w: 3.0, h: 4.0 },
        Shape::Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        },
        Shape::Triangle {
            a: 1.0,
            b: 2.0,
            c: 10.0,
        },
    ];

    // Zuzia Bukowy:
    // Borrowing: iterujemy po referencjach, `shapes` nadal należy do nas
    for s in &shapes {
        match area(s) {
            Ok(a) => println!("{s:?} -> pole = {a:.2}"),
            Err(e) => println!("{s:?} -> błąd: {e}"),
        }
    }

    // Iteratory: filter_map odrzuca błędy, sum sumuje poprawne pola
    let total: f64 = shapes.iter().filter_map(|s| area(s).ok()).sum();
    println!("Suma poprawnych pól: {total:.2}");

    // Option: największa figura (f64 nie implementuje Ord, więc total_cmp)
    let largest = shapes
        .iter()
        .filter_map(|s| area(s).ok().map(|a| (s, a)))
        .max_by(|x, y| x.1.total_cmp(&y.1));
    if let Some((s, a)) = largest {
        println!("Największa figura: {s:?} ({a:.2})");
    }

    // Ownership: into_iter przenosi własność, po tej linii `shapes` jest niedostępne
    let circles: Vec<Shape> = shapes
        .into_iter()
        .filter(|s| matches!(s, Shape::Circle { .. }))
        .collect();
    println!("Liczba kół: {}", circles.len());
}

use std::collections::HashMap;

pub type F1 = fn(f64) -> f64;
pub type F2 = fn(f64, f64) -> f64;

pub struct Ctx {
    pub vs: HashMap<String, f64>,
    pub f1: HashMap<String, F1>,
    pub f2: HashMap<String, F2>,
}

impl Ctx {
    pub fn new() -> Self {
        let mut vs = HashMap::new();
        vs.insert("pi".into(), std::f64::consts::PI);
        vs.insert("e".into(), std::f64::consts::E);

        let mut f1: HashMap<String, F1> = HashMap::new();
        f1.insert("sqrt".into(), f64::sqrt);
        f1.insert("exp".into(), f64::exp);
        f1.insert("abs".into(), f64::abs);
        f1.insert("sin".into(), f64::sin);
        f1.insert("cos".into(), f64::cos);
        f1.insert("tan".into(), f64::tan);
        f1.insert("cot".into(), |v| 1.0 / v.tan());
        f1.insert("sec".into(), |v| 1.0 / v.cos());
        f1.insert("csc".into(), |v| 1.0 / v.sin());
        f1.insert("asin".into(), f64::asin);
        f1.insert("acos".into(), f64::acos);
        f1.insert("atan".into(), f64::atan);
        f1.insert("acot".into(), |v| std::f64::consts::FRAC_PI_2 - v.atan());
        f1.insert("asec".into(), |v| (1.0 / v).acos());
        f1.insert("acsc".into(), |v| (1.0 / v).asin());
        f1.insert("arcsin".into(), f64::asin);
        f1.insert("arccos".into(), f64::acos);
        f1.insert("arctan".into(), f64::atan);
        f1.insert("arccot".into(), |v| std::f64::consts::FRAC_PI_2 - v.atan());
        f1.insert("arcsec".into(), |v| (1.0 / v).acos());
        f1.insert("arccsc".into(), |v| (1.0 / v).asin());
        f1.insert("sinh".into(), f64::sinh);
        f1.insert("cosh".into(), f64::cosh);
        f1.insert("tanh".into(), f64::tanh);
        f1.insert("coth".into(), |v| 1.0 / v.tanh());
        f1.insert("sech".into(), |v| 1.0 / v.cosh());
        f1.insert("csch".into(), |v| 1.0 / v.sinh());
        f1.insert("asinh".into(), f64::asinh);
        f1.insert("acosh".into(), f64::acosh);
        f1.insert("atanh".into(), f64::atanh);
        f1.insert("acoth".into(), |v| (1.0 / v).atanh());
        f1.insert("asech".into(), |v| (1.0 / v).acosh());
        f1.insert("acsch".into(), |v| (1.0 / v).asinh());
        f1.insert("arsinh".into(), f64::asinh);
        f1.insert("arcosh".into(), f64::acosh);
        f1.insert("artanh".into(), f64::atanh);
        f1.insert("arcoth".into(), |v| (1.0 / v).atanh());
        f1.insert("arsech".into(), |v| (1.0 / v).acosh());
        f1.insert("arcsch".into(), |v| (1.0 / v).asinh());
        f1.insert("ln".into(), f64::ln);
        f1.insert("log".into(), f64::log10);
        f1.insert("floor".into(), f64::floor);
        f1.insert("ceil".into(), f64::ceil);
        f1.insert("round".into(), f64::round);

        let mut f2: HashMap<String, F2> = HashMap::new();
        f2.insert("pow".into(), |b, e| b.powf(e));
        f2.insert("root".into(), |x, n| x.powf(1.0 / n));
        f2.insert("logb".into(), |x, b| x.log(b));

        Ctx { vs, f1, f2 }
    }
}

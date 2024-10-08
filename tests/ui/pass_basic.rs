#[struct_cache_field::impl_cached_method]
impl Hoge {
    pub fn two_times_x(&self) -> u64 {
        2 * self.x
    }

    fn x_plus_1(&mut self) -> u64 {
        self.x = self.x + 1;
        self.x
    }
}

#[struct_cache_field::add_cache_field]
struct Hoge {
    x: u64,
}

impl Hoge {
    #[allow(unused)]
    pub fn three_times_x(&self) -> u64 {
        3 * self.x
    }
}

fn main() {
    let mut hoge = Hoge {
        x: 1,
        __cache_fields__: Default::default(),
    };

    assert_eq!(hoge.two_times_x(), &2);
    assert_eq!(hoge.two_times_x(), &2);
    hoge.x = 2;
    assert_eq!(hoge.two_times_x(), &2);

    assert_eq!(hoge.x_plus_1(), &3);
    assert_eq!(hoge.x_plus_1(), &3);
    hoge.x = 4;
    assert_eq!(hoge.x_plus_1(), &3);
}

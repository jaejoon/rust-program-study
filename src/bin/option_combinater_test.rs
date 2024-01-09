

fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    let a_map = a.map(|num| num+1); // use a for map so a has no data
    dbg!(a_map);
    let a_filters = a.filter(|num| num == &1); // 
    dbg!(a_filters);
    let a_or_else = a.or_else(|| Some(5)); // No input data for closure. if a has no data a_or_else become to Some(5)
    dbg!(a_or_else);
    let a_unwrap_or_else = a.unwrap_or_else(|| 0); // if a has a data, a_unwrap_or_else's value is a's data. but if not, a_unwrap_or_else will be set into initial value 0
    dbg!(a_unwrap_or_else);

    let b: Option<i32> = None;
    dbg!(a);
    let a_is_some = b.is_some();
    dbg!(a_is_some);
    let a_is_none = b.is_none();
    dbg!(a_is_none);
    let a_map = b.map(|num| num+1); // use a for map so a has no data
    dbg!(a_map);
    let a_filters = b.filter(|num| num == &1); // 
    dbg!(a_filters);
    let a_or_else = b.or_else(|| Some(5)); // No input data for closure. if a has no data a_or_else become to Some(5)
    dbg!(a_or_else);
    let a_unwrap_or_else = b.unwrap_or_else(|| 0); // if a has a data, a_unwrap_or_else's value is a's data. but if not, a_unwrap_or_else will be set into initial value 0
    dbg!(a_unwrap_or_else);
}
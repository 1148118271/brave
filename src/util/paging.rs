
pub fn paging<T: Clone>(page_num: isize, limit_num: isize, data: &Vec<T>) -> Vec<T> {
    if page_num < 0 || limit_num <= 0 {
        return vec![]
    }
    let start = (page_num - 1) * limit_num;
    let len = data.len() as isize;
    let num = start + limit_num;
    let end = if len > num { num } else { len };
    let d = if start == 0 { &data[..end as usize] }
                    else { &data[start as usize..end as usize] };
    d.to_vec()
}
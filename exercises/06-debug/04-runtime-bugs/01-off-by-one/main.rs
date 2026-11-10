// Pagination helpers for a blog engine.

/// Number of pages needed for `total` items, `per_page` items per page.
fn total_pages(total: usize, per_page: usize) -> usize {
    total / per_page
}

/// `(start, end)` slice bounds of the items shown on 0-based `page`.
/// Must satisfy `start <= end <= total`.
fn page_bounds(page: usize, per_page: usize, total: usize) -> (usize, usize) {
    let start = page * per_page;
    let end = start + per_page;
    (start, end)
}

/// 0-based index of the last page (0 when there are no pages).
fn last_page_index(total: usize, per_page: usize) -> usize {
    total_pages(total, per_page)
}

fn main() {
    let items = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];
    let per_page = 5;

    let pages = total_pages(items.len(), per_page);
    println!(
        "{pages} page(s) for {} item(s), last page index {}",
        items.len(),
        last_page_index(items.len(), per_page)
    );

    for page in 0..pages {
        let (start, end) = page_bounds(page, per_page, items.len());
        println!("page {page}: {:?}", &items[start..end]);
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_pages_rounds_up() {
        assert_eq!(total_pages(1, 10), 1);
        assert_eq!(total_pages(11, 10), 2);
        assert_eq!(total_pages(95, 10), 10);
    }

    #[test]
    fn test_total_pages_edge_cases() {
        assert_eq!(total_pages(0, 10), 0);
        assert_eq!(total_pages(10, 10), 1);
        assert_eq!(total_pages(20, 10), 2);
    }

    #[test]
    fn test_page_bounds_within_total() {
        assert_eq!(page_bounds(0, 10, 25), (0, 10));
        assert_eq!(page_bounds(1, 10, 25), (10, 20));
        assert_eq!(page_bounds(2, 10, 25), (20, 25));
    }

    #[test]
    fn test_page_bounds_beyond_last() {
        // Pages past the end collapse to an empty range at the very end.
        assert_eq!(page_bounds(7, 10, 25), (25, 25));
        assert_eq!(page_bounds(0, 10, 0), (0, 0));
    }

    #[test]
    fn test_last_page_index() {
        assert_eq!(last_page_index(1, 10), 0);
        assert_eq!(last_page_index(25, 10), 2);
        assert_eq!(last_page_index(30, 10), 2);
        assert_eq!(last_page_index(0, 10), 0);
    }
}

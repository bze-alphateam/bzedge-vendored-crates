use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;

#[pin_project(PinnedDrop)]
struct S {
    #[pin]
    f: u8,
}

#[pinned_drop]
impl PinnedDrop for S {
    fn drop(self: Pin<&mut Self>) {
        self.project().f.get_unchecked_mut(); //~ ERROR call to unsafe function is unsafe and requires unsafe function or block [E0133]
    }
}

fn main() {}
